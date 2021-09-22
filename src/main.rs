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

    let battery_provider = battery::BatteryDataProvider::new()
        .unwrap_or_else(|e| panic!("{}", e));

    let desktop_notifier =
        notification::desktop::DesktopNotificationProvider::new();

    let mut app = application::App::new(
        opts.verbose,
        opts.threshold,
        battery_provider,
        desktop_notifier,
    )
    .unwrap_or_else(|e| panic!("{}", e));

    event::event_loop(&mut app).unwrap();
}
