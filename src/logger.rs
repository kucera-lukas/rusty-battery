use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Initialize `env_logger`.
pub fn init(verbose: u8) {
    let (mut builder, level_filter) = create_builder(verbose);
    builder.init();
    log::debug!("env_logger initialized with RUST_LOG={}", level_filter);
}

/// Create `env_logger::Builder` and `log::LevelFilter`.
pub fn create_builder(verbose: u8) -> (Builder, LevelFilter) {
    let level_filter = match verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2..=u8::MAX => LevelFilter::Debug,
    };

    let mut builder = Builder::new();

    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .filter(None, level_filter);

    log::debug!("env_logger initialized with RUST_LOG={}", level_filter);

    (builder, level_filter)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn builder_filter(verbose: u8) -> LevelFilter {
        create_builder(verbose).1
    }

    #[test]
    fn test_logger_filter_error() {
        let filter = builder_filter(0);

        assert_eq!(filter, LevelFilter::Error);
    }

    #[test]
    fn test_logger_filter_info() {
        let filter = builder_filter(1);

        assert_eq!(filter, LevelFilter::Info);
    }

    #[test]
    fn test_logger_filter_debug() {
        let filter = builder_filter(2);

        assert_eq!(filter, LevelFilter::Debug);
    }
}
