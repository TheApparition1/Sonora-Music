use std::path::PathBuf;
use std::sync::Mutex;
use cocoa::base::id;
use cocoa::base::nil;
use cocoa::foundation::{NSString};
use cocoa::foundation::NSAutoreleasePool;

use objc::{class, msg_send, sel, sel_impl};

// Wrapper to make Objective-C objects Send/Sync
struct AudioPlayer {
    player: id,
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        unsafe {
            if self.player != nil {
                let _: () = msg_send![self.player, stop];
                // Let the system handle cleanup - don't manually release
                // AVAudioPlayer has complex internal ARC that crashes with manual release
            }
        }
    }
}

unsafe impl Send for AudioPlayer {}
unsafe impl Sync for AudioPlayer {}

struct AudioState {
    player: Option<AudioPlayer>,
    current_index: Option<usize>,
    repeat_mode: String,
    shuffle_mode: bool,
    is_replacing: bool,
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
fn play_music(file_path: String, index: usize, repeat_mode: String, shuffle_mode: bool, state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        // Validate file path
        if file_path.is_empty() {
            return Err("File path is empty".to_string());
        }
        
        // Take old player out of state and set replacing flag
        let old_player = {
            let mut audio_state = state.lock().expect("Audio state lock poisoned");
            if audio_state.is_replacing {
                return Err("Player is being replaced".to_string());
            }
            audio_state.is_replacing = true;
            audio_state.player.take()
        };
        
        // Stop old player if exists (Drop will handle cleanup)
        if let Some(audio_player) = old_player {
            if audio_player.player != nil {
                let _: () = msg_send![audio_player.player, stop];
            }
        }
        
        // Create NSString safely
        let ns_string: id = NSString::alloc(nil).init_str(&file_path);
        if ns_string == nil {
            let mut audio_state = state.lock().expect("Audio state lock poisoned");
            audio_state.is_replacing = false;
            return Err("Failed to create NSString".to_string());
        }
        
        let url: id = msg_send![class!(NSURL), fileURLWithPath:ns_string];
        let _: () = msg_send![ns_string, release];
        
        if url == nil {
            let mut audio_state = state.lock().expect("Audio state lock poisoned");
            audio_state.is_replacing = false;
            return Err("Failed to create NSURL".to_string());
        }

        let av_player_class = class!(AVAudioPlayer);
        let player: id = msg_send![av_player_class, alloc];
        if player == nil {
            let _: () = msg_send![url, release];
            let mut audio_state = state.lock().expect("Audio state lock poisoned");
            audio_state.is_replacing = false;
            return Err("Failed to allocate AVAudioPlayer".to_string());
        }
        
        let player: id = msg_send![player, initWithContentsOfURL:url error:nil];
        let _: () = msg_send![url, release];

        if player != nil {
            // Set numberOfLoops to -1 for infinite loop when repeat mode is "one"
            if repeat_mode == "one" {
                let _: () = msg_send![player, setNumberOfLoops:-1];
            } else {
                let _: () = msg_send![player, setNumberOfLoops:0];
            }
            
            let _: () = msg_send![player, play];
            
            let mut audio_state = state.lock().expect("Audio state lock poisoned");
            audio_state.player = Some(AudioPlayer { player });
            audio_state.current_index = Some(index);
            audio_state.repeat_mode = repeat_mode;
            audio_state.shuffle_mode = shuffle_mode;
            audio_state.is_replacing = false;
            
            Ok(format!("Playing: {}", file_path))
        } else {
            let mut audio_state = state.lock().expect("Audio state lock poisoned");
            audio_state.is_replacing = false;
            Err("Failed to create audio player".to_string())
        }
    }
}

#[tauri::command]
fn pause_music(state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        let audio_state = state.lock().expect("Audio state lock poisoned");
        if audio_state.is_replacing {
            return Err("Player is being replaced".to_string());
        }
        if let Some(audio_player) = &audio_state.player {
            if audio_player.player != nil {
                let _: () = msg_send![audio_player.player, pause];
                Ok("Paused".to_string())
            } else {
                Err("Player object is invalid".to_string())
            }
        } else {
            Err("No audio playing".to_string())
        }
    }
}

#[tauri::command]
fn resume_music(state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        let audio_state = state.lock().expect("Audio state lock poisoned");
        if audio_state.is_replacing {
            return Err("Player is being replaced".to_string());
        }
        if let Some(audio_player) = &audio_state.player {
            if audio_player.player != nil {
                let _: () = msg_send![audio_player.player, play];
                Ok("Resumed".to_string())
            } else {
                Err("Player object is invalid".to_string())
            }
        } else {
            Err("No audio playing".to_string())
        }
    }
}

#[tauri::command]
fn set_repeat_mode(repeat_mode: String, state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        let mut audio_state = state.lock().expect("Audio state lock poisoned");
        if audio_state.is_replacing {
            return Err("Player is being replaced".to_string());
        }
        audio_state.repeat_mode = repeat_mode.clone();
        
        if let Some(audio_player) = &audio_state.player {
            if audio_player.player != nil {
                if repeat_mode == "one" {
                    let _: () = msg_send![audio_player.player, setNumberOfLoops:-1];
                } else {
                    let _: () = msg_send![audio_player.player, setNumberOfLoops:0];
                }
                Ok(format!("Repeat mode set to {}", repeat_mode))
            } else {
                Err("Player object is invalid".to_string())
            }
        } else {
            Ok("Repeat mode saved".to_string())
        }
    }
}

#[tauri::command]
fn skip_next(music_files: Vec<String>, current_index: usize, repeat_mode: String, shuffle_mode: bool, state: tauri::State<SharedAudioState>) -> Result<usize, String> {
    let mut audio_state = state.lock().expect("Audio state lock poisoned");
    audio_state.repeat_mode = repeat_mode.clone();
    audio_state.shuffle_mode = shuffle_mode;
    
    if repeat_mode == "one" {
        return Ok(current_index);
    }
    
    let next_index = if current_index + 1 < music_files.len() {
        current_index + 1
    } else {
        if repeat_mode == "all" {
            0
        } else {
            return Err("End of playlist".to_string());
        }
    };
    audio_state.current_index = Some(next_index);
    Ok(next_index)
}

#[tauri::command]
fn skip_previous(music_files: Vec<String>, current_index: usize, repeat_mode: String, shuffle_mode: bool, state: tauri::State<SharedAudioState>) -> Result<usize, String> {
    let mut audio_state = state.lock().expect("Audio state lock poisoned");
    audio_state.repeat_mode = repeat_mode.clone();
    audio_state.shuffle_mode = shuffle_mode;
    
    if repeat_mode == "one" {
        return Ok(current_index);
    }
    
    let prev_index = if current_index > 0 {
        current_index - 1
    } else {
        if repeat_mode == "all" {
            music_files.len() - 1
        } else {
            return Err("Start of playlist".to_string());
        }
    };
    audio_state.current_index = Some(prev_index);
    Ok(prev_index)
}

#[tauri::command]
fn get_current_time(state: tauri::State<SharedAudioState>) -> Result<f64, String> {
    unsafe {
        let audio_state = state.lock().expect("Audio state lock poisoned");
        if audio_state.is_replacing {
            return Err("Player is being replaced".to_string());
        }
        if let Some(audio_player) = &audio_state.player {
            if audio_player.player != nil {
                let current_time: f64 = msg_send![audio_player.player, currentTime];
                Ok(current_time)
            } else {
                Err("Player object is invalid".to_string())
            }
        } else {
            Err("No audio playing".to_string())
        }
    }
}

#[tauri::command]
fn get_duration(state: tauri::State<SharedAudioState>) -> Result<f64, String> {
    unsafe {
        let audio_state = state.lock().expect("Audio state lock poisoned");
        if audio_state.is_replacing {
            return Err("Player is being replaced".to_string());
        }
        if let Some(audio_player) = &audio_state.player {
            if audio_player.player != nil {
                let duration: f64 = msg_send![audio_player.player, duration];
                Ok(duration)
            } else {
                Err("Player object is invalid".to_string())
            }
        } else {
            Err("No audio playing".to_string())
        }
    }
}

#[tauri::command]
fn seek_to_time(time: f64, state: tauri::State<SharedAudioState>) -> Result<String, String> {
    unsafe {
        let audio_state = state.lock().expect("Audio state lock poisoned");
        if audio_state.is_replacing {
            return Err("Player is being replaced".to_string());
        }
        if let Some(audio_player) = &audio_state.player {
            if audio_player.player != nil {
                let _: () = msg_send![audio_player.player, setCurrentTime:time];
                Ok(format!("Seeked to {}", time))
            } else {
                Err("Player object is invalid".to_string())
            }
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
        repeat_mode: "off".to_string(),
        shuffle_mode: false,
        is_replacing: false,
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
            set_repeat_mode,
            skip_next,
            skip_previous,
            get_current_time,
            get_duration,
            seek_to_time
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}