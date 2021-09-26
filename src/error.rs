use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("something went wrong while fetching battery information: {}", .0)]
    Battery(#[from] BatteryError),
    #[error("something went wrong with user notification: {}", .0)]
    Notification(#[from] NotificationError),
}

#[derive(Error, Debug)]
pub enum BatteryError {
    #[error("battery information failure")]
    SystemError(#[from] battery::Error),
}

#[derive(Error, Debug)]
pub enum NotificationError {
    #[error("something went wrong while showing a desktop notification: {}", .0)]
    Desktop(#[from] notify_rust::error::Error),
}
