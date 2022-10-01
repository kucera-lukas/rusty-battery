//! KDE Connect device
use std::collections::{HashMap, HashSet};
use std::result;

use crate::{common, error};

type Result<T> = result::Result<T, error::KDEConnect>;

/// KDE Connect device representation.
#[derive(Clone, Debug)]
pub struct KDEConnect {
    /// ID of the device
    id: String,
    /// Name of the device
    name: String,
}

impl KDEConnect {
    /// Ping the given `KDEConnect` via the `ping-msg` option.
    pub fn ping(&self, message: &str) -> Result<()> {
        log::trace!("device/kde_connect: pinging {}", self.id);

        execute(&[
            "--device",
            &self.id,
            "--ping-msg",
            // needs to be wrapped in quotes
            // otherwise only the first word of the message would be sent
            &format!("\"{message}\""),
        ])?;

        log::debug!("device/kde_connect: {self} pinged");

        Ok(())
    }
}

impl TryFrom<&str> for KDEConnect {
    type Error = error::KDEConnectDevice;

    fn try_from(value: &str) -> result::Result<Self, Self::Error> {
        let mut data = value.split_whitespace().map(ToOwned::to_owned);

        let id: String = data.next().ok_or(error::KDEConnectDevice::ID)?;
        log::trace!("device/kde_connect: id = {id}");

        let name: String = data.next().ok_or(error::KDEConnectDevice::Name)?;
        log::trace!("device/kde_connect: name = {name}");

        Ok(Self { id, name })
    }
}

pub mod map {
    use super::{list, HashMap, KDEConnect, Result};

    /// Return a mapping between name and its `KDEConnect` instance.
    ///
    /// `Device`s are collected via the `list-devices` KDE Connect CLI option.
    pub fn all() -> Result<HashMap<String, KDEConnect>> {
        parse(&list::all()?)
    }

    /// Return a mapping between name and its `KDEConnect` instance.
    ///
    /// `KDEConnect`s are collected via the
    /// `list-available` KDE Connect CLI option.
    pub fn available() -> Result<HashMap<String, KDEConnect>> {
        parse(&list::available()?)
    }

    /// Return a mapping between name and its `KDEConnect` instance.
    ///
    /// Data is parsed from the given string.
    fn parse(list: &str) -> Result<HashMap<String, KDEConnect>> {
        list.lines()
            .map(|line| {
                let device = KDEConnect::try_from(line)?;

                log::debug!("device/kde_connect: created {device}");

                Ok((device.name.clone(), device))
            })
            .collect()
    }
} // map

pub mod find {
    use super::{common, error, HashMap, HashSet, KDEConnect};

    /// Search the `HashMap` for `KDEConnect`s with name in the `HashSet`.
    pub fn all(
        devices: &mut HashMap<String, KDEConnect>,
        names: &HashSet<String>,
    ) -> Vec<KDEConnect> {
        names
            .iter()
            .filter_map(|name| {
                common::warn_on_err("device/kde_connect", one(devices, name))
            })
            .collect()
    }

    /// Search the `HashMap` for a `KDEConnect` with the name.
    pub fn one(
        devices: &mut HashMap<String, KDEConnect>,
        name: &str,
    ) -> Result<KDEConnect, error::KDEConnectDevice> {
        devices
            .remove(name)
            .ok_or(error::KDEConnectDevice::NotFound { name: name.into() })
    }
} // find

mod list {
    use super::{execute, Result};

    /// Return stdout of the `list-devices` KDE Connect CLI option.
    pub fn all() -> Result<String> {
        log::debug!("device/kde_connect: listing all");

        execute(&["--list-devices", "--id-name-only"])
    }

    /// Return stdout of the `list-available` KDE Connect CLI option.
    pub fn available() -> Result<String> {
        log::debug!("device/kde_connect: listing all available");

        execute(&["--list-available", "--id-name-only"])
    }
} // list

/// Execute KDE Connect CLI command with the given arguments.
///
/// Warn if any data is passed into stderr.
/// Return stdout data.
fn execute(args: &[&str]) -> Result<String> {
    let output =
        common::command(&format!("kdeconnect-cli {}", args.join(" ")))?;

    let stderr = common::slice_to_string(output.stderr.as_slice());
    if !stderr.is_empty() {
        log::warn!("kdeconnect/cli: stderr = {}", &stderr.trim());
    }

    let stdout = common::slice_to_string(output.stdout.as_slice());
    if !stdout.is_empty() {
        log::trace!("kdeconnect/cli: stdout = {}", &stdout.trim());
    }

    Ok(stdout)
}

mod std_fmt_impls {
    use std::fmt;

    use super::KDEConnect;

    impl fmt::Display for KDEConnect {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "KDE Connect Device: name = {}, id = {}",
                self.name, self.id,
            )
        }
    }
} // std_fmt_impls
