use rodio;
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
    pub file_path: String,
    playback_state: PlaybackState,
}
impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer {
            stream_handler: rodio::OutputStreamBuilder::open_default_stream()
                .expect("open default audio system"),
            file_path: String::from("songs/"),
            playback_state: PlaybackState::Stopped,
        }
    }
    pub fn resume_audio(&mut self) {
        self.playback_state = PlaybackState::Playing;
        println!("Audio resumed");
    }

    pub fn play_audio(&mut self, duration: Duration, file_name: &String) {
        self.file_path = String::from("songs/") + file_name;

        let file = File::open(&self.file_path).unwrap();
        let _sink = rodio::play(&self.stream_handler.mixer(), file).unwrap();

        self.playback_state = PlaybackState::Playing;
        std::thread::sleep(duration);
        self.playback_state = PlaybackState::Stopped;
    }
}
