use clap::Parser;
use indicatif::ProgressBar;
use std::thread;
use std::time;

#[derive(Parser)]
#[command(
    author = "Jonathan Milligan",
    version,
    about = "Yet another Pomodoro timer"
)]
struct Args {
    // How long the work time should be
    #[arg(short, long, help = "Work duration")]
    work: u64,

    // How long the break time should be
    #[arg(short, long, help = "Chill duration", default_value = "0")]
    chill: u64,

    // How many times to repeat the cycle
    #[arg(short, long, default_value = "1", help = "How many times to repeat")]
    repeat: u64,

    // Optionally name the work session
    #[arg(short, long, help = "Name of the work session")]
    name: Option<String>,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.repeat {
        let work_pb = ProgressBar::new(args.work * 60);

        if let Some(name) = &args.name {
            println!("Work session: {}", name);
        } else {
            println!("Work session");
        }
        for _ in 0..args.work * 60 {
            work_pb.inc(1);
            thread::sleep(time::Duration::from_secs(1));
        }

        let chill_pb = ProgressBar::new(args.chill * 60);
        for _ in 0..args.chill * 60 {
            chill_pb.inc(1);
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
