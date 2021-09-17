use thiserror::Error;

use battery::State;

#[derive(Error, Debug)]
pub enum BatteryError {
    #[error("could not find any battery device")]
    DeviceError,
    #[error("battery information failure")]
    SystemError(#[from] battery::Error),
    #[error("unknown battery state: {state}")]
    UnknownState { state: State },
}

#[derive(Error, Debug)]
pub enum EventError {
    #[error("something went wrong while getting battery information: {.0}")]
    Battery(#[from] BatteryError),
    #[error(
        "something went wrong while showing a desktop notification: {.0}"
    )]
    Notification(#[from] notify_rust::error::Error),
}
