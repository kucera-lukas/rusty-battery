use structopt::StructOpt;

/// Tool to help you care about your device's battery health
/// by showing a desktop notification whenever battery percentage
/// exceeds a given threshold.
#[derive(StructOpt, Debug)]
#[structopt(name = "rusty-battery")]
pub struct Opts {
    /// Activates verbose mode
    #[structopt(short, long, parse(from_occurrences))]
    pub verbose: u8,

    /// Battery charge threshold
    #[structopt(short, long, default_value = "80")]
    pub threshold: u8,
}

pub fn parse() -> Opts {
    Opts::from_args()
}
