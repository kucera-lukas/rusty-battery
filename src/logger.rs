use chrono::Local;
use env_logger::Builder;
use log::LevelFilter;
use std::io::Write;

/// Initialize `env_logger`
pub fn init(verbose: u8) {
    let level = match verbose {
        0 => LevelFilter::Error,
        1 => LevelFilter::Info,
        2..=u8::MAX => LevelFilter::Debug,
    };

    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args(),
            )
        })
        .filter(None, level)
        .init();

    log::debug!("env_logger initialized with RUST_LOG={}", level);
}
