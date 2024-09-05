use serde::Serialize;
use std::path::PathBuf;

use crate::song::Song;

#[derive(Debug, Serialize)]
pub struct Playlist {
    pub name: String,
    pub cover_path: PathBuf,
    pub song_list: Vec<Song>,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            name: String::from("New playlist"),
            cover_path: PathBuf::default(),
            song_list: Vec::default(),
        }
    }

    pub fn new_playlist_from(this: &String, song_list: Vec<Song>) -> Self {
        let first_song_cover = song_list
            .get(0)
            .cloned()
            .unwrap_or_default()
            .get_cover_path();
        Self {
            name: this.to_owned(),
            cover_path: first_song_cover,
            song_list,
        }
    }
}
