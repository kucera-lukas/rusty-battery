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
#![allow(clippy::module_name_repetitions)]

mod application;
mod battery;
mod cli;
mod error;
mod event;
mod logger;
mod notification;

fn main() {
    let opts = cli::parse();

    logger::init(opts.verbose);

    let mut app = application::App::from(opts);

    event::event_loop(&mut app);
}
