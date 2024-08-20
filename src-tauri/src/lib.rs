// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

mod player;

use std::{fs, path::PathBuf};

use player::Player;

#[tauri::command]
async fn play_music() {
    let downloads = dirs::download_dir().expect("Downloads do not exist");
    let dir = PathBuf::from(format!("{}/music.mp3", downloads.display()));

    let mut player = Player::new();
    let _ = player.play(dir);

    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // let sink = Sink::try_new(&stream_handle).unwrap();

    // let file = BufReader::new(File::open(dir).expect("File could not read"));
    // let source = Decoder::new(file).expect("Could not decode file");
    // sink.append(source);

    // // The sound plays in a separate thread. This call will block the current thread until the sink
    // // has finished playing all its queued sounds.
    // sink.sleep_until_end();
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
        .invoke_handler(tauri::generate_handler![get_files, play_music])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
