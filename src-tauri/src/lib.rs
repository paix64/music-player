// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use rodio::{Decoder, OutputStream, Source};
use std::{
    fs::{self, File},
    io::BufReader,
};

#[tauri::command]
fn play_music() {
    // Get an output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Load a sound from a file, using a path relative to Cargo.toml
    let downloads = dirs::download_dir().expect("Downloads do not exist");
    let dir = format!("{}/music.mp3",downloads.display());

    let file = BufReader::new(File::open(dir).expect("File could not read"));
    // Decode that sound file into a source
    let source = Decoder::new(file).expect("Could not decode file");
    // Play the sound directly on the device
    let _ = stream_handle.play_raw(source.convert_samples());

    // The sound plays in a separate audio thread,
    // so we need to keep the main thread alive while it's playing.
    std::thread::sleep(std::time::Duration::from_secs(3));
}

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
