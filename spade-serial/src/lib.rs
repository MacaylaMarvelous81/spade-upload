//! Communication with devices running Spade.
//!
//! Issues commands to a device running Spade over a provided serial port. It
//! can be used to upload games and check if the device is running a legacy
//! Spade version. This crate is not thread safe.
//!
//! First, get a `Read + Write`r for the serial port connected to the device.
//! You can do this by using the
//! [serialport](https://crates.io/crates/serialport) crate; this example uses
//! the device at `/dev/cu.usbmodem14101`.
//! ```rust
//! use std::time::Duration;
//!
//! let mut port = serialport::new("/dev/cu.usbmodem14101", 115200)
//!     .timeout(Duration::from_millis(1000))
//!     .open()?;
//! # let legacy = spade_serial::is_running_legacy(&mut port).unwrap_or(false);
//! # Ok::<(), serialport::Error>(())
//! ```
//! Then, pass it to methods in this crate.
//! ```rust
//! # use std::time::Duration;
//! #
//! # let mut port = serialport::new("/dev/cu.usbmodem14101", 115200)
//! #    .timeout(Duration::from_millis(1000))
//! #    .open()?;
//! let legacy = spade_serial::is_running_legacy(&mut port).unwrap_or(false);
//! # Ok::<(), serialport::Error>(())
//! ```
#![warn(missing_docs)]

use std::convert::TryFrom;
use std::fmt;
use std::io::{ErrorKind, Read, Write};
use std::num::TryFromIntError;
use std::str::Utf8Error;

/// Checks if the device is running a legacy Spade version.
///
/// This function issues the legacy startup sequence, `[0, 1, 2, 3, 4]` and
/// interprets the response. If the device responds with 'found startup seq!',
/// the device is found to be running a legacy Spade version, in which case
/// `Ok(true)` will be returned.
///
/// ### Errors
/// This function may return any IO errors from `Write::write_all` or
/// `Read::read`. It may also return an error of `ErrorKind::InvalidData` if
/// the response from the device is not valid UTF-8.
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

/// Represents the possible responses from the device following an UPLOAD
/// command, given the serial communication was successful.
///
/// Returned in a Result by `spade_serial::upload_game`.
pub enum UploadResult {
    /// Represents the response `'ALL_GOOD'`. This means the game was accepted
    /// by the device.
    AllGood,
    /// Represents the response `'OO_FLASH'`. This means the game was rejected
    /// by the device because it did not have enough space to fit into its
    /// flash memory.
    OutOfFlash,
    /// Represents the response `'OO_METADATA'`. This means the game was
    /// rejected by the device because it has already reached the limit of games
    /// that can be stored.
    OutOfMetadata,
}

/// Represents the possible communication errors while trying to upload a game
/// with `spade_serial::upload_game`.
#[derive(Debug, Clone)]
pub enum UploadError {
    /// The name provided was too large (over 100 bytes).
    InvalidName,
    /// An important type conversion failed. This could be due to the size of
    /// `usize` on the running device or invalud UTF-8 from the serial device.
    FailedConversion,
    /// An error occured during an I/O operation, like reading or writing from
    /// the serial port.
    IOError,
    /// The output from the device was read, but no response regarding the
    /// upload operation was found.
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

/// Uploads a game to a device running Spade.
///
/// This function will send a game to a device via the provided `Write` using
/// the `'UPLOAD'` command. Then, it waits for one of the expected
/// `UploadResult`s from the `Read`.
///
/// It returns the `UploadResult` if the I/O communication was successful, and
/// an `UploadError` if an error occurs before the upload completes.
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
