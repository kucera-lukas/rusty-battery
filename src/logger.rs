use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;

/// Initialize `env_logger`.
pub fn init(verbose: &clap_verbosity_flag::Verbosity) {
    let level_filter = verbose.log_level_filter();

    create_builder(level_filter).init();

    log::debug!("logger: initialized with RUST_LOG={level_filter}");
}

/// Return `env_logger::Builder`
fn create_builder(level_filter: LevelFilter) -> Builder {
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

    builder
}

#[cfg(test)]
mod tests {
    use super::*;

    fn init_log() {
        let _init = env_logger::builder().is_test(true).try_init();
    }

    #[test]
    #[should_panic(expected = "Builder::init should not be called \
        after logger initialized: SetLoggerError(())")]
    fn test_init_logger_initialized() {
        init(&clap_verbosity_flag::Verbosity::new(0, 0));
        init(&clap_verbosity_flag::Verbosity::new(0, 0));
    }

    #[test]
    fn test_log() {
        init_log();

        log::error!("test-error");
    }

    fn assert_create_builder(level_filter: LevelFilter) {
        let builder = create_builder(level_filter);

        assert!(format!("{builder:?}").contains(&format!("{level_filter:?}")));
    }

    #[test]
    fn test_create_builder_off() {
        assert_create_builder(LevelFilter::Off);
    }

    #[test]
    fn test_create_builder_error() {
        assert_create_builder(LevelFilter::Error);
    }

    #[test]
    fn test_create_builder_warn() {
        assert_create_builder(LevelFilter::Warn);
    }

    #[test]
    fn test_create_builder_info() {
        assert_create_builder(LevelFilter::Info);
    }

    #[test]
    fn test_create_builder_debug() {
        assert_create_builder(LevelFilter::Debug);
    }

    #[test]
    fn test_create_builder_trace() {
        assert_create_builder(LevelFilter::Trace);
    }
}
