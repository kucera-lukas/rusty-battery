use clap::{Parser, Subcommand};

use crate::parser;

/// Tool to help you care about your device's battery health.
#[derive(Parser, Debug)]
#[command(author, version, about)]
#[command(propagate_version = true)]
pub struct Cli {
    /// Control log level with `--verbose` and `--quiet` flags.
    #[clap(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug, PartialEq, Eq)]
pub enum Command {
    /// Notify whenever battery percentage exceeds the given threshold.
    Notify {
        /// Battery charge threshold
        ///
        /// Whenever the chosen battery device reaches this charge threshold
        /// and will be charging, notifications will be sent, alerting that
        /// the charger should be unplugged.
        ///
        /// [minimum: 0] [maximum: 100]
        #[arg(
        short,
        long,
        value_parser = parser::threshold,
        default_value_t = 80
        )]
        threshold: u8,

        /// Battery model name
        ///
        /// If this value is omitted and only battery device is found
        /// for the current device, that one will be used.
        ///
        /// Otherwise, please use the `batteries` subcommand
        /// to get a list of all battery devices to get the model of the
        /// wanted battery device which should be monitored.
        #[arg(short, long)]
        model: Option<String>,

        /// Number of seconds to wait before refreshing battery device data
        ///
        /// After every battery device refresh, its data will be checked.
        /// Notifications will be sent everytime they should be, based on the
        /// new refreshed battery device data.
        #[clap(long, default_value_t = 30)]
        refresh_secs: u64,

        /// KDE Connect device names
        ///
        /// If this value is not present, KDE Connect will not be used.
        ///
        /// If this value is empty,
        /// all of the KDE Connect devices will be pinged.
        #[arg(long = "kde-connect", num_args = 0..=255)]
        kde_connect_names: Option<Vec<String>>,

        /// Disable desktop notifications
        ///
        /// Specify this flag if you don't want desktop notifications
        /// to be shown whenever the chosen battery percentage exceeds the
        /// given threshold.
        #[arg(long)]
        disable_desktop: bool,
    },
    /// List all available batteries of the current device.
    Batteries,
    /// List all available KDE Connect devices.
    KDEConnectDevices,
}

pub fn parse() -> Cli {
    Cli::parse()
}
