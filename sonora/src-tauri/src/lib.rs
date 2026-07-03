use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::thread;
use std::sync::mpsc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use symphonia::core::audio::{AudioBuffer, AudioBufferRef, SignalSpec};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::{FormatOptions, FormatReader};
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::core::sample::Sample;
use symphonia::core::units::Time;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::StreamConfig;

// Audio state
struct AudioState {
    is_playing: Arc<AtomicBool>,
    current_index: Option<usize>,
    shuffle: bool,
    shuffled_indices: Vec<usize>,
    volume: f32,
    current_time: Arc<Mutex<f64>>,
    duration: Arc<Mutex<f64>>,
    stream: Option<cpal::Stream>,
    tx: Option<mpsc::Sender<bool>>,
}

type SharedAudioState = Mutex<AudioState>;

#[tauri::command]
fn select_folder() -> Result<String, String> {
    let dialog = rfd::FileDialog::new()
        .set_title("Select Music Folder")
        .pick_folder();

    match dialog {
        Some(path) => Ok(path.to_string_lossy().to_string()),
        None => Err("No folder selected".to_string()),
    }
}

#[tauri::command]
fn get_music_files(folder_path: String) -> Result<Vec<String>, String> {
    let path = PathBuf::from(&folder_path);
    let mut music_files = Vec::new();
    let audio_extensions = ["mp3", "wav", "flac", "m4a", "aac", "ogg", "wma"];

    let entries = std::fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    for entry in entries.flatten() {
        let file_path = entry.path();
        let extension = file_path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.to_lowercase());

        if let Some(ext) = extension {
            if audio_extensions.contains(&ext.as_str()) {
                if let Some(name) = entry.file_name().to_str() {
                    music_files.push(name.to_string());
                }
            }
        }
    }

    Ok(music_files)
}

#[tauri::command]
fn play_music(
    file_path: String,
    index: usize,
    state: tauri::State<SharedAudioState>,
) -> Result<String, String> {
    let path = PathBuf::from(&file_path);
    if !path.exists() {
        return Err(format!("File does not exist: {}", file_path));
    }

    // Stop any existing playback
    let mut audio_state = state.lock().unwrap();
    if let Some(tx) = audio_state.tx.take() {
        let _ = tx.send(true);
    }
    audio_state.stream = None;
    audio_state.is_playing.store(false, Ordering::SeqCst);

    // Create new state
    let is_playing = Arc::new(AtomicBool::new(true));
    let current_time = Arc::new(Mutex::new(0.0));
    let duration = Arc::new(Mutex::new(0.0));
    let (tx, rx) = mpsc::channel();

    // Spawn playback thread
    let is_playing_clone = is_playing.clone();
    let current_time_clone = current_time.clone();
    let duration_clone = duration.clone();
    let volume = audio_state.volume;
    thread::spawn(move || {
        // Open file
        let Ok(source) = std::fs::File::open(&path) else {
            eprintln!("Failed to open file");
            is_playing_clone.store(false, Ordering::SeqCst);
            return;
        };

        let mss = MediaSourceStream::new(Box::new(source), Default::default());
        let mut hint = Hint::new();
        if let Some(ext) = path.extension() {
            if let Some(ext_str) = ext.to_str() {
                hint.with_extension(ext_str);
            }
        }

        let meta_opts = MetadataOptions::default();
        let fmt_opts = FormatOptions::default();

        let Ok(probed) = symphonia::default::get_probe().format(
            &hint,
            mss,
            &fmt_opts,
            &meta_opts,
        ) else {
            eprintln!("Failed to probe file");
            is_playing_clone.store(false, Ordering::SeqCst);
            return;
        };

        let mut reader = probed.format;
        let track = reader
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .unwrap();
        let track_id = track.id;

        let dec_opts = DecoderOptions::default();
        let Ok(mut decoder) = symphonia::default::get_codecs().make(&track.codec_params, &dec_opts) else {
            eprintln!("Failed to create decoder");
            is_playing_clone.store(false, Ordering::SeqCst);
            return;
        };

        // Get duration
        let track_duration = track
            .codec_params
            .time_base
            .and_then(|tb| track.codec_params.n_frames.map(|frames| tb.calc_time(frames).seconds as f64 + tb.calc_time(frames).frac as f64))
            .unwrap_or(0.0);
        *duration_clone.lock().unwrap() = track_duration;

        // Get sample format and channels
        let sample_rate = track.codec_params.sample_rate.unwrap_or(44100);
        let channels = track.codec_params.channels.unwrap().count();
        let config = StreamConfig {
            channels: channels.into(),
            sample_rate: cpal::SampleRate(sample_rate),
            buffer_size: cpal::BufferSize::Default,
        };

        // Get output device
        let host = cpal::default_host();
        let Some(device) = host.default_output_device() else {
            eprintln!("No output device found");
            is_playing_clone.store(false, Ordering::SeqCst);
            return;
        };

        // Create ring buffer for audio data
        let (mut prod, mut cons) = ringbuf::HeapRb::<f32>::new(sample_rate as usize * 4).split();

        // Create audio stream
        let stream_result = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Fill data from ring buffer
                let mut idx = 0;
                while idx < data.len() && !cons.is_empty() {
                    if let Some(sample) = cons.pop() {
                        data[idx] = sample * volume;
                        idx += 1;
                    }
                }
                // Fill remaining with silence
                for i in idx..data.len() {
                    data[i] = 0.0;
                }
            },
            move |err| {
                eprintln!("Stream error: {}", err);
            },
            None,
        );

        let Ok(stream) = stream_result else {
            eprintln!("Failed to build stream");
            is_playing_clone.store(false, Ordering::SeqCst);
            return;
        };

        // Start stream
        if stream.play().is_err() {
            eprintln!("Failed to start stream");
            is_playing_clone.store(false, Ordering::SeqCst);
            return;
        }

        // Keep stream alive in this thread (so we need to store it somewhere)
        // But since we can't move stream to the state (it's not Send), we'll keep it here
        // and check for stop signal
        let mut current_sample = 0u64;
        let sample_rate_f = sample_rate as f64;

        loop {
            // Check if we need to stop
            if rx.try_recv().is_ok() {
                break;
            }
            if !is_playing_clone.load(Ordering::SeqCst) {
                continue;
            }

            // Read next packet
            let Ok(packet) = reader.next_packet() else {
                break;
            };
            if packet.track_id() != track_id {
                continue;
            }

            // Decode packet
            let Ok(decoded) = decoder.decode(&packet) else {
                continue;
            };

            // Convert to f32 and write to ring buffer
            let spec = decoded.spec();
            let duration = decoded.capacity() as u64;
            let num_channels = spec.channels.count();
            let num_frames = duration as usize;
            let mut samples = vec![0.0f32; num_frames * num_channels];

            // Iterate over each channel plane and copy samples
            for (ch, channel) in decoded.planes().planes().iter().enumerate() {
                for (frame, &sample) in channel.iter().enumerate() {
                    let idx = frame * num_channels + ch;
                    samples[idx] = sample;
                }
            }

            // Write to ring buffer
            let mut written = 0;
            while written < samples.len() && !prod.is_full() {
                if prod.push(samples[written]).is_ok() {
                    written += 1;
                }
            }

            // Update current time
            current_sample += duration;
            *current_time_clone.lock().unwrap() = current_sample as f64 / sample_rate_f;
        }
    });

    // Update state
    audio_state.is_playing = is_playing;
    audio_state.current_index = Some(index);
    audio_state.tx = Some(tx);
    audio_state.current_time = current_time;
    audio_state.duration = duration;

    Ok(format!("Playing: {}", file_path))
}

#[tauri::command]
fn pause_music(state: tauri::State<SharedAudioState>) -> Result<String, String> {
    let audio_state = state.lock().unwrap();
    audio_state.is_playing.store(false, Ordering::SeqCst);
    Ok("Paused".to_string())
}

#[tauri::command]
fn resume_music(state: tauri::State<SharedAudioState>) -> Result<String, String> {
    let audio_state = state.lock().unwrap();
    audio_state.is_playing.store(true, Ordering::SeqCst);
    Ok("Resumed".to_string())
}

#[tauri::command]
fn skip_next(
    music_files: Vec<String>,
    state: tauri::State<SharedAudioState>,
) -> Result<usize, String> {
    let mut audio_state = state.lock().unwrap();
    if let Some(current_index) = audio_state.current_index {
        if music_files.is_empty() {
            return Err("No music files available".to_string());
        }

        let next_index = if audio_state.shuffle {
            if audio_state.shuffled_indices.is_empty() {
                let mut indices: Vec<usize> = (0..music_files.len()).collect();
                use rand::seq::SliceRandom;
                indices.shuffle(&mut rand::thread_rng());
                audio_state.shuffled_indices = indices;
            }

            let current_shuffled_pos = audio_state
                .shuffled_indices
                .iter()
                .position(|&i| i == current_index)
                .unwrap_or(0);
            let next_shuffled_pos = if current_shuffled_pos + 1 < audio_state.shuffled_indices.len() {
                current_shuffled_pos + 1
            } else {
                0
            };
            audio_state.shuffled_indices[next_shuffled_pos]
        } else if current_index + 1 < music_files.len() {
            current_index + 1
        } else {
            0
        };

        audio_state.current_index = Some(next_index);
        Ok(next_index)
    } else {
        Err("No track currently playing".to_string())
    }
}

#[tauri::command]
fn skip_previous(
    music_files: Vec<String>,
    state: tauri::State<SharedAudioState>,
) -> Result<usize, String> {
    let mut audio_state = state.lock().unwrap();
    if let Some(current_index) = audio_state.current_index {
        if music_files.is_empty() {
            return Err("No music files available".to_string());
        }

        let prev_index = if audio_state.shuffle {
            if audio_state.shuffled_indices.is_empty() {
                let mut indices: Vec<usize> = (0..music_files.len()).collect();
                use rand::seq::SliceRandom;
                indices.shuffle(&mut rand::thread_rng());
                audio_state.shuffled_indices = indices;
            }

            let current_shuffled_pos = audio_state
                .shuffled_indices
                .iter()
                .position(|&i| i == current_index)
                .unwrap_or(0);
            let prev_shuffled_pos = if current_shuffled_pos > 0 {
                current_shuffled_pos - 1
            } else {
                audio_state.shuffled_indices.len() - 1
            };
            audio_state.shuffled_indices[prev_shuffled_pos]
        } else if current_index > 0 {
            current_index - 1
        } else {
            music_files.len() - 1
        };

        audio_state.current_index = Some(prev_index);
        Ok(prev_index)
    } else {
        Err("No track currently playing".to_string())
    }
}

#[tauri::command]
fn get_current_time(state: tauri::State<SharedAudioState>) -> Result<f64, String> {
    let audio_state = state.lock().unwrap();
    Ok(*audio_state.current_time.lock().unwrap())
}

#[tauri::command]
fn get_duration(state: tauri::State<SharedAudioState>) -> Result<f64, String> {
    let audio_state = state.lock().unwrap();
    Ok(*audio_state.duration.lock().unwrap())
}

#[tauri::command]
fn seek_to_time(time: f64, state: tauri::State<SharedAudioState>) -> Result<String, String> {
    // TODO: Implement seeking! For now, just return Ok
    Ok(format!("Seeked to {} (not fully implemented yet)", time))
}

#[tauri::command]
fn set_volume(volume: f64, state: tauri::State<SharedAudioState>) -> Result<String, String> {
    let mut audio_state = state.lock().unwrap();
    let clamped_volume = volume.max(0.0).min(1.0) as f32;
    audio_state.volume = clamped_volume;
    Ok(format!("Volume set to {}", clamped_volume))
}

#[tauri::command]
fn toggle_shuffle(state: tauri::State<SharedAudioState>) -> Result<bool, String> {
    let mut audio_state = state.lock().unwrap();
    audio_state.shuffle = !audio_state.shuffle;

    if audio_state.shuffle {
        audio_state.shuffled_indices.clear();
    }

    Ok(audio_state.shuffle)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let audio_state = SharedAudioState::new(AudioState {
        is_playing: Arc::new(AtomicBool::new(false)),
        current_index: None,
        shuffle: false,
        shuffled_indices: Vec::new(),
        volume: 0.5,
        current_time: Arc::new(Mutex::new(0.0)),
        duration: Arc::new(Mutex::new(0.0)),
        stream: None,
        tx: None,
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(audio_state)
        .invoke_handler(tauri::generate_handler![
            select_folder,
            get_music_files,
            play_music,
            pause_music,
            resume_music,
            skip_next,
            skip_previous,
            get_current_time,
            get_duration,
            seek_to_time,
            set_volume,
            toggle_shuffle
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
