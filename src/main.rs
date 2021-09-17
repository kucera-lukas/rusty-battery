//! CLI tool to help you care about your device's battery health.

#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery
)]
#![allow(clippy::module_name_repetitions, clippy::items_after_statements)]

mod battery;
mod cli;
mod error;
mod event;
mod notification;

fn main() {
    let opts: cli::Opts = cli::parse();

    event::event_loop(opts.threshold).unwrap();
}
