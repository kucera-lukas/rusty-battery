#![doc = include_str ! ("../docs/README.md")]
#![doc(
    html_favicon_url = "https://raw.githubusercontent.com/kucera-lukas/rusty-battery/main/docs/assets/img/favicon.ico",
    html_logo_url = "https://raw.githubusercontent.com/kucera-lukas/rusty-battery/main/docs/assets/img/logo.png"
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

use std::process;

mod cli;
mod common;
mod device;
mod error;
mod event;
mod logger;
mod notification;
mod notify;
mod parser;

fn main() -> ! {
    process::exit(match run_app() {
        Ok(_) => 0,
        Err(e) => error::handle(e),
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
            summary,
            body,
            kde_connect_names,
            disable_desktop,
        } => notify::notify(
            threshold,
            model.as_deref(),
            refresh_secs,
            summary,
            body,
            kde_connect_names,
            disable_desktop,
        )?,
        cli::Command::Batteries => batteries()?,
        cli::Command::KDEConnectDevices => kde_connect_devices()?,
    }

    Ok(())
}

fn batteries() -> error::Result<()> {
    device::Type::Battery.print()
}

fn kde_connect_devices() -> error::Result<()> {
    device::Type::KDEConnect.print()
}
