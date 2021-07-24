use structopt::StructOpt;

/// Tool to help you care about your device's battery health
/// by showing a desktop notification whenever battery percentage
/// exceeds a given threshold.
#[derive(StructOpt, Debug)]
#[structopt(name = "rusty-battery")]
pub struct Opts {
    /// Battery charge threshold
    #[structopt(short, long, default_value = "80")]
    pub threshold: u8,
}

pub fn parse() -> Opts {
    Opts::from_args()
}
