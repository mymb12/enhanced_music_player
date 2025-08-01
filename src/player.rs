use rodio;
use rodio::Decoder;
use std::io::BufReader;
use std::path::PathBuf;
use std::{fs::File, time::Duration};

enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

enum PlayerCommand {
    Play,
    Pause,
    // Stop,
    LoadSong(PathBuf),
}

pub struct AudioPlayer {
    pub stream_handler: rodio::OutputStream,
    playback_state: PlaybackState,
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer {
            stream_handler: rodio::OutputStreamBuilder::open_default_stream()
                .expect("open default audio system"),
            playback_state: PlaybackState::Stopped,
        }
    }

    pub fn resume_audio(&mut self) {
        self.playback_state = PlaybackState::Playing;
        println!("Audio resumed");
    }

    pub fn play_audio(&mut self, file_name: &String) {
        let file = File::open("songs/".to_string() + file_name).unwrap();
        let reader = BufReader::new(file);
        // let _sink = rodio::play(&self.stream_handler.mixer(), file).unwrap();
        let _sink = rodio::Sink::connect_new(&self.stream_handler.mixer());

        let source = Decoder::new(reader).expect("Failed to decode mp3");
        _sink.append(source);

        self.playback_state = PlaybackState::Playing;

        _sink.set_volume(0.2);
        _sink.sleep_until_end();

        self.playback_state = PlaybackState::Stopped;
    }
}
