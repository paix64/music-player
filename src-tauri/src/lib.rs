// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod player;

use lazy_static::lazy_static;
use player::Player;
use std::{fs, path::PathBuf};
use tokio::sync::Mutex;

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
fn get_files(_dir: &str) -> String {
    let paths = fs::read_dir(dirs::home_dir().unwrap()).unwrap();
    let mut list: Vec<String> = vec![];

    for path in paths {
        list.push(format!("Name: {}", path.unwrap().path().display()));
    }

    format!("{:?}", list)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_files,
            play_music,
            pause_resume,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
