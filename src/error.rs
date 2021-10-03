use std::io;
use std::result;

use thiserror::Error;

pub type Result<T> = result::Result<T, Error>;

pub type BatteryResult<T> = result::Result<T, BatteryError>;
pub type BatteryDeviceResult<T> = result::Result<T, BatteryDeviceError>;

pub type NotificationResult<T> = result::Result<T, NotificationError>;

pub type KDEConnectResult<T> = result::Result<T, KDEConnectError>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something went wrong with battery information: {}", .0)]
    Battery(#[from] BatteryError),
    #[error("something went wrong with KDE Connect: {}", .0)]
    KDEConnect(#[from] KDEConnectError),
    #[error("something went wrong with notification: {}", .0)]
    Notification(#[from] NotificationError),
}

#[derive(Debug)]
pub struct Model(pub Option<String>);

#[derive(Error, Debug)]
pub enum BatteryError {
    #[error("battery information failure: {}", .0)]
    System(#[from] battery::Error),
    #[error("battery device not found: model = {model}")]
    NotFound { model: Model },
    #[error("battery device error: {}", .0)]
    Device(#[from] BatteryDeviceError),
}

#[derive(Error, Debug)]
pub enum BatteryDeviceError {
    #[error("failed to retrieve battery model")]
    Model,
    #[error("failed to retrieve battery serial number")]
    SerialNumber,
}

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("something went wrong with desktop notification: {}", .0)]
    Desktop(#[from] notify_rust::error::Error),
}

#[derive(Error, Debug)]
pub enum KDEConnectError {
    #[error("kdeconnect-cli is not installed on this system: {}", .0)]
    Cli(#[from] io::Error),
    #[error("KDE Connect device error: {}", .0)]
    Device(#[from] KDEConnectDeviceError),
}

#[derive(Error, Debug)]
pub enum KDEConnectDeviceError {
    #[error("failed to retrieve device id")]
    ID,
    #[error("failed to retrieve device name")]
    Name,
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
