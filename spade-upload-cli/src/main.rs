use clap::Parser;
use spade_serial::{is_running_legacy, upload_game};
use std::fs;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;
use std::time::Duration;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    device: String,
    name: String,
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
