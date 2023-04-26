# Pomodoro

One day I realized an incredible fact. There are very few Pomodoro CLI tools out
there. They are incredibly rare. No developers ever build pomodoro timers as a
simple project when they're learning a new language. Fortunately I decided to
come to the rescue and made a small tool in Rust to manage my time.

## Requirements

- Rust compiler

## Installation

Clone the repository with
`git clone "https://github.com/realprogrammersusevim/pomodoro"` and run
`cargo build --release` to create your binary. Then move the compiled binary
from `target/release/pomodoro` to wherever you want. I would suggest somewhere
in your `$PATH` so you can run it from anywhere.

**Note:** macOS has security features that prevent a binary from sending system
notifications from outside a secured location. If you're on a Mac you'll need to
put the `pomodoro` binary in either `/usr/local/bin/` or `/Applications/`, with
`/usr/local/bin/` being the preferred location.

## Usage

```help
Yet another Pomodoro timer

Usage: pomodoro [OPTIONS] --work <WORK>

Options:
  -w, --work <WORK>      Work duration
  -c, --chill <CHILL>    Chill duration [default: 0]
  -r, --repeat <REPEAT>  How many times to repeat [default: 1]
  -n, --name <NAME>      Name of the work session
  -a, --alert            Disable notifications
  -t, --time             Use 24hr clock
  -h, --help             Print help information
  -V, --version          Print version information
```

## Showcase

https://user-images.githubusercontent.com/93488695/207990368-c7b53b6e-00d7-4fc3-b530-169a39ef8081.mov
