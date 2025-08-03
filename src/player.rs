use rodio;
use rodio::Decoder;
use std::fs::File;
use std::io::{self, BufReader};
use std::sync::mpsc;
use std::thread;

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

    pub fn handle_keyboard_input(&mut self) {
        println!("Starting receiving user input:");
        println!("Commands: 'resume', 'pause', 'new <filename>', 'quit'");

        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            let mut input = String::new();

            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            let input = input.trim().to_string();
            if let Err(_) = tx.send(input) {
                println!("Receiver dropped, exiting input thread.");
                break;
            }
        });

        loop {
            if let Ok(received) = rx.try_recv() {
                match received.as_str() {
                    "resume" => self.resume_audio(),
                    "pause" => self.pause_audio(),
                    "quit" => {
                        println!("Exiting...");
                        break;
                    }
                    input if input.starts_with("new ") => {
                        let file_name = input.strip_prefix("new ").unwrap_or("");
                        if !file_name.is_empty() {
                            self.play_audio(&file_name.to_string());
                        } else {
                            println!("Please provide a filename: new <filename>");
                        }
                    }

                    _ => println!(
                        "Unknown command: '{}'. Available: resume, pause, new <filename>, quit",
                        received
                    ),
                }
            }
        }

        println!("Exiting main thread");
    }
}
