use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::process::Command;
use std::result;

use crate::common;
use crate::error;

type Result<T> = result::Result<T, error::KDEConnect>;
type DeviceResult<T> = result::Result<T, error::KDEConnectDevice>;

#[derive(Clone, Debug)]
pub struct Device {
    id: String,
    name: String,
}

impl TryFrom<&str> for Device {
    type Error = error::KDEConnectDevice;

    fn try_from(value: &str) -> result::Result<Self, Self::Error> {
        let mut data = value.split_whitespace().map(ToOwned::to_owned);
        let id: String = data.next().ok_or(error::KDEConnectDevice::ID)?;
        let name: String = data.next().ok_or(error::KDEConnectDevice::Name)?;
        Ok(Self { id, name })
    }
}

#[derive(Debug)]
pub struct Notifier {
    threshold: u8,
    device_names: Option<HashSet<String>>,
}

impl Notifier {
    /// Create a new `Notifier` instance.
    pub fn new(threshold: u8, device_names: HashSet<String>) -> Self {
        Self {
            threshold,
            device_names: if device_names.is_empty() {
                None
            } else {
                Some(device_names)
            },
        }
    }

    /// Ping all saved `Device` instances letting them know that the battery charge threshold has
    /// been reached.
    pub fn ping(&self) -> Result<()> {
        self.find_available()?.iter().try_for_each(|device| {
            ping(device, &common::warning_message(self.threshold))
        })?;

        log::debug!("available KDE Connect devices pinged");

        Ok(())
    }

    fn find_available(&self) -> Result<Vec<Device>> {
        let mut devices = available_devices_map()?;

        Ok(match &self.device_names {
            None => devices.into_values().collect(),
            Some(names) => names
                .iter()
                .filter_map(|name| {
                    common::warn_on_err(find_device(&mut devices, name))
                })
                .collect(),
        })
    }
}

/// Print all available `Device` instances formatted in a nice and readable way.
///
/// Acts as an high level API for the CLI `KDEConnectDevices` subcommand.
pub fn print_devices() -> Result<()> {
    common::print_slice(
        &all_devices_map()?.into_values().collect::<Vec<Device>>(),
    );
    Ok(())
}

fn all_devices_map() -> Result<HashMap<String, Device>> {
    device_map(&list_devices()?)
}

fn available_devices_map() -> Result<HashMap<String, Device>> {
    device_map(&list_available()?)
}

/// Return a mapping between name and the corresponding `Device` instance.
fn device_map(list: &str) -> Result<HashMap<String, Device>> {
    list.lines()
        .map(|line| {
            let device = Device::try_from(line)?;
            Ok((device.name.clone(), device))
        })
        .collect()
}

/// Return `Device` from the given `HashMap` if there is a mapping to it via the given name.
fn find_device(
    devices: &mut HashMap<String, Device>,
    name: &str,
) -> DeviceResult<Device> {
    devices
        .remove(name)
        .ok_or(error::KDEConnectDevice::NotFound { name: name.into() })
}

/// Ping KDE Connect device with the given `id` via the `kdeconnect-cli` command.
fn ping(device: &Device, message: &str) -> Result<()> {
    execute(&["--device", &device.id, "--ping-msg", message])?;
    log::debug!("pinged - {}", &device);
    Ok(())
}

fn list_devices() -> Result<String> {
    execute(&["--list-devices", "--id-name-only"])
}

fn list_available() -> Result<String> {
    execute(&["--list-available", "--id-name-only"])
}

/// Execute `kdeconnect-cli` command with the given arguments and return its output.
fn execute(args: &[&str]) -> Result<String> {
    let output = Command::new("kdeconnect-cli").args(args).output()?;

    log::debug!("kdeconnect-cli: args = {:?}", args);

    let stderr = common::slice_to_string(output.stderr.as_slice());
    if !stderr.is_empty() {
        log::warn!("kdeconnect-cli: stderr = {}", stderr);
    }

    Ok(common::slice_to_string(output.stdout.as_slice()))
}

mod std_fmt_impls {
    use std::borrow::Borrow;
    use std::fmt;

    use super::{Device, Notifier};

    impl fmt::Display for Device {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "KDE Connect device: name = {}, id = {}",
                self.name, self.id,
            )
        }
    }

    impl fmt::Display for Notifier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let device_names = self.device_names.as_ref().map_or_else(
                || "All".into(),
                |names| {
                    format!(
                        "[{}]",
                        names
                            .iter()
                            .map(Borrow::borrow)
                            .collect::<Vec<&str>>()
                            .join(", ")
                    )
                },
            );

            write!(
                f,
                "KDE Connect Notifier: threshold = {}, device_names = {}",
                self.threshold, device_names,
            )
        }
    }
}
