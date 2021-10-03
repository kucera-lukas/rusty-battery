use structopt::StructOpt;

/// Tool to help you care about your device's battery health.
#[derive(StructOpt, Debug)]
#[structopt(name = "rusty-battery")]
pub struct Opts {
    /// Activates verbose mode
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    #[structopt(subcommand)]
    pub cmd: Command,
}

#[derive(StructOpt, Debug, PartialEq)]
pub enum Command {
    /// Notify whenever battery percentage exceeds the given threshold.
    Notify {
        /// Battery charge threshold
        #[structopt(short, long, default_value = "80")]
        threshold: u8,

        /// Battery model name
        #[structopt(short, long)]
        model: Option<String>,
    },
    /// List all available batteries of the current device.
    Batteries,
    /// List all available KDE Connect devices.
    KDEConnectDevices,
}

pub fn parse() -> Opts {
    Opts::from_args()
}
