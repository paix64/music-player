// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod player;

use lazy_static::lazy_static;
use player::Player;
use std::{fs, path::PathBuf};
use tokio::sync::Mutex;
use walkdir::WalkDir;

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
}

#[tauri::command]
async fn play_music() {
    let downloads = dirs::download_dir().expect("Downloads do not exist");
    let dir = PathBuf::from(format!("{}/music.mp3", downloads.display()));

    let mut player = PLAYER.lock().await;

    let _ = player.play(dir);
}

#[tauri::command]
async fn pause_resume() {
    let mut player = PLAYER.lock().await;
    // println!("{}", player.get_current_song());
    player.pause_resume();
}

#[tauri::command]
fn get_music(_dir: &str) -> String {
    let paths = dirs::audio_dir().unwrap();
    let mut list: Vec<String> = vec![];

    for entry in WalkDir::new(paths).into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_str().unwrap();
        let formats = [".mp3", ".ogg", ".wav", ".flac", ".aac", ".m4a"];
        if formats.iter().any(|&format| file_name.contains(format)) {
            list.push(format!("{}", entry.path().display()));
        }
    }

    format!("{:?}", list)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_music,
            play_music,
            pause_resume,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
