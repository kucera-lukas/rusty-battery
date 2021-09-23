use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Initialize `env_logger`.
pub fn init(verbose: u8) {
    let mut builder = create_builder(verbose);
    builder.init();
}

/// Return `log::LevelFilter`.
const fn create_level_filter(verbose: u8) -> LevelFilter {
    match verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2..=u8::MAX => LevelFilter::Debug,
    }
}

/// Return `env_logger::Builder`
pub fn create_builder(verbose: u8) -> Builder {
    let level_filter = create_level_filter(verbose);

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

    log::debug!("env_logger created with RUST_LOG={}", level_filter);

    builder
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(
        expected = "Builder::init should not be called after logger initialized: SetLoggerError(())"
    )]
    fn test_init_logger_initialized() {
        init(0);
        init(0);
    }

    #[test]
    fn test_create_level_filter_error() {
        let filter = create_level_filter(0);

        assert_eq!(filter, LevelFilter::Error);
    }

    #[test]
    fn test_create_level_filter_info() {
        let filter = create_level_filter(1);

        assert_eq!(filter, LevelFilter::Info);
    }

    #[test]
    fn test_create_level_filter_debug() {
        let filter = create_level_filter(2);

        assert_eq!(filter, LevelFilter::Debug);
    }

    fn assert_create_builder(verbose: u8) {
        let builder = create_builder(verbose);
        let level_filter = create_level_filter(verbose);

        assert!(
            format!("{:?}", builder).contains(&format!("{:?}", level_filter))
        );
    }

    #[test]
    fn test_create_builder_error() {
        assert_create_builder(0);
    }

    #[test]
    fn test_create_builder_info() {
        assert_create_builder(1);
    }

    #[test]
    fn test_create_builder_debug() {
        assert_create_builder(2);
    }
}
