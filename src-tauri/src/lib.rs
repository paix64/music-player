// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod player;
mod playlist;
mod song;

use dirs::config_dir;
use lazy_static::lazy_static;
use player::Player;
use playlist::Playlist;
use song::Song;
use std::{
    fs::{self, read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Mutex;
use walkdir::WalkDir;

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
    static ref APP_NAME: Arc<String> = Arc::new(String::from("burock"));
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
async fn seek_position(n_seconds: i32) {
    let player = PLAYER.lock().await;
    player.seek_position(n_seconds);
}

#[tauri::command]
async fn adjust_volume(by: f32) {
    let mut player = PLAYER.lock().await;
    player.adjust_volume(by);
}

#[tauri::command]
async fn skip_music(to_index: i32) {
    let mut player = PLAYER.lock().await;
    player.skip(to_index);
}

#[tauri::command]
async fn get_queue_of_covers() -> Vec<PathBuf> {
    let player = PLAYER.lock().await;
    player
        .queue
        .iter()
        .map(|song| song.get_cover_path())
        .collect()
}

#[tauri::command]
async fn not_playing() -> bool {
    let player = PLAYER.lock().await;
    player.not_playing()
}

#[tauri::command]
async fn add_music() {
    let music_dir = get_audio_from_path(".");

    let mut player = PLAYER.lock().await;
    player.add_to_queue(music_dir.get(25).unwrap().to_path_buf());
    player.add_to_queue(music_dir.get(13).unwrap().to_path_buf());
    player.add_to_queue(music_dir.get(20).unwrap().to_path_buf());
}

#[tauri::command]
async fn get_current_song_info(key: String) -> String {
    let player = PLAYER.lock().await;
    let current_song = player.current_song.clone().unwrap_or_default();

    match key.as_str() {
        "title" => current_song.title.unwrap_or_default(),
        "album" => current_song.album.unwrap_or_default(),
        "artist" => current_song.artist.unwrap_or_default(),
        "genre" => current_song.genre.unwrap_or_default(),
        "year" => current_song.year.unwrap_or_default().to_string(),
        "track" => current_song.track.unwrap_or_default().to_string(),
        "duration" => player.get_song_duration().to_string(),
        "album_cover" => player.get_album_cover().display().to_string(),
        _ => String::default(),
    }
}

fn get_audio_from_path(_dir: &str) -> Vec<PathBuf> {
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

#[tauri::command]
async fn create_playlists() {
    let player = PLAYER.lock().await;
    let songs = get_audio_from_path("dir");

    for song in songs {
        let info = player.get_song_info(song);
        process_playlist_type("Album ".to_owned() + &info.album.unwrap_or_default());
        process_playlist_type("Artist ".to_owned() + &info.artist.unwrap_or_default());
        process_playlist_type("Genre ".to_owned() + &info.genre.unwrap_or_default());
    }
}

async fn get_songs_of_album(album: &String) -> Vec<Song> {
    let player = PLAYER.lock().await;
    let songs = get_audio_from_path("dir");
    let mut song_list = vec![];

    for song in songs {
        let info = player.get_song_info(song);
        if &info.album.clone().unwrap_or_default() == album {
            song_list.push(info);
        }
    }
    song_list
}

#[tauri::command]
async fn get_album_playlist(album: String) -> Playlist {
    let app_cache_path = format!(
        "{}/{}/cache.bu",
        config_dir().unwrap().display().to_string(),
        Arc::clone(&APP_NAME)
    );
    let cache = read_to_string(&app_cache_path).unwrap_or_default();
    let song_list: Vec<Song> = get_songs_of_album(&album).await;
    let mut playlist = Playlist::new();

    for l in cache.lines() {
        let mut l = l.split_whitespace();
        let type_of = l.next().unwrap_or_default();
        let album_of = &l.clone().collect::<Vec<&str>>().join(&String::from(" "));
        println!("{:?} {:?}", type_of, album_of);
        if album_of == &album {
            playlist = Playlist::new_playlist_from(
                &song_list.get(0).unwrap().album.clone().unwrap(),
                song_list.clone(),
            );
        }
    }
    println!("{:?}", playlist);
    playlist
}

fn process_playlist_type(t: String) {
    let app_cache_path = format!(
        "{}/{}/cache.bu",
        config_dir().unwrap().display().to_string(),
        Arc::clone(&APP_NAME)
    );
    let cache = read_to_string(&app_cache_path).unwrap_or_default();

    if t == String::default() {
        return;
    }
    if cache.contains(&t) {
        return;
    }

    println!("{app_cache_path}");

    let _ = fs::create_dir_all(format!(
        "{}/{}",
        config_dir().unwrap().display().to_string(),
        Arc::clone(&APP_NAME)
    ));

    let mut cache_file = OpenOptions::new()
        .append(true)
        .write(true)
        .create(true)
        .open(&app_cache_path)
        .unwrap();

    let wrt = t + "\n";
    let _ = cache_file.write_all(wrt.as_bytes());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            skip_music,
            add_music,
            play_pause,
            get_song_position,
            get_current_song_info,
            seek_position,
            not_playing,
            get_queue_of_covers,
            adjust_volume,
            create_playlists,
            get_album_playlist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
