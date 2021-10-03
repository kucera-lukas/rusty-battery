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

use std::convert::TryFrom;
use std::process;

mod battery;
mod cli;
mod common;
mod error;
mod event;
mod logger;
mod notification;

fn main() {
    process::exit(match run_app() {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("{}", e);
            1
        }
    })
}

fn run_app() -> error::Result<()> {
    let opts = cli::parse();

    logger::init(opts.verbose);

    match opts.cmd {
        cli::Command::Notify { threshold, model } => {
            notify(threshold, model.as_deref())?;
        }
        cli::Command::Batteries => batteries()?,
    }

    Ok(())
}

fn notify(threshold: u8, model: Option<&str>) -> error::Result<()> {
    let mut battery_device =
        battery::BatteryDevice::try_from(model.as_deref())?;
    let mut notifier = notification::Notifier::try_from(threshold)?;

    event::event_loop(threshold, &mut battery_device, &mut notifier)?;

    Ok(())
}

fn batteries() -> error::Result<()> {
    Ok(battery::print_devices()?)
}
