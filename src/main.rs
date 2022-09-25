#![doc = include_str!("../README.md")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/kucera-lukas/rusty-battery/main/assets/img/favicon.ico",
    html_logo_url = "https://raw.githubusercontent.com/kucera-lukas/rusty-battery/main/assets/img/logo.png"
)]
#![warn(
    clippy::correctness,
    clippy::suspicious,
    clippy::style,
    clippy::complexity,
    clippy::perf,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
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

    logger::init(&opts.verbose);

    match opts.cmd {
        cli::Command::Notify {
            threshold,
            model,
            refresh_secs,
            kde_connect_names,
        } => notify(
            threshold,
            model.as_deref(),
            refresh_secs,
            kde_connect_names,
        )?,
        cli::Command::Batteries => batteries()?,
        cli::Command::KDEConnectDevices => kde_connect_devices()?,
    }

    Ok(())
}

fn notify(
    threshold: u8,
    model: Option<&str>,
    refresh_secs: u64,
    kde_connect_names: Option<Vec<String>>,
) -> error::Result<()> {
    let mut battery_device = battery::Device::try_from(model)?;
    let mut notifier = notification::Notifier::new(
        threshold,
        kde_connect_names.map(common::vec_to_set),
    );

    event::loop_(&mut battery_device, &mut notifier, refresh_secs)?;

    Ok(())
}

fn batteries() -> error::Result<()> {
    Ok(battery::print_devices()?)
}

fn kde_connect_devices() -> error::Result<()> {
    Ok(notification::kde_connect::print_devices()?)
}
