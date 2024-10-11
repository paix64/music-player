// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
mod player;
mod playlist;
mod song;

use dirs::config_dir;
use lazy_static::lazy_static;
use reqwest;
use std::{
    fs::{self, read_to_string, OpenOptions},
    io::Write,
    path::PathBuf,
    sync::Arc,
};
use tokio::sync::Mutex;
use walkdir::WalkDir;
use youtube_dl::{SearchOptions, YoutubeDl, YoutubeDlOutput};

use crate::player::Player;
use crate::playlist::Playlist;
use crate::song::Song;

lazy_static! {
    static ref PLAYER: Mutex<Player> = Mutex::new(Player::new());
    static ref APP_NAME: Arc<String> = Arc::new(String::from("bupl"));
}

#[tauri::command]
async fn player_song_position() -> u64 {
    let player = PLAYER.lock().await;
    player.song_position()
}

#[tauri::command]
async fn player_play_or_pause() {
    let song_finished = player_song_finished().await;
    let mut player = PLAYER.lock().await;

    let first_song = player.queue.get(0).unwrap().get_path();
    if song_finished {
        player.play(first_song);
    } else {
        player.pause_resume();
    }
}

#[tauri::command]
async fn player_skip(to: i32) {
    let mut player = PLAYER.lock().await;
    player.skip(to);
}

#[tauri::command]
async fn player_seek_position(by: i32) {
    let player = PLAYER.lock().await;
    player.seek_position(by);
}

#[tauri::command]
async fn player_adjust_volume(by: f32) {
    let mut player = PLAYER.lock().await;
    player.adjust_volume(by);
}

#[tauri::command]
async fn player_shuffle_queue() {
    let mut player = PLAYER.lock().await;
    player.shuffle_queue();
}

#[tauri::command]
async fn player_song_finished() -> bool {
    let player = PLAYER.lock().await;
    player.song_finished()
}

#[tauri::command]
async fn player_song_paused() -> bool {
    let player = PLAYER.lock().await;
    player.song_paused()
}

#[tauri::command]
async fn fetch_album_cover(title: String, album: String) {
    println!("Searching for cover art for: {}", album);
    let cache_path = dirs::cache_dir()
        .ok_or("Failed to get cache directory")
        .unwrap();
    let audio_path = dirs::audio_dir()
        .ok_or("Failed to get audio directory")
        .unwrap();

    let search_query = format!("{} {}", title, album);
    let search = SearchOptions::youtube(&search_query).with_count(1);
    println!("Searching for: {}", search_query);

    let yt_search = YoutubeDl::search_for(&search).run().unwrap();
    let video = match yt_search {
        YoutubeDlOutput::Playlist(playlist) => {
            if let Some(video) = playlist.entries.unwrap().first() {
                video.clone()
            } else {
                return;
            }
        }
        YoutubeDlOutput::SingleVideo(video) => *video,
    };

    let thumbnail_url = video.thumbnail.ok_or("No thumbnail found").unwrap();
    println!("Thumbnail URL: {}", thumbnail_url);

    let thumbnail_path = format!(
        "{}/{}/{}.png",
        cache_path.display(),
        Arc::clone(&APP_NAME),
        album
    );
    let response = reqwest::get(&thumbnail_url).await.unwrap();
    let mut file = tokio::fs::File::create(&thumbnail_path).await.unwrap();
    let mut content = response.bytes().await.unwrap();
    tokio::io::copy(&mut content.as_ref(), &mut file)
        .await
        .unwrap();

    let video_url = video.webpage_url.ok_or("No video URL found").unwrap();
    let audio_output = YoutubeDl::new(&video_url)
        .socket_timeout("15")
        .extract_audio(true)
        .format("m4a")
        .run()
        .unwrap();
}

#[tauri::command]
async fn player_current_song_info(key: String) -> String {
    let player = PLAYER.lock().await;
    let current_song = player.current_song.clone().unwrap_or_default();

    match key.as_str() {
        "title" => current_song.title.unwrap_or_default(),
        "album" => current_song.album.unwrap_or_default(),
        "artist" => current_song.artist.unwrap_or_default(),
        "genre" => current_song.genre.unwrap_or_default(),
        "year" => current_song.year.unwrap_or_default().to_string(),
        "track" => current_song.track.unwrap_or_default().to_string(),
        "duration" => current_song.duration.as_secs().to_string(),
        "cover_path" => current_song.get_cover_path().display().to_string(),
        _ => String::default(),
    }
}

#[tauri::command]
async fn player_cover_path_queue() -> Vec<PathBuf> {
    let player = PLAYER.lock().await;
    player
        .queue
        .iter()
        .map(|song| song.get_cover_path())
        .collect()
}

#[tauri::command]
async fn player_repeat() -> bool {
    let player = PLAYER.lock().await;
    player.repeat
}

#[tauri::command]
async fn player_toggle_repeat() {
    let mut player = PLAYER.lock().await;
    player.repeat = !player.repeat;
}

#[tauri::command]
async fn create_playlist_types() {
    let player = PLAYER.lock().await;
    let songs = get_audio_from_path("dir");

    for song in songs {
        let info = player.get_song_info(song);
        process_playlist_type("Album ".to_owned() + &info.album.unwrap_or_default());
        process_playlist_type("Artist ".to_owned() + &info.artist.unwrap_or_default());
        process_playlist_type("Genre ".to_owned() + &info.genre.unwrap_or_default());
    }
}

#[tauri::command]
async fn play_album_playlist(album: String) {
    let playlist = get_album_playlist(album).await;
    let mut player = PLAYER.lock().await;
    player.empty_queue();
    for song in playlist.song_list {
        player.add_to_queue(song.get_path());
    }
    let first_song = player.queue.get(0).cloned().unwrap_or_default().get_path();
    player.play(first_song)
}

#[tauri::command]
async fn get_album_playlists() -> Vec<Playlist> {
    let app_cache_path = format!(
        "{}/{}/cache.bu",
        config_dir().unwrap().display().to_string(),
        Arc::clone(&APP_NAME)
    );
    let cache = read_to_string(&app_cache_path).unwrap_or_default();
    let mut playlist_list = vec![];

    for l in cache.lines() {
        let mut l = l.split_whitespace();
        let type_of = l.next().unwrap_or_default();
        let album_of = l.clone().collect::<Vec<&str>>().join(&String::from(" "));
        if type_of == "Album" {
            let playlist = get_album_playlist(album_of).await;
            if playlist.cover_path.exists() {
                playlist_list.push(playlist)
            }
        }
    }
    playlist_list
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
        let _type_of = l.next().unwrap_or_default();
        let album_of = &l.clone().collect::<Vec<&str>>().join(&String::from(" "));
        if album_of == &album {
            playlist = Playlist::new_playlist_from(
                &song_list
                    .get(0)
                    .cloned()
                    .unwrap_or_default()
                    .album
                    .clone()
                    .unwrap_or_default(),
                song_list.clone(),
            );
        }
    }
    playlist
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
            player_song_position,
            player_song_paused,
            player_song_finished,
            player_skip,
            player_play_or_pause,
            player_current_song_info,
            player_seek_position,
            player_repeat,
            player_toggle_repeat,
            player_cover_path_queue,
            player_adjust_volume,
            player_shuffle_queue,
            create_playlist_types,
            get_album_playlists,
            play_album_playlist,
            fetch_album_cover
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
