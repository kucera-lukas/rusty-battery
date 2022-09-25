use structopt::StructOpt;

/// Tool to help you care about your device's battery health.
#[derive(StructOpt, Debug)]
#[structopt(name = "rusty-battery")]
pub struct Opts {
    /// Activates verbose mode
    #[structopt(short, long, parse(from_occurrences), global = true)]
    pub verbose: u8,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug, PartialEq, Eq)]
pub enum Command {
    /// Notify whenever battery percentage exceeds the given threshold.
    Notify {
        /// Battery charge threshold
        ///
        /// Whenever the chosen battery device reaches this charge threshold and will be
        /// charging, notifications will be sent, alerting that the charger should be unplugged.
        #[structopt(short, long, default_value = "80")]
        threshold: u8,

        /// Battery model name
        ///
        /// If this value is omitted and only battery device is found for the current device,
        /// that one will be used.
        ///
        /// Otherwise, please use the `batteries` subcommand to get a list of all battery devices
        /// to get the model of the wanted battery device which should be monitored.
        #[structopt(short, long)]
        model: Option<String>,

        /// Number of seconds to wait before refreshing battery device data
        ///
        /// After every battery device refresh, its data will be checked. Notifications will be
        /// sent everytime they should be, based on the new refreshed battery device data.
        #[structopt(long, default_value = "30")]
        refresh_secs: u64,

        /// KDE Connect device names
        ///
        /// If this value is not present, KDE Connect will not be used.
        ///
        /// If this value is empty, all of the KDE Connect devices will be pinged.
        #[structopt(long = "kde-connect")]
        kde_connect_names: Option<Vec<String>>,
    },
    /// List all available batteries of the current device.
    Batteries,
    /// List all available KDE Connect devices.
    KDEConnectDevices,
}

pub fn parse() -> Opts {
    Opts::from_args()
}
