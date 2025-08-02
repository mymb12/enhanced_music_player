use rodio;
use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;

enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

pub struct AudioPlayer {
    pub stream_handler: rodio::OutputStream,
    pub sink: Option<rodio::Sink>,
    playback_state: PlaybackState,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer {
            stream_handler: rodio::OutputStreamBuilder::open_default_stream()
                .expect("open default audio system"),
            sink: None,
            playback_state: PlaybackState::Stopped,
        }
    }

    pub fn resume_audio(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
            self.playback_state = PlaybackState::Playing;

            println!("Song is resumed");
        } else {
            println!("No audio loaded to resume");
        }
    }

    pub fn pause_audio(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
            self.playback_state = PlaybackState::Paused;

            println!("Song is paused");
        } else {
            println!("No audio loaded to pause");
        }
    }

    pub fn play_audio(&mut self, file_name: &String) {
        let file_path = "songs/".to_string() + file_name;
        let file = match File::open(&file_path) {
            Ok(file) => file,
            Err(e) => {
                println!("Failed to open file {}: {}", file_path, e);
                return;
            }
        };

        let reader = BufReader::new(file);
        let source = match Decoder::new(reader) {
            Ok(source) => source,
            Err(e) => {
                println!("Failed to decode audio file: {}", e);
                return;
            }
        };

        let sink = rodio::Sink::connect_new(&self.stream_handler.mixer());
        sink.append(source);
        sink.set_volume(0.2);

        self.sink = Some(sink);
        self.playback_state = PlaybackState::Playing;

        println!("Started playing: {}", file_name);
    }

    pub fn is_finished(&self) -> bool {
        match &self.sink {
            Some(sink) => sink.empty(),
            None => true,
        }
    }

    pub fn get_state(&self) -> &PlaybackState {
        &self.playback_state
    }
}
