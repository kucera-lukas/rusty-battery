//! CLI tool to help you care about your device's battery health.

#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::module_name_repetitions, clippy::items_after_statements)]

mod battery;
mod cli;
mod event;
mod notification;

fn main() {
    let opts: cli::Opts = cli::parse();

    event::event_loop(opts.threshold).unwrap()
}
