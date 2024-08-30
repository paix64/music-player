use std::path::PathBuf;

use serde::Serialize;

use crate::song::Song;

#[derive(Debug, Serialize)]
pub struct Playlist {
    name: String,
    cover_path: PathBuf,
    song_list: Vec<Song>,
}

impl Playlist {
    pub fn new() -> Self {
        Self {
            name: String::from("New playlist"),
            cover_path: PathBuf::default(),
            song_list: Vec::default(),
        }
    }

    pub fn new_album_playlist_from(song: &Song, song_list: Vec<Song>) -> Self {
        Self {
            name: song.album.clone().unwrap_or_default(),
            cover_path: song.get_cover_path(),
            song_list,
        }
    }

    pub fn new_playlist_from(this: &String, song_list: Vec<Song>) -> Self {
        let first_song_cover = song_list.get(0).unwrap().get_cover_path();
        Self {
            name: this.to_owned(),
            cover_path: first_song_cover,
            song_list,
        }
    }
}
