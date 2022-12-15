use chrono;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use notify_rust::Notification;
use std::thread;
use std::time::Duration;

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

    // Flag to disable notifications
    #[arg(short, long, help = "Disable notifications")]
    alert: bool,

    // Format time with 24hr clock
    #[arg(short, long, help = "Use 24hr clock")]
    time: bool,
}

fn main() {
    let args = Args::parse();

    for i in 0..args.repeat {
        if let Some(name) = &args.name {
            println!("Work session: {}", name);
        } else {
            println!("Work session");
        }

        let work_pb = ProgressBar::new(args.work * 60);
        let work_end = chrono::Local::now() + chrono::Duration::minutes(args.work as i64);
        let formatted_work_end = match args.time {
            false => work_end.format("%I:%M %p").to_string(),
            true => work_end.format("%H:%M").to_string(),
        };
        work_pb.set_style(ProgressStyle::with_template("{bar:60.green} {msg}").unwrap());
        for _ in 0..args.work * 60 {
            work_pb.inc(1);
            work_pb.set_message(format!(
                "{}m {}s - {}",
                (args.work * 60 - work_pb.position()) / 60,
                (60 - work_pb.position() % 60),
                formatted_work_end
            ));
            thread::sleep(Duration::from_secs(1));

            if args.alert {
                Notification::new()
                    .summary("Pomodoro")
                    .body("Work session is over!")
                    .show()
                    .unwrap();
            }
        }

        // Check if chill is enabled and this isn't the last session
        if args.chill != 0 && i < args.repeat - 1 {
            println!();

            let chill_pb = ProgressBar::new(args.chill * 60);
            let chill_end = chrono::Local::now() + chrono::Duration::minutes(args.chill as i64);
            let formatted_chill_end = match args.time {
                false => chill_end.format("%I:%M %p").to_string(),
                true => chill_end.format("%H:%M").to_string(),
            };
            chill_pb.set_style(ProgressStyle::with_template("{bar:60.blue} {msg}").unwrap());
            for _ in 0..args.chill * 60 {
                chill_pb.inc(1);
                chill_pb.set_message(format!(
                    "{}m {}s - {}",
                    (args.chill * 60 - chill_pb.position()) / 60,
                    (60 - chill_pb.position() % 60),
                    formatted_chill_end
                ));
                thread::sleep(Duration::from_secs(1));
            }

            if args.alert {
                Notification::new()
                    .summary("Chill over")
                    .body("The chill session is over, back to work")
                    .show()
                    .unwrap();
            }
        }
    }
}
