use std::path::PathBuf;
use std::time::Duration;

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;
use walkdir::WalkDir;

#[derive(Default, Debug, Clone)]
pub struct Song {
    path: PathBuf,
    cover_path: Option<PathBuf>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub track: Option<u32>,
    pub track_total: Option<u32>,
    pub duration: Duration,
    pub channels: Option<u8>,
    pub sample_rate: Option<u32>,
    pub audio_bitrate: Option<u32>,
    pub bit_depth: Option<u8>,
}

impl Song {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            cover_path: None,
            title: None,
            artist: None,
            album: None,
            genre: None,
            year: None,
            track: None,
            track_total: None,
            duration: Duration::from_secs(0),
            channels: None,
            sample_rate: None,
            audio_bitrate: None,
            bit_depth: None,
        }
    }

    pub fn load_metadata(&mut self) {
        println!("Loading metadata for {:?}", self.path);

        let tag_file = Probe::open(&self.path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let audio_properties = &tag_file.properties();
        let metadata_tag = &tag_file.primary_tag().expect("Could not get metadata tag");

        // Audio Metadata
        self.duration = audio_properties.duration();
        self.channels = audio_properties.channels();
        self.sample_rate = audio_properties.sample_rate();
        self.audio_bitrate = audio_properties.audio_bitrate();
        self.bit_depth = audio_properties.bit_depth();

        // Music Metadata
        self.title = metadata_tag.title().map(|s| s.to_string());
        self.album = metadata_tag.album().map(|s| s.to_string());
        self.artist = metadata_tag.artist().map(|s| s.to_string());
        self.genre = metadata_tag.genre().map(|s| s.to_string());
        self.track = metadata_tag.track().map(|s| s as u32);
        self.track_total = metadata_tag.track_total().map(|s| s as u32);
        self.year = metadata_tag.year().map(|s| s as u32);
        self.cover_path = Song::set_cover_path(".", self.album.clone().unwrap_or_default());
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_cover_path(&self) -> PathBuf {
        self.cover_path.clone().unwrap_or_default()
    }

    fn set_cover_path(_dir: &str, album: String) -> Option<PathBuf> {
        let paths = dirs::audio_dir().unwrap();
        let mut cover_path = None;

        let cover_file_names = vec![format!("{album}-cover.jpg"), format!("{album}-cover.png")];

        for entry in WalkDir::new(paths).into_iter().filter_map(|e| e.ok()) {
            let file_name = entry.file_name().to_str().unwrap();
            if cover_file_names.iter().any(|name| file_name.contains(name)) {
                cover_path = Some(entry.path().to_owned());
                break;
            }
        }
        cover_path
    }
}
