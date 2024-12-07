use std::time::Duration;

use serial_test::serial;
use spade_serial::{upload_game, UploadResult};

/// Tests a successful upload. Make sure there is enough space on the device.
#[test]
#[serial]
fn upload() {
    let device = std::env::var("TEST_DEVICE").unwrap();

    let mut port = serialport::new(device, 115200)
        .timeout(Duration::from_millis(1000))
        .open()
        .unwrap();

    assert_eq!(
        upload_game(
            &mut port,
            &String::from("tests/upload.rs"),
            &String::from("console.log('from spade-serial tests')")
        ),
        Ok(UploadResult::AllGood)
    );
}
