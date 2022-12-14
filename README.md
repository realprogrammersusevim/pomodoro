# Pomodoro

One day I realized an incredible fact. There are very few Pomodoro CLI tools out
there. They are incredibly rare. Fortunately I decided to come to the rescue and
made a small tool in Rust to manage my time.

## Installation

Clone the repository and run `cargo build --release` to create your binary. Then
move the binary from `target/release/pomodoro` to anywhere you want. I would
suggest somewhere in your `$PATH` so you can run it from anywhere.

## Usage

```help
Usage: pomodoro [OPTIONS] --work <WORK> --chill <CHILL>

Options:
  -w, --work <WORK>
  -c, --chill <CHILL>
  -r, --repeat <REPEAT>  [default: 1]
  -h, --help             Print help information
  -V, --version          Print version information
```
