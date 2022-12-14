use clap::Parser;
use indicatif::ProgressBar;
use std::thread;
use std::time;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    // How long the work time should be
    #[arg(short, long)]
    work: u64,

    // How long the break time should be
    #[arg(short, long)]
    chill: u64,

    // How many times to repeat the cycle
    #[arg(short, long, default_value = "1")]
    repeat: u64,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.repeat {
        println!("Work time for {} minutes!", args.work);
        let work_pb = ProgressBar::new(args.work * 60);
        for _ in 0..args.work * 60 {
            work_pb.inc(1);
            thread::sleep(time::Duration::from_secs(1));
        }

        println!("Chill time for {} minutes!", args.chill);
        let chill_pb = ProgressBar::new(args.chill * 60);
        for _ in 0..args.chill * 60 {
            chill_pb.inc(1);
            thread::sleep(time::Duration::from_secs(1));
        }
    }
}
