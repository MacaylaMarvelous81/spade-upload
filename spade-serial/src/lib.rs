use std::convert::TryFrom;
use std::fmt;
use std::io::{ErrorKind, Read, Write};
use std::num::TryFromIntError;
use std::str::Utf8Error;

pub fn is_running_legacy(io: &mut (impl Write + Read)) -> Result<bool, std::io::Error> {
    let legacy_startup_seq = [0, 1, 2, 3, 4];
    io.write_all(&legacy_startup_seq).and_then(|_| {
        let mut response_buf = [0; 18];
        io.read(&mut response_buf[..]).and_then(|_| {
            std::str::from_utf8(&response_buf)
                .map(|response| response == "found startup seq!")
                .map_err(|_| std::io::Error::from(ErrorKind::InvalidData))
        })
    })
}

pub enum UploadResult {
    AllGood,
    OutOfFlash,
    OutOfMetadata,
}

#[derive(Debug, Clone)]
pub enum UploadError {
    InvalidName,
    FailedConversion,
    IOError,
    NoResponse,
}

impl From<std::io::Error> for UploadError {
    fn from(_value: std::io::Error) -> Self {
        UploadError::IOError
    }
}

impl From<TryFromIntError> for UploadError {
    fn from(_value: TryFromIntError) -> Self {
        UploadError::FailedConversion
    }
}

impl From<Utf8Error> for UploadError {
    fn from(_value: Utf8Error) -> Self {
        UploadError::FailedConversion
    }
}

impl fmt::Display for UploadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "error occured during upload")
    }
}

pub fn upload_game(
    io: &mut (impl Write + Read),
    name: &String,
    game: &String,
) -> Result<UploadResult, UploadError> {
    if name.len() > 100 {
        Err(UploadError::InvalidName)
    } else {
        io.write_all("UPLOAD".as_bytes())?;
        io.write_all(name.as_bytes())?;
        io.write_all(vec![0; 100 - name.len()].as_slice())?;

        let game_len = u32::try_from(game.len())?;
        io.write_all(&game_len.to_le_bytes())?;

        io.write_all(game.as_bytes())?;

        // Look for ALL_GOOD, OO_FLASH, or OO_METADATA
        let mut buf = [0; 11];
        loop {
            buf.rotate_left(1);
            if io.read(&mut buf[10..])? > 0 {
                let buf_str = std::str::from_utf8(&buf)?;

                match buf_str {
                    str if str.contains("ALL_GOOD") => break Ok(UploadResult::AllGood),
                    str if str.contains("OO_FLASH") => break Ok(UploadResult::OutOfFlash),
                    str if str.contains("OO_METADATA") => break Ok(UploadResult::OutOfMetadata),
                    _ => continue,
                }
            } else {
                break Err(UploadError::NoResponse);
            }
        }
    }
}
