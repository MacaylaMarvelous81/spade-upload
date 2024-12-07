use clap::Parser;
use spade_serial::{is_running_legacy, upload_game};
use std::fs;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

/// Uploads games to a Sprig device running Spade using serial communications.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The serial port of the Sprig device.
    device: String,
    /// The name that the game should appear under. Limited to 100 bytes.
    name: String,
    /// Path to the JavaScript source of a Sprig game. If not specified, the
    /// game is read from stdin.
    source: Option<PathBuf>,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut port = serialport::new(args.device, 115200)
        .timeout(Duration::from_millis(1000))
        .open()
        .unwrap();

    if is_running_legacy(&mut port).unwrap() {
        eprintln!("The device is a legacy Spade version.");

        ExitCode::FAILURE
    } else {
        let game = match args.source {
            Some(path) => fs::read_to_string(path).unwrap(),
            None => {
                let mut game = String::new();
                stdin().read_to_string(&mut game).unwrap();
                game
            }
        };

        let upload = upload_game(&mut port, &args.name, &game);

        if upload.is_ok() {
            ExitCode::SUCCESS
        } else {
            ExitCode::FAILURE
        }
    }
}
