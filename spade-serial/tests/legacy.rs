use std::time::Duration;

use serial_test::serial;
use spade_serial::is_running_legacy;

/// Test on a device running a non-legacy Spade version.
#[test]
#[serial]
fn test_legacy() {
    let device = std::env::var("TEST_DEVICE").unwrap();

    let mut port = serialport::new(device, 115200)
        .timeout(Duration::from_millis(1000))
        .open()
        .unwrap();

    assert!(!is_running_legacy(&mut port).unwrap());
}
