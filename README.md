# Pomodoro

One day I realized an incredible fact. There are very few Pomodoro CLI tools out
there. They are incredibly rare. Fortunately I decided to come to the rescue and
made a small tool in Rust to manage my time.

## Requirements

- Rust compiler

## Installation

Clone the repository with
`git clone "https://github.com/realprogrammersusevim/pomodoro"` and run
`cargo build --release` to create your binary. Then move the compiled binary
from `target/release/pomodoro` to anywhere you want. I would suggest somewhere
in your `$PATH` so you can run it from anywhere.

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
