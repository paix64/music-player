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
    let mut player = PLAYER.lock().await;
    player.get_position()
}

#[tauri::command]
async fn play_music() {
    let music_dir = get_music(".");

    let mut player = PLAYER.lock().await;

    player.add_to_queue(music_dir.get(4).unwrap().to_path_buf());
    let song_path = player
        .queue()
        .get(0)
        .unwrap()
        .song_path()
        .to_path_buf()
        .clone();
    player.play(song_path);
}

#[tauri::command]
async fn pause_resume() {
    let mut player = PLAYER.lock().await;
    player.pause_resume();
}

#[tauri::command]
async fn seek_position(pos: Duration) {
    let mut player = PLAYER.lock().await;
    player.set_position(pos);
}

#[tauri::command]
async fn skip_music(to_index: i32) {
    let mut player = PLAYER.lock().await;
    player.skip(to_index);
}

#[tauri::command]
async fn song_finished() -> bool {
    let player = PLAYER.lock().await;
    player.song_finished()
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
    let current_song = player.get_current_song_info().unwrap_or_default();

    if key == "title" {
        current_song.title.unwrap_or_default()
    } else if key == "album" {
        current_song.album.unwrap_or_default()
    } else if key == "artist" {
        current_song.artist.unwrap_or_default()
    } else if key == "genre" {
        current_song.genre.unwrap_or_default()
    } else if key == "year" {
        current_song.year.unwrap_or_default().to_string()
    } else if key == "track" {
        current_song.track.unwrap_or_default().to_string()
    } else if key == "track_total" {
        current_song.track_total.unwrap_or_default().to_string()
    } else if key == "duration" {
        player.get_song_duration().to_string()
    } else if key == "album_cover" {
        player.get_album_cover().display().to_string()
    } else {
        String::default()
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
            play_music,
            pause_resume,
            skip_music,
            add_music,
            get_song_position,
            get_current_song_info,
            seek_position,
            song_finished
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
