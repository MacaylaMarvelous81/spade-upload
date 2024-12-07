# spade-upload
Command-line program, GUI (planned), and crate to upload games to devices
running Spade, like the [Sprig console](https://sprig.hackclub.com). This
is not thread safe.

### Testing
`spade-serial` has a mock serial device for unit testing. Integration tests
use an actual device. When running integration tests, set `TEST_DEVICE` to the
device to use. For example:
```sh
TEST_DEVICE=/dev/cu.usbmodem14101 cargo test
```
