use rand::seq::SliceRandom;
use rand::thread_rng;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, path::PathBuf, time::Duration, vec};

use crate::song::Song;

pub struct Player {
    _output_stream: (OutputStream, OutputStreamHandle),
    sink: Sink,
    pub current_song: Option<Song>,
    pub queue: Vec<Song>,
    pub repeat_current_song: bool,
    pub is_shuffled: bool,
    queue_index: i32,
    volume: f32,
}

impl Player {
    pub fn new() -> Self {
        let o = OutputStream::try_default().unwrap();
        let s = Sink::try_new(&o.1).unwrap();

        Self {
            _output_stream: o,
            sink: s,
            current_song: None,
            queue: vec![],
            repeat_current_song: false,
            is_shuffled: false,
            queue_index: 0,
            volume: 0.5,
        }
    }

    pub fn play(&mut self, path: PathBuf) {
        self.sink.stop(); // If it is already running stop it

        self.current_song = self.queue.get(self.queue_index as usize).cloned();

        let file = BufReader::new(File::open(path.clone()).unwrap());
        let source = Decoder::new(file).unwrap();

        self.sink.append(source);
        self.sink.play();
    }

    pub fn add_to_queue(&mut self, path: PathBuf) {
        let mut song = Song::new(path);
        song.load_metadata();

        self.queue.push(song);
    }

    pub fn get_song_info(&self, path: PathBuf) -> Song {
        let mut song = Song::new(path);
        song.load_metadata();
        song
    }

    pub fn pause_resume(&mut self) {
        if self.sink.is_paused() {
            self.sink.play()
        } else {
            self.sink.pause()
        }
    }

    pub fn skip(&mut self, to_index: i32) {
        let index = (self.queue_index as i32 + to_index) as usize;
        let next_song = self.queue.get(index).expect("Index does not exist");

        self.queue_index += to_index;
        self.play(next_song.get_path())
    }

    pub fn shuffle(&mut self) {
        if !self.is_shuffled {
            let mut rng = thread_rng();
            self.queue.shuffle(&mut rng);
        }
        self.is_shuffled = true;
    }

    pub fn get_song_duration(&self) -> u32 {
        match &self.current_song {
            Some(s) => Duration::as_secs(&s.duration) as u32,
            None => 0,
        }
    }

    pub fn get_album_cover(&self) -> PathBuf {
        match &self.current_song {
            Some(s) => s.get_cover_path(),
            None => PathBuf::default(),
        }
    }

    pub fn empty_queue(&mut self) {
        self.queue.clear();
        self.sink.clear();
        self.queue_index = 0;
    }

    pub fn get_position(&self) -> u32 {
        Duration::as_secs_f32(&self.sink.get_pos()) as u32
    }

    pub fn not_playing(&self) -> bool {
        self.sink.empty()
    }

    pub fn adjust_volume(&mut self, by: f32) {
        self.volume += by;

        self.volume = self.volume.clamp(0.0, 1.0);
        self.sink.set_volume(self.volume)
    }

    pub fn seek_position(&self, n_seconds: i32) {
        let mut new_position = self.get_position() as i32 + n_seconds;

        new_position = new_position.clamp(0, self.get_song_duration() as i32);
        self.sink
            .try_seek(Duration::from_secs(new_position as u64))
            .expect("Cannot change player position");
    }
}

unsafe impl Sync for Player {}
unsafe impl Send for Player {}
