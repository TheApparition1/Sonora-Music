use std::path::PathBuf;
use std::sync::Mutex;
use cocoa::base::{id, nil};
use cocoa::foundation::NSString;
use objc::{class, msg_send, sel, sel_impl};

// Wrapper to make Objective-C objects Send/Sync
struct AudioPlayer {
    player: id,
}

unsafe impl Send for AudioPlayer {}
unsafe impl Sync for AudioPlayer {}

struct AudioState {
    player: Option<AudioPlayer>,
    current_index: Option<usize>,
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
        let extension = file_path.extension()
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
fn play_music(file_path: String, index: usize, state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        // Stop current player if exists
        {
            let audio_state = state.lock().unwrap();
            if let Some(audio_player) = &audio_state.player {
                let _: () = msg_send![audio_player.player, stop];
            }
        }

        let ns_string: id = NSString::alloc(nil).init_str(&file_path);
        let url: id = msg_send![class!(NSURL), fileURLWithPath:ns_string];

        let av_player_class = class!(AVAudioPlayer);
        let player: id = msg_send![av_player_class, alloc];
        let mut error: id = nil;
        let player: id = msg_send![player, initWithContentsOfURL:url error:&mut error];

        if player != nil {
            let _: () = msg_send![player, play];

            let mut audio_state = state.lock().unwrap();
            audio_state.player = Some(AudioPlayer { player });
            audio_state.current_index = Some(index);

            Ok(format!("Playing: {}", file_path))
        } else {
            Err(format!("Failed to create audio player for: {}", file_path))
        }
    }
}

#[tauri::command]
fn pause_music(state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        let audio_state = state.lock().unwrap();
        if let Some(audio_player) = &audio_state.player {
            let _: () = msg_send![audio_player.player, pause];
            Ok("Paused".to_string())
        } else {
            Err("No audio playing".to_string())
        }
    }
}

#[tauri::command]
fn resume_music(state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        let audio_state = state.lock().unwrap();
        if let Some(audio_player) = &audio_state.player {
            let _: () = msg_send![audio_player.player, play];
            Ok("Resumed".to_string())
        } else {
            Err("No audio playing".to_string())
        }
    }
}

#[tauri::command]
fn skip_next(music_files: Vec<String>, state: tauri::State<SharedAudioState>) -> Result<usize, String> {
    let mut audio_state = state.lock().unwrap();
    if let Some(current_index) = audio_state.current_index {
        if music_files.is_empty() {
            return Err("No music files available".to_string());
        }
        let next_index = if current_index + 1 < music_files.len() {
            current_index + 1
        } else {
            0 // Loop back to start
        };
        audio_state.current_index = Some(next_index);
        Ok(next_index)
    } else {
        Err("No track currently playing".to_string())
    }
}

#[tauri::command]
fn skip_previous(music_files: Vec<String>, state: tauri::State<SharedAudioState>) -> Result<usize, String> {
    let mut audio_state = state.lock().unwrap();
    if let Some(current_index) = audio_state.current_index {
        if music_files.is_empty() {
            return Err("No music files available".to_string());
        }
        let prev_index = if current_index > 0 {
            current_index - 1
        } else {
            music_files.len() - 1 // Loop to end
        };
        audio_state.current_index = Some(prev_index);
        Ok(prev_index)
    } else {
        Err("No track currently playing".to_string())
    }
}

#[tauri::command]
fn get_current_time(state: tauri::State<SharedAudioState>) -> Result<f64, String> {
    unsafe {
        let audio_state = state.lock().unwrap();
        if let Some(audio_player) = &audio_state.player {
            let current_time: f64 = msg_send![audio_player.player, currentTime];
            Ok(current_time)
        } else {
            Err("No audio playing".to_string())
        }
    }
}

#[tauri::command]
fn get_duration(state: tauri::State<SharedAudioState>) -> Result<f64, String> {
    unsafe {
        let audio_state = state.lock().unwrap();
        if let Some(audio_player) = &audio_state.player {
            let duration: f64 = msg_send![audio_player.player, duration];
            Ok(duration)
        } else {
            Err("No audio playing".to_string())
        }
    }
}

#[tauri::command]
fn seek_to_time(time: f64, state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        let audio_state = state.lock().unwrap();
        if let Some(audio_player) = &audio_state.player {
            let _: () = msg_send![audio_player.player, setCurrentTime:time];
            Ok(format!("Seeked to {}", time))
        } else {
            Err("No audio playing".to_string())
        }
    }
}

#[tauri::command]
fn set_volume(volume: f64, state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        let audio_state = state.lock().unwrap();
        if let Some(audio_player) = &audio_state.player {
            let _: () = msg_send![audio_player.player, setVolume:volume];
            Ok(format!("Volume set to {}", volume))
        } else {
            Err("No audio playing".to_string())
        }
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let audio_state = SharedAudioState::new(AudioState {
        player: None,
        current_index: None,
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
            set_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}