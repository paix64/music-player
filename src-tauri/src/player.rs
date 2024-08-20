use std::{
    fs::File,
    io::BufReader,
    path::{Path, PathBuf},
    sync::Arc,
    thread,
    time::Duration,
};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};

pub struct Player {
    output_stream: Arc<(OutputStream, OutputStreamHandle)>,
    sink: Arc<Sink>,
    song_length: u32,
    current_song: String,
    volume: f32,
}

impl Player {
    pub fn new() -> Self {
        Self {
            output_stream: Arc::new(OutputStream::try_default().unwrap()),
            sink: Arc::new(Sink::new_idle().0),
            song_length: 0,
            current_song: String::from("None"),
            volume: 0.5,
        }
    }

    pub fn play(&mut self, path: PathBuf) {
        self.sink.stop(); // If it is already running stop it

        println!("{path:?}");

        self.current_song = path
            .clone()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();

        println!("{}", self.current_song);

        self.set_current_song(&path);
        // self.set_song_length(&path);

        self.sink = Arc::new(Sink::try_new(&self.output_stream.1).unwrap());

        // clone sink for thread
        let sclone = self.sink.clone();

        let stream_thread = thread::spawn(move || {
            let file = BufReader::new(File::open(path).unwrap());
            let source = Decoder::new(file).unwrap();

            sclone.append(source);
            sclone.sleep_until_end();
        });

        stream_thread.join().expect("Thread panicked");
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

    pub fn get_current_song(&self) -> String {
        self.current_song.clone()
    }

    pub fn get_song_length(&self) -> u32 {
        self.song_length
    }

    pub fn is_sink_empty(&self) -> bool {
        self.sink.empty()
    }

    pub fn set_current_song(&mut self, path: &PathBuf) {
        self.current_song = path
            .clone()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
    }

    // pub fn set_song_length(&mut self, path: &PathBuf) {
    //     let path = Path::new(&path);
    //     let tagged_file = Probe::open(path)
    //         .expect("ERROR: Bad path provided!")
    //         .read()
    //         .expect("ERROR: Failed to read file!");

    //     let properties = &tagged_file.properties();
    //     let duration = properties.duration();

    //     // update song length, currently playing
    //     self.song_length = duration.as_secs() as u32;
    // }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume += volume;

        if self.volume < 0.0 {
            self.volume = 0.0;
        } else if self.volume > 1.0 {
            self.volume = 1.0;
        }
        self.sink.set_volume(self.volume)
    }

    pub fn get_position(&self) -> Duration {
        self.sink.get_pos()
    }

    pub fn set_position(&self, seconds: Duration) {
        self.sink
            .try_seek(seconds)
            .expect("Cannot change player position");
    }
}
