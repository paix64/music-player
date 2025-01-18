use image::{ImageError, ImageReader};
use lofty::file::{AudioFile, TaggedFileExt};
use lofty::probe::Probe;
use lofty::tag::Accessor;
use reqwest;
use serde::Serialize;
use std::fs;
use std::io::Cursor;
use std::path::{Path, PathBuf};
use std::time::Duration;
use youtube_dl::{SearchOptions, SingleVideo, YoutubeDl, YoutubeDlOutput};

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

    pub async fn load_metadata(&mut self) {
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
        self.cover_data = metadata_tag.pictures().get(0).map(|p| p.data().to_owned());

        let cache = dirs::cache_dir().unwrap_or_default();

        let format = format!(
            "{}/{}/{}",
            cache.display(),
            "bupl".to_string(),
            self.album.clone().unwrap_or_default()
        );

        let extensions = ["webp", "png", "jpg"];
        let cover_exists = extensions.iter().any(|ext| {
            let path = Path::new(&format).with_extension(ext);
            fs::metadata(&path).is_ok()
        });

        if !cover_exists {
            println!("Cover not found, fetching...");
            let _ = self
                .process_cover_data(self.cover_data.clone(), format)
                .await;
        }

        self.cover_path = Song::set_cover_path(self.album.clone().unwrap_or_default());
    }

    pub fn get_path(&self) -> PathBuf {
        self.path.clone()
    }

    pub fn get_cover_path(&self) -> PathBuf {
        self.cover_path.clone().unwrap_or_default()
    }

    async fn process_cover_data(
        &self,
        data: Option<Vec<u8>>,
        format: String,
    ) -> Result<(), ImageError> {
        println!("Processing, {}", self.album.clone().unwrap_or_default());

        match data {
            None => {
                println!("Cover metadata not found");
                fetch_album_cover(
                    self.title.clone().unwrap_or_default(),
                    self.album.clone().unwrap_or_default(),
                )
                .await;
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

    fn set_cover_path(album: String) -> Option<PathBuf> {
        let cache_path = format!(
            "{}/{}",
            dirs::cache_dir().unwrap().display(),
            "bupl".to_string()
        );
        let mut cover_path = None;

        let _ = fs::create_dir_all(&cache_path);

        for entry in fs::read_dir(&cache_path).unwrap().filter_map(|e| e.ok()) {
            if entry.file_type().unwrap().is_file() {
                let file_name_str = entry.file_name();
                let file_name = file_name_str.to_str().unwrap().split(".").next().unwrap();
                if file_name == album {
                    cover_path = Some(entry.path());
                    break;
                }
            }
        }
        println!("Cover path: {:?}", cover_path);
        cover_path
    }
}

async fn fetch_video_info(title: String, album: String) -> SingleVideo {
    let search_query = format!("{} {}", title, album);
    let search = SearchOptions::youtube(&search_query);
    println!("Searching for: {}", search_query);

    let yt_search = YoutubeDl::search_for(&search).run().unwrap();
    let video = match yt_search {
        YoutubeDlOutput::Playlist(playlist) => {
            if let Some(video) = playlist.entries.unwrap().first() {
                video.clone()
            } else {
                return SingleVideo::default();
            }
        }
        YoutubeDlOutput::SingleVideo(video) => *video,
    };
    video
}

async fn fetch_album_cover(title: String, album: String) {
    println!("Searching for cover art for: {}", album);
    let cache_path = dirs::cache_dir()
        .ok_or("Failed to get cache directory")
        .unwrap();

    let video = fetch_video_info(title, album.clone()).await;

    let thumbnail_url = video.thumbnail.ok_or("No thumbnail found").unwrap();
    println!("Thumbnail URL: {}", thumbnail_url);

    let extension = if let Some(query_start) = thumbnail_url.find('?') {
        thumbnail_url[..query_start]
            .split('.')
            .last()
            .unwrap_or("jpg")
    } else {
        thumbnail_url.split('.').last().unwrap_or("jpg")
    };

    let thumbnail_path = format!(
        "{}/{}/{}.{}",
        cache_path.display(),
        "bupl".to_string(),
        &album,
        extension
    );
    let response = reqwest::get(&thumbnail_url).await.unwrap();
    let mut file = tokio::fs::File::create(&thumbnail_path).await.unwrap();
    let content = response.bytes().await.unwrap();
    tokio::io::copy(&mut content.as_ref(), &mut file)
        .await
        .unwrap();
}
