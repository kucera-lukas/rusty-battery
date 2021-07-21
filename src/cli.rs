use clap::{AppSettings, Clap};

/// Tool to help you care about your device's battery health
/// by showing a desktop notification whenever battery percentage
/// exceeds a given threshold.
#[derive(Clap)]
#[clap(version = "0.1.0", author = "Lukas Kucera <lukas.kucera.g@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    /// Battery charge threshold
    #[clap(short, long)]
    pub threshold: i32,
}

pub fn parse() -> Opts {
    Opts::parse()
}
