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
mod common;
mod error;
mod event;
mod logger;
mod notification;

fn main() {
    let opts = cli::parse();

    logger::init(opts.verbose);

    match opts.cmd {
        cli::Command::Notify { threshold, model } => {
            let mut app = match application::App::new(
                opts.verbose,
                threshold,
                model.as_deref(),
            ) {
                Ok(app) => app,
                Err(e) => {
                    eprintln!("{}", e);
                    return;
                }
            };

            match event::event_loop(&mut app) {
                Ok(_) => {}
                Err(e) => eprintln!("{}", e),
            };
        }
        cli::Command::Batteries => match battery::print_devices() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
            }
        },
    }
}
