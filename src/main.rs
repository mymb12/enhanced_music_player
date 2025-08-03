use clap::Parser;

pub mod cli;
pub mod player;

fn main() {
    let args = cli::Cli::parse();
    let mut audio_player = player::AudioPlayer::new();

    audio_player.play_audio(&args.path);
    audio_player.handle_keyboard_input();
}
