#![doc = include_str ! ("../README.md")]
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
use std::sync::mpsc;

mod battery;
mod cli;
mod common;
mod error;
mod event;
mod logger;
mod notification;
mod parser;

fn main() {
    process::exit(match run_app() {
        Ok(_) => 0,
        Err(e) => {
            log::error!("{e}");
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
            disable_desktop,
        } => notify(
            threshold,
            model.as_deref(),
            refresh_secs,
            kde_connect_names,
            disable_desktop,
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
    disable_desktop: bool,
) -> error::Result<()> {
    validate_notify_input(
        threshold,
        model,
        refresh_secs,
        &kde_connect_names,
        disable_desktop,
    )?;

    let mut battery_device = battery::Device::try_from(model)?;
    let mut notifier = notification::Notifier::new(
        threshold,
        kde_connect_names.map(common::vec_to_set),
        disable_desktop,
    )?;

    let (shutdown_sender, shutdown_receiver) = mpsc::channel();

    event::set_handler(shutdown_sender)?;

    event::loop_(
        &shutdown_receiver,
        &mut battery_device,
        &mut notifier,
        refresh_secs,
    )?;

    Ok(())
}

fn validate_notify_input(
    _threshold: u8,
    _model: Option<&str>,
    _refresh_secs: u64,
    kde_connect_names: &Option<Vec<String>>,
    disable_desktop: bool,
) -> error::Result<()> {
    if disable_desktop && kde_connect_names.is_none() {
        return Err(error::Error::from(error::Notification::Config {
            kind: "both desktop and KDE connect notifications are disabled"
                .into(),
        }));
    };

    Ok(())
}

fn batteries() -> error::Result<()> {
    Ok(battery::print_devices()?)
}

fn kde_connect_devices() -> error::Result<()> {
    Ok(notification::kde_connect::print_devices()?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_notify_input_notifications_disabled() {
        let result = notify(0, None, 0, None, true);

        assert!(result.is_err());
        result.unwrap_or_else(|e| {
            assert!(matches!(
                e,
                error::Error::Notification(error::Notification::Config { .. })
            ));
        });
    }
}
