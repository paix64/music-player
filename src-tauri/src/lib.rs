// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod player;

use lazy_static::lazy_static;
use player::Player;
use std::path::PathBuf;
use tokio::sync::Mutex;
use walkdir::WalkDir;

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
}

#[tauri::command]
async fn get_song_length() -> u32 {
    let mut player = PLAYER.lock().await;
    player.get_song_length()
}

#[tauri::command]
async fn get_song_position() -> u32 {
    let mut player = PLAYER.lock().await;
    player.get_position()
}

#[tauri::command]
async fn play_music() {
    let music_dir = get_music(".");

    let mut player = PLAYER.lock().await;

    player.play(music_dir.get(4).unwrap().to_path_buf());
    println!("{:?}", player.get_current_song());
}

#[tauri::command]
async fn pause_resume() {
    let mut player = PLAYER.lock().await;
    player.pause_resume();
}

#[tauri::command]
async fn skip_music() {
    let mut player = PLAYER.lock().await;
    player.next();
}

#[tauri::command]
async fn add_music() {
    let music_dir = get_music(".");

    let mut player = PLAYER.lock().await;
    player.add_to_queue(music_dir.get(1).unwrap().to_path_buf());
    player.add_to_queue(music_dir.get(2).unwrap().to_path_buf());
    player.add_to_queue(music_dir.get(3).unwrap().to_path_buf());
}

#[tauri::command]
async fn get_current_song() -> String{
    let mut player = PLAYER.lock().await;
    player.get_current_song().unwrap()
}

#[tauri::command]
fn get_music(_dir: &str) -> Vec<PathBuf> {
    let paths = dirs::audio_dir().unwrap();
    let mut path_list: Vec<PathBuf> = vec![];

    for entry in WalkDir::new(paths).into_iter().filter_map(|e| e.ok()) {
        let file_name = entry.file_name().to_str().unwrap();
        let formats = [".mp3", ".ogg", ".wav", ".flac", ".aac", ".m4a"];
        if formats.iter().any(|&format| file_name.contains(format)) {
            path_list.push(entry.path().to_owned());
        }
    }

    path_list
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_music,
            play_music,
            pause_resume,
            skip_music,
            add_music,
            get_song_length,
            get_song_position,
            get_current_song
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
