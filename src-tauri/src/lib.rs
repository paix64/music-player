// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod player;

use lazy_static::lazy_static;
use player::Player;
use std::{path::PathBuf, time::Duration};
use tokio::sync::Mutex;
use walkdir::WalkDir;

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
}

#[tauri::command]
async fn get_song_position() -> u32 {
    let player = PLAYER.lock().await;
    player.get_position()
}

#[tauri::command]
async fn play_pause() {
    let mut player = PLAYER.lock().await;

    let first_song = player.queue.get(0).unwrap().get_path();
    if player.not_playing() {
        player.play(first_song);
    } else {
        player.pause_resume();
    }
}

#[tauri::command]
async fn seek_position(pos: Duration) {
    let player = PLAYER.lock().await;
    player.set_position(pos);
}

#[tauri::command]
async fn skip_music(to_index: i32) {
    let mut player = PLAYER.lock().await;
    player.skip(to_index);
}

#[tauri::command]
async fn get_queue_of_covers() -> Vec<PathBuf> {
    let player = PLAYER.lock().await;
    player.queue.iter().map(|song| song.get_cover_path()).collect()
}

#[tauri::command]
async fn not_playing() -> bool {
    let player = PLAYER.lock().await;
    player.not_playing()
}

#[tauri::command]
async fn add_music() {
    let music_dir = get_music(".");

    let mut player = PLAYER.lock().await;
    player.add_to_queue(music_dir.get(23).unwrap().to_path_buf());
    player.add_to_queue(music_dir.get(20).unwrap().to_path_buf());
    player.add_to_queue(music_dir.get(13).unwrap().to_path_buf());
}

#[tauri::command]
async fn get_current_song_info(key: String) -> String {
    let mut player = PLAYER.lock().await;
    let current_song = player.current_song.clone().unwrap_or_default();

    match key.as_str() {
        "title" => current_song.title.unwrap_or_default(),
        "album" => current_song.album.unwrap_or_default(),
        "artist" => current_song.artist.unwrap_or_default(),
        "genre" => current_song.genre.unwrap_or_default(),
        "year" => current_song.year.unwrap_or_default().to_string(),
        "track" => current_song.track.unwrap_or_default().to_string(),
        "track_total" => current_song.track_total.unwrap_or_default().to_string(),
        "duration" => player.get_song_duration().to_string(),
        "album_cover" => player.get_album_cover().display().to_string(),
        _ => String::default(),
    }
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
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_music,
            skip_music,
            add_music,
            play_pause,
            get_song_position,
            get_current_song_info,
            seek_position,
            not_playing,
            get_queue_of_covers
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
