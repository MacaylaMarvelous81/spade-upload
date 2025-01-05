# spade-serial
Rust crate to communicate with devices running Spade, like the
[Sprig console](https://sprig.hackclub.com). It interacts with `Read + Write`rs,
like those provided by the
[serialport crate](https://crates.io/crates/serialport).  
This crate can test whether the device is running a legacy Spade version and
upload games.

### Testing
This crate uses a mock serial device for unit testing which emulates the
expected behavior of the device. Doc tests and integration tests test with
actual devices. The environment variable `TEST_DEVICE` should be set to the
device to use. For example:

```sh
TEST_DEVICE=/dev/cu.usbmodem14101 cargo test
```
