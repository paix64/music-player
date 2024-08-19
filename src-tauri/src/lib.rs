// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use rodio::{Decoder, OutputStream, Sink};
use std::{
    fs::{self, File},
    io::BufReader,
};

mod player;
use player::Player;

#[tauri::command]
async fn play_music() {
    // _stream must live as long as the sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();

    let downloads = dirs::download_dir().expect("Downloads do not exist");
    let dir = format!("{}/music.mp3", downloads.display());

    let file = BufReader::new(File::open(dir).expect("File could not read"));
    // Decode that sound file into a source
    let source = Decoder::new(file).expect("Could not decode file");
    // Play the sound directly on the device

    sink.append(source);

    // The sound plays in a separate thread. This call will block the current thread until the sink
    // has finished playing all its queued sounds.
    sink.sleep_until_end();
}

#[tauri::command]
fn pause_resume() {}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
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
        .invoke_handler(tauri::generate_handler![greet, get_files, play_music])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
