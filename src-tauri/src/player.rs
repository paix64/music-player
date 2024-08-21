mod song;

use std::{fs::{self, File}, io::{BufReader, Read}, path::{Path, PathBuf}, time::Duration, vec};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use lofty::{file::{AudioFile, TaggedFileExt}, picture, probe::Probe, tag::Accessor};

pub struct Player {
    _output_stream: (OutputStream, OutputStreamHandle),
    sink: Sink,
    song_length: u32,
    current_song: PathBuf,
    queue: Vec<PathBuf>,
    volume: f32,
}

impl Player {
    pub fn new() -> Self {
        let o = OutputStream::try_default().unwrap();
        let s = Sink::try_new(&o.1).unwrap();

        Self {
            _output_stream: o,
            sink: s,
            song_length: 0,
            current_song: PathBuf::from("None"),
            queue: vec![],
            volume: 0.5,
        }
    }

    pub fn play(&mut self, path: PathBuf) {
        self.sink.stop(); // If it is already running stop it

        self.set_current_song_path(&path);
        self.set_song_length(&path);

        self.add_to_queue(path)
    }

    pub fn add_to_queue(&self, path: PathBuf) {
        let file = BufReader::new(File::open(path).unwrap());
        let source = Decoder::new(file).unwrap();

        self.sink.append(source);
    }

    pub fn pause_resume(&mut self) {
        if self.sink.is_paused() {
            self.sink.play()
        } else {
            self.sink.pause()
        }
    }

    pub fn skip(&self) {
        self.sink.skip_one();
    }

    pub fn get_current_song(&self) -> PathBuf {
        self.current_song.clone()
    }

    pub fn get_song_length(&self) -> u32 {
        self.song_length
    }

    pub fn is_sink_empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn set_current_song_path(&mut self, path: &PathBuf) {
        self.current_song = path.to_owned();
        // self.current_song = path
        //     .clone()
        //     .file_name()
        //     .unwrap()
        //     .to_str()
        //     .unwrap()
        //     .to_string();
    }

    pub fn set_song_length(&mut self, path: &PathBuf) {
        let tagged_file = Probe::open(path)
            .expect("ERROR: Bad path provided!")
            .read()
            .expect("ERROR: Failed to read file!");

        let properties = &tagged_file.properties();
        let metadata = &tagged_file.primary_tag().unwrap();
        let duration = properties.duration();
        println!("{:?}",metadata.tag_type());
        println!("{:?}",metadata.title());
        println!("{:?}",metadata.album());
        println!("{:?}",metadata.artist());
        println!("{:?}",metadata.genre());
        println!("{:?}",metadata.track());
        println!("{:?}",metadata.track_total());
        println!("{:?}",metadata.year());

        self.song_length = duration.as_secs() as u32;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume += volume;

        if self.volume < 0.0 {
            self.volume = 0.0;
        } else if self.volume > 1.0 {
            self.volume = 1.0;
        }
        self.sink.set_volume(self.volume)
    }

    pub fn get_position(&self) -> u32 {
        Duration::as_secs_f32(&self.sink.get_pos()) as u32
    }

    pub fn set_position(&self, seconds: Duration) {
        self.sink
            .try_seek(seconds)
            .expect("Cannot change player position");
    }
}

unsafe impl Sync for Player {}
unsafe impl Send for Player {}
