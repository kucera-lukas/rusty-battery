pub mod battery;
mod cli;
mod event;
mod notification;

use notify_rust::NotificationHandle;

fn main() {
    let opts: cli::Opts = cli::parse();

    event::event_loop(opts.threshold).unwrap()
}
