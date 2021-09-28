use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something went wrong with battery information: {}", .0)]
    Battery(#[from] BatteryError),
    #[error("something went wrong with user notification: {}", .0)]
    Notification(#[from] NotificationError),
}

#[derive(Debug)]
pub struct Model(pub Option<String>);

#[derive(Error, Debug)]
pub enum BatteryError {
    #[error("battery information failure")]
    System(#[from] battery::Error),
    #[error("battery device not found: model = {model}")]
    NotFound { model: Model },
    #[error("battery device error: {}", .0)]
    Device(#[from] DeviceError),
}

#[derive(Error, Debug)]
pub enum DeviceError {
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
