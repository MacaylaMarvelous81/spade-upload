use std::fs;
use std::io::{stdin, Read, Write};
use std::path::PathBuf;
use std::process::{ExitCode};
use std::time::Duration;
use clap::Parser;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    device: String,
    name: String,
    source: Option<PathBuf>
}

fn is_legacy_serial(io: &mut (impl Write + Read)) -> Result<bool, &str> {
    let legacy_startup_seq = [0, 1, 2, 3, 4];
    io.write(&legacy_startup_seq).map_err(|_| "Failed to write legacy startup sequence.")?;

    let mut response_buf = [0; 18];
    io.read(&mut response_buf[..]).map_err(|_| "Failed to read response from device.")?;

    let response = std::str::from_utf8(&response_buf);

    match response {
        Ok(response) => Ok(response == "found startup seq!"),
        Err(_) => Err("Received response in invalid encoding")
    }
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut port = serialport::new(args.device, 115200).timeout(Duration::from_millis(1000)).open().unwrap();

    if is_legacy_serial(&mut port).unwrap() {
        eprintln!("The device is a legacy Spade version.");

        ExitCode::FAILURE
    } else {
        port.write("UPLOAD".as_bytes()).unwrap();
        port.flush().unwrap();
        let name = &args.name[0..];
        port.write(name.as_bytes()).unwrap();
        port.write(vec![0; 100 - name.len()].as_slice()).unwrap();
        port.flush().unwrap();
        let game = match args.source {
            Some(path) => fs::read_to_string(path).unwrap(),
            None => {
                let mut game = String::new();
                stdin().read_to_string(&mut game).unwrap();
                game
            }
        };
        let length = u32::try_from(game.len()).unwrap();
        port.write(&length.to_le_bytes()).unwrap();
        port.write(game.as_bytes()).unwrap();
        port.flush().unwrap();

        // Look for ALL_GOOD, OO_FLASH, or OO_METADATA
        let mut buf = [0; 11];
        loop {
            buf.rotate_left(1);
            port.read(&mut buf[10..]).unwrap();
            let buf_str = std::str::from_utf8(&buf).unwrap();
            if buf_str.contains("ALL_GOOD") || buf_str.contains("OO_FLASH") || buf_str.contains("OO_METADATA") {
                break;
            }
        }

        ExitCode::SUCCESS
    }
}
