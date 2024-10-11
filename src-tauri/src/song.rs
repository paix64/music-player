use image::{ImageError, ImageReader};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;
use serde::Serialize;
use std::fs;
use std::io::Cursor;
use std::path::PathBuf;
use std::time::Duration;
use walkdir::WalkDir;

#[derive(Default, Debug, Clone, Serialize)]
pub struct Song {
    path: PathBuf,
    cover_path: Option<PathBuf>,
    pub cover_data: Option<Vec<u8>>,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub track: Option<u32>,

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
            cover_data: None,
            title: None,
            artist: None,
            album: None,
            genre: None,
            year: None,
            track: None,
            duration: Duration::default(),
            channels: None,
            sample_rate: None,
            audio_bitrate: None,
            bit_depth: None,
        }
    }

    pub fn load_metadata(&mut self) {
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
        self.year = metadata_tag.year().map(|s| s as u32);
        self.cover_path = Song::set_cover_path(".", self.album.clone().unwrap_or_default());
        self.cover_data = metadata_tag.pictures().get(0).map(|p| p.data().to_owned());

        let cache = dirs::cache_dir().unwrap_or_default();
        let app_name = "bupl".to_string();
        let format = format!(
            "{}/{}/{}.png",
            cache.display(),
            app_name,
            self.album.clone().unwrap_or_default()
        );
        if fs::metadata(&format).is_err() {
            let _ = self.process_cover_data(self.cover_data.clone(), format);
        }
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_cover_path(&self) -> PathBuf {
        self.cover_path.clone().unwrap_or_default()
    }

    fn process_cover_data(&self, data: Option<Vec<u8>>, format: String) -> Result<(), ImageError> {
        println!("Processing, {}", self.album.clone().unwrap_or_default());

        match data {
            None => {
                println!("Cover metadata not found");
            }
            Some(data) => {
                let mut img = ImageReader::new(Cursor::new(data))
                    .with_guessed_format()?
                    .decode()?;
                img = img.crop_imm(
                    (img.width() - img.height()) / 2,
                    0,
                    img.height(),
                    img.height(),
                );
                img.save(format)?
            }
        }
        Ok(())
    }

    fn set_cover_path(_dir: &str, album: String) -> Option<PathBuf> {
        let paths = dirs::audio_dir().unwrap();
        let mut cover_path = None;

        let cover_file_names = vec![format!("{album}.jpg"), format!("{album}.png")];

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
