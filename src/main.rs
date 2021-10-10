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
#![allow(clippy::needless_for_each)]

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
        cli::Command::Notify {
            threshold,
            model,
            kde_connect_names,
        } => notify(threshold, model.as_deref(), kde_connect_names)?,
        cli::Command::Batteries => batteries()?,
        cli::Command::KDEConnectDevices => kde_connect_devices()?,
    }

    Ok(())
}

fn notify(
    threshold: u8,
    model: Option<&str>,
    kde_connect_names: Option<Vec<String>>,
) -> error::Result<()> {
    let mut battery_device = battery::Device::try_from(model.as_deref())?;

    let mut notifier = notification::Notifier::new(
        threshold,
        kde_connect_names.map(common::vec_to_hashset),
    )?;

    event::loop_(threshold, &mut battery_device, &mut notifier)?;

    Ok(())
}

fn batteries() -> error::Result<()> {
    Ok(battery::print_devices()?)
}

fn kde_connect_devices() -> error::Result<()> {
    Ok(notification::kde_connect::print_devices()?)
}
