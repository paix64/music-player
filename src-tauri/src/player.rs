mod song;

use std::{fs::File, io::BufReader, path::PathBuf, time::Duration, vec};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

use song::Song;

pub struct Player {
    _output_stream: (OutputStream, OutputStreamHandle),
    sink: Sink,
    pub current_song: Option<Song>,
    pub queue: Vec<Song>,
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

    pub fn get_song_duration(&mut self) -> u32 {
        match &self.current_song {
            Some(s) => Duration::as_secs(&s.duration) as u32,
            None => 0,
        }
    }

    pub fn get_album_cover(&mut self) -> PathBuf {
        match &self.current_song {
            Some(s) => s.get_cover_path(),
            None => PathBuf::default(),
        }
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

    pub fn seek_position(&self, seconds: u64) {
        self.sink
            .try_seek(Duration::from_secs(seconds))
            .expect("Cannot change player position");
    }
}

unsafe impl Sync for Player {}
unsafe impl Send for Player {}
