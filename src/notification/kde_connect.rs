use std::collections::{HashMap, HashSet};
use std::process::{Command, Output};
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

#[derive(Debug)]
pub struct Notifier {
    threshold: u8,
    devices: Vec<Device>,
}

impl Notifier {
    /// Create a new `Notifier` instance.
    pub fn new(threshold: u8, device_names: &HashSet<String>) -> Result<Self> {
        log::debug!("creating KDE Connect notifier...");

        let mut devices = device_map()?;

        let result = Self {
            threshold,
            devices: if device_names.is_empty() {
                devices.into_values().collect()
            } else {
                device_names
                    .iter()
                    .map(|name| find_device(&mut devices, name))
                    .collect::<DeviceResult<Vec<Device>>>()?
            },
        };

        log::info!("{}", result);

        Ok(result)
    }

    /// Ping all saved `Device` instances letting them know that the battery charge threshold has
    /// been reached.
    pub fn ping(&self) -> Result<()> {
        self.devices.iter().try_for_each(|device| {
            ping(&device.id, &common::warning_message(self.threshold))
        })?;

        log::debug!("KDE Connect devices pinged");

        Ok(())
    }
}

/// Print all available `Device` instances formatted in a nice and readable way.
///
/// Acts as an high level API for the CLI `KDEConnectDevices` subcommand.
pub fn print_devices() -> Result<()> {
    common::print_slice(&device_map()?.into_values().collect::<Vec<Device>>());
    Ok(())
}

/// Return a mapping between name and the corresponding `Device` instance.
fn device_map() -> Result<HashMap<String, Device>> {
    String::from_utf8_lossy(
        execute(&["--list-devices", "--id-name-only"])?
            .stdout
            .as_slice(),
    )
    .lines()
    .map(|line| {
        let mut data = line.split_whitespace().map(ToOwned::to_owned);
        let id: String = data.next().ok_or(error::KDEConnectDevice::ID)?;
        let name: String = data.next().ok_or(error::KDEConnectDevice::Name)?;
        Ok((name.clone(), Device { id, name }))
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
fn ping(id: &str, message: &str) -> Result<()> {
    execute(&["--device", id, "--ping-msg", message])?;
    log::debug!("pinged {}", id);
    Ok(())
}

/// Execute `kdeconnect-cli` command with the given arguments and return its output.
fn execute(args: &[&str]) -> Result<Output> {
    let output = Command::new("kdeconnect-cli").args(args).output()?;

    log::debug!("kdeconnect-cli: args = {:?}, output = {:?}", args, &output);

    Ok(output)
}

mod std_fmt_impls {
    use std::fmt;

    use super::{Device, Notifier};

    impl fmt::Display for Device {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "KDE Connect device: name = {}, id = {}",
                self.name, self.id
            )
        }
    }

    impl fmt::Display for Notifier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let devices = self
                .devices
                .iter()
                .map(|device| format!("{}", device))
                .collect::<Vec<String>>()
                .join(" ");

            write!(
                f,
                "KDE Connect Notifier: threshold = {}, devices = [{}]",
                self.threshold, devices
            )
        }
    }
}
