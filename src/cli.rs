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

    /// Battery model name
    #[structopt(short, long)]
    pub model: Option<String>,
}

pub fn parse() -> Opts {
    Opts::from_args()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let opts = parse();

        assert_eq!(opts.verbose, 0);
        assert_eq!(opts.threshold, 80);
        assert_eq!(opts.model, None);
    }
}
