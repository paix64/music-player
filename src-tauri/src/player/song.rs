use std::path::PathBuf;
use std::time::Duration;

use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;

#[derive(Default, Debug, Clone)]
pub struct Song {
    name: String,
    path: PathBuf,
    audio_metadata: AudioMetadata,
    music_metadata: MusicMetadata,
}

impl Song {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .map(|s| s.to_string_lossy())
            .unwrap()
            .into_owned();

        Self {
            name,
            path,
            audio_metadata: AudioMetadata::default(),
            music_metadata: MusicMetadata::default(),
        }
    }

    pub fn load_metadata(&mut self) {
        println!("Loading metadata for {:?}", self.song_path());

        let tag_file = Probe::open(&self.path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let audio_properties = &tag_file.properties();
        let metadata_tag = &tag_file.primary_tag().expect("Could not get metadata tag");

        // Audio Metadata
        let duration = audio_properties.duration();
        let channels = audio_properties.channels();
        let sample_rate = audio_properties.sample_rate();
        let audio_bitrate = audio_properties.audio_bitrate();
        let bit_depth = audio_properties.bit_depth();

        // Music Metadata
        let title = metadata_tag.title().map(|s| s.to_string());
        let album = metadata_tag.album().map(|s| s.to_string());
        let artist = metadata_tag.artist().map(|s| s.to_string());
        let genre = metadata_tag.genre().map(|s| s.to_string());
        let track = metadata_tag.track().map(|s| s as u32);
        let track_total = metadata_tag.track_total().map(|s| s as u32);
        let year = metadata_tag.year().map(|s| s as u32);

        let music_metadata =
            MusicMetadata::new(title, artist, album, genre, year, track, track_total);
        let audio_metadata =
            AudioMetadata::new(duration, channels, sample_rate, audio_bitrate, bit_depth);

        self.audio_metadata = audio_metadata;
        self.music_metadata = music_metadata;
    }

    pub fn song_path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn audio_metadata(&self) -> &AudioMetadata {
        &self.audio_metadata
    }

    pub fn music_metadata(&self) -> &MusicMetadata {
        &self.music_metadata
    }
}

#[derive(Default, Debug, Clone)]
pub struct AudioMetadata {
    pub duration: Duration,
    pub channels: Option<u8>,
    pub sample_rate: Option<u32>,
    pub audio_bitrate: Option<u32>,
    pub bit_depth: Option<u8>,
}

impl AudioMetadata {
    pub fn new(
        duration: Duration,
        channels: Option<u8>,
        sample_rate: Option<u32>,
        audio_bitrate: Option<u32>,
        bit_depth: Option<u8>,
    ) -> Self {
        Self {
            duration,
            channels,
            sample_rate,
            audio_bitrate,
            bit_depth,
        }
    }
}

#[derive(Default, Debug, Clone)]
pub struct MusicMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub genre: Option<String>,
    pub year: Option<u32>,
    pub track: Option<u32>,
    pub track_total: Option<u32>,
}

impl MusicMetadata {
    pub fn new(
        title: Option<String>,
        artist: Option<String>,
        album: Option<String>,
        genre: Option<String>,
        year: Option<u32>,
        track: Option<u32>,
        track_total: Option<u32>,
    ) -> Self {
        Self {
            title,
            artist,
            album,
            genre,
            year,
            track,
            track_total,
        }
    }
}
