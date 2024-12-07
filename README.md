# spade-upload
Command-line program, GUI (planned), and crate to upload games to devices
running Spade, like the [Sprig console](https://sprig.hackclub.com). This
is not thread safe.

### Testing
When testing, it may be necessary to make sure the tests do not run in
parallel because the same device will be accessed with thread unsafety.
For example, `cargo test` may be run as `cargo test -- --test-threads 1`.
