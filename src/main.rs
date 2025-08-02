use clap::Parser;
use std::{io, sync::mpsc, thread, time::Duration};

use crate::player::AudioPlayer;

pub mod cli;
pub mod player;

/* fn handle_keyboard_input(player: &mut player::AudioPlayer) {
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
} */

fn handle_keyboard_input(player: &mut AudioPlayer) {
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
                "resume" => player.resume_audio(),
                "pause" => player.pause_audio(),
                "quit" => {
                    println!("Exiting...");
                    break;
                }
                input if input.starts_with("new ") => {
                    let file_name = input.strip_prefix("new ").unwrap_or("");
                    if !file_name.is_empty() {
                        player.play_audio(&file_name.to_string());
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

fn main() {
    let args = cli::Cli::parse();
    let mut audio_player = player::AudioPlayer::new();

    audio_player.play_audio(&args.path);

    handle_keyboard_input(&mut audio_player);
}
