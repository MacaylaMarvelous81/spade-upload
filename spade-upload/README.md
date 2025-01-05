# spade-upload

Command-line utility which uploads games to a device running Spade, like the
[Sprig console](https://sprig.hackclub.com).

## Usage

Connect the Sprig to a device with spade-upload, then run pass the device,
game name, and game path to it. Alternatively, pipe the game source to
spade-upload instead of providing a path.

## Reference

> The same information can be accessed with `spade-upload -h`

**Usage**: **spade-upload** &lt;DEVICE&gt; &lt;NAME&gt; [SOURCE]

**Arguments:**

- &lt;DEVICE&gt;: The serial port of the Sprig device
- &lt;NAME&gt;: The name that the game should appear under. Limited to 100
  bytes
- [SOURCE]: Path to the JavaScript source of a Sprig game. If not specified,
  the game is read from stdin

**Options:**

- -h, --help: Print help
- -V, --version: Print version
