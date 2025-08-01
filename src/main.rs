use clap::Parser;
use std::{io, sync::mpsc, thread};

mod cli;
mod player;

fn handle_keyboard_input(player: &mut player::AudioPlayer) {
    println!("Starting receving the user input:\n1)resume\n2)pause\n3)new song");
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || loop {
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line.");

        if let Err(_) = tx.send(input.trim().to_string()) {
            println!("Receiver dropped, exiting input thread.");
            break;
        }
    });

    for received in rx.iter() {
        match received.as_str() {
            "resume" => player.resume_audio(),
            _ => println!("nothing to understand"),
        }
    }

    println!("Exiting main thread");
}

fn main() {
    let args = cli::Cli::parse();
    let mut audio_player = player::AudioPlayer::new();
    // handle_keyboard_input(&mut audio_player);
    // ERROR: add an external thread for input procession

    audio_player.play_audio(&args.path);
}
