# RustyPiano 🎹

RustyPiano is a simple terminal piano written in Rust. Press keys `a`, `s`, `d`, `f`, `g`, `h`, `j` to play the notes `sa re ga ...`. It's a fun little way to experiment with audio in Rust.

Built with ❤️ in Rust.

## Features

- Plays simple `sa re ga` notes using keyboard input
- Lightweight and terminal-based
- Powered by [`rodio`](https://crates.io/crates/rodio) for audio output
- GPLv3 licensed, open-source

## Installation

Add RustyPiano to your project via Cargo:

```bash
cargo install RustyPiano
RustyPiano
```

or clone and run directly

```bash
git clone https://github.com/yourusername/RustyPiano.git
cd RustyPiano
cargo run
```

## Notes
| Key | Note Frequency |
|-----|----------------|
| a   | 240 Hz         |
| s   | 270 Hz         |
| d   | 300 Hz         |
| f   | 320 Hz         |
| g   | 360 Hz         |
| h   | 400 Hz         |
| j   | 450 Hz         |
| k   | 480 Hz         |
| q   | Quit           |

## Example
RustyPiano - Press keys a,s,d,f,g,h,j to play notes, q to quit
> a
> s
> d
> q

## License

This project is licensed under the GNU GPL v3 - see the [LICENSE](RustyPiano/LICENSE) for details.

### Built with ♥️ in Rust
