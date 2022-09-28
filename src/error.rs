use std::io;
use std::result;
use std::sync::mpsc;

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something went wrong with battery information: {}", .0)]
    Battery(#[from] Battery),
    #[error("something went wrong with KDE Connect: {}", .0)]
    KDEConnect(#[from] KDEConnect),
    #[error("something went wrong with notification: {}", .0)]
    Notification(#[from] Notification),
    #[error("something went wrong with the system: {}", .0)]
    System(#[from] System),
}

#[derive(Debug)]
pub struct Model(pub Option<String>);

#[derive(Error, Debug)]
pub enum Battery {
    #[error("battery routine failure: {}", .0)]
    Routine(#[from] battery::Error),
    #[error("battery device not found: model = \"{model}\"")]
    NotFound { model: Model },
    #[error("battery device error: {}", .0)]
    Device(#[from] BatteryDevice),
}

#[derive(Error, Debug)]
pub enum BatteryDevice {
    #[error("failed to retrieve battery model")]
    Model,
    #[error("failed to retrieve battery serial number")]
    SerialNumber,
}

#[derive(Error, Debug)]
pub enum Notification {
    #[error("configuration failure: {kind}")]
    Config { kind: String },
    #[error("something went wrong with desktop notification: {}", .0)]
    Desktop(#[from] notify_rust::error::Error),
}

#[derive(Error, Debug)]
pub enum KDEConnect {
    #[error("kdeconnect-cli is not installed on this system: {}", .0)]
    Cli(#[from] io::Error),
    #[error("KDE Connect device error: {}", .0)]
    Device(#[from] KDEConnectDevice),
}

#[derive(Error, Debug)]
pub enum KDEConnectDevice {
    #[error("KDE Connect device not found: name = {name}")]
    NotFound { name: String },
    #[error("failed to retrieve device id")]
    ID,
    #[error("failed to retrieve device name")]
    Name,
}

#[derive(Error, Debug)]
pub enum System {
    #[error("signal handler failure: {}", .0)]
    Handler(#[from] ctrlc::Error),
    #[error("receive timeout error: {}", .0)]
    RecvTimeout(#[from] mpsc::RecvTimeoutError),
}

mod std_fmt_impls {
    use std::fmt;

    use super::Model;

    impl fmt::Display for Model {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.0 {
                None => write!(f, "None"),
                Some(model) => write!(f, "{}", model),
            }
        }
    }
}
