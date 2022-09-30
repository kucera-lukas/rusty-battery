use std::fmt::Debug;
use std::io;
use std::result;
use std::sync::mpsc;

use clap::error::ErrorKind;
use clap::CommandFactory;
use thiserror::Error;

use crate::cli;

pub type Result<T> = result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Battery: {}", .0)]
    Battery(#[from] Battery),
    #[error("KDE Connect: {}", .0)]
    KDEConnect(#[from] KDEConnect),
    #[error("Notification: {}", .0)]
    Notification(#[from] Notification),
    #[error("System: {}", .0)]
    System(#[from] System),
}

#[derive(Debug)]
pub struct Model(pub Option<String>);

#[derive(Error, Debug)]
pub enum Battery {
    #[error("routine failure: {}", .0)]
    Routine(#[from] battery::Error),
    #[error("device not found: model = \"{model}\"")]
    NotFound { model: Model },
    #[error("device: {}", .0)]
    Device(#[from] BatteryDevice),
}

#[derive(Error, Debug)]
pub enum BatteryDevice {
    #[error("failed to retrieve model")]
    Model,
    #[error("failed to retrieve serial number")]
    SerialNumber,
}

#[derive(Error, Debug)]
pub enum Notification {
    #[error("configuration failure: {kind}")]
    Config { kind: String },
    #[error("desktop: {}", .0)]
    Desktop(#[from] notify_rust::error::Error),
}

#[derive(Error, Debug)]
pub enum KDEConnect {
    #[error("CLI is not installed: {}", .0)]
    Cli(#[from] io::Error),
    #[error("device: {}", .0)]
    Device(#[from] KDEConnectDevice),
}

#[derive(Error, Debug)]
pub enum KDEConnectDevice {
    #[error("not found: name = {name}")]
    NotFound { name: String },
    #[error("failed to retrieve id")]
    ID,
    #[error("failed to retrieve name")]
    Name,
}

#[derive(Error, Debug)]
pub enum System {
    #[error("signal handler: {}", .0)]
    Handler(#[from] ctrlc::Error),
    #[error("receive timeout: {}", .0)]
    RecvTimeout(#[from] mpsc::RecvTimeoutError),
}

pub fn handle(e: Error) -> ! {
    let mut cmd = cli::Cli::command();

    cmd.error(ErrorKind::Io, e).exit()
}

mod std_fmt_impls {
    use std::fmt;

    use super::Model;

    impl fmt::Display for Model {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match &self.0 {
                None => write!(f, "None"),
                Some(model) => write!(f, "{model}"),
            }
        }
    }
}
