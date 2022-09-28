use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::result;

use crate::common;
use crate::error;

type Result<T> = result::Result<T, error::KDEConnect>;
type DeviceResult<T> = result::Result<T, error::KDEConnectDevice>;

/// KDE Connect device representation.
#[derive(Clone, Debug)]
pub struct Device {
    /// ID of the device
    id: String,
    /// Name of the device
    name: String,
}

impl TryFrom<&str> for Device {
    type Error = error::KDEConnectDevice;

    fn try_from(value: &str) -> result::Result<Self, Self::Error> {
        let mut data = value.split_whitespace().map(ToOwned::to_owned);

        let id: String = data.next().ok_or(error::KDEConnectDevice::ID)?;
        log::trace!("notification/kde_connect: device id = {}", id);

        let name: String = data.next().ok_or(error::KDEConnectDevice::Name)?;
        log::trace!("notification/kde_connect: device name = {}", name);

        Ok(Self { id, name })
    }
}

/// KDE Connect notifier.
#[derive(Debug)]
pub struct Notifier {
    /// Charge threshold used to create warning message.
    threshold: u8,
    /// Names of KDE Connect devices which should be pinged.
    ///
    /// If this value is `None` every available KDE Connect device will pinged.
    device_names: Option<HashSet<String>>,
}

impl Notifier {
    /// Create a new `Notifier` instance.
    pub fn new(threshold: u8, device_names: HashSet<String>) -> Result<Self> {
        let notifier = Self {
            threshold,
            device_names: if device_names.is_empty() {
                log::info!(
                    "notification/kde_connect: no device names specified, \
                    all available devices will be pinged",
                );

                None
            } else {
                log::info!(
                    "notification/kde_connect: will ping devices with names {}",
                    common::format_string_set(&device_names),
                );

                Some(device_names)
            },
        };

        // check kdeconnect-cli status and also warn if some specified devices aren't available
        notifier.find_available()?;

        Ok(notifier)
    }

    /// Ping all available `Device` instances.
    pub fn ping(&self) -> Result<()> {
        self.find_available()?.iter().try_for_each(|device| {
            log::trace!(
                "notification/kde_connect: pinging device {}",
                device.id
            );

            ping(device, &common::warning_message(self.threshold))
        })?;

        log::debug!("notification/kde_connect: available devices pinged");

        Ok(())
    }

    /// Return all `Device` instances which are currently available.
    ///
    /// If no `device_names` were specified at the creation, all available devices will be returned.
    fn find_available(&self) -> Result<Vec<Device>> {
        let mut devices = available_devices_map()?;

        Ok(match &self.device_names {
            None => devices.into_values().collect(),
            Some(names) => find_devices(&mut devices, names),
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

/// Return a mapping between name and its corresponding `Device` instance.
///
/// `Devices` are collected from the `list-devices` `kde-connect-cli` argument.
fn all_devices_map() -> Result<HashMap<String, Device>> {
    device_map(&list_devices()?)
}

/// Return a mapping between name and its corresponding `Device` instance.
///
/// `Devices` are collected from the `list-available` `kde-connect-cli` argument.
fn available_devices_map() -> Result<HashMap<String, Device>> {
    device_map(&list_available()?)
}

/// Return a mapping between name and the corresponding `Device` instance.
///
/// Data is parsed from the given string.
fn device_map(list: &str) -> Result<HashMap<String, Device>> {
    list.lines()
        .map(|line| {
            let device = Device::try_from(line)?;

            log::debug!("notification/kde_connect: created device {}", device,);

            Ok((device.name.clone(), device))
        })
        .collect()
}

/// Search the given `HashMap` of devices for devices with a name present in the given `HashSet`.
fn find_devices(
    devices: &mut HashMap<String, Device>,
    names: &HashSet<String>,
) -> Vec<Device> {
    names
        .iter()
        .filter_map(|name| {
            common::warn_on_err(
                "notification/kde_connect",
                find_device(devices, name),
            )
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

/// Ping the given `Device` via the `kdeconnect-cli` `ping-msg` command.
fn ping(device: &Device, message: &str) -> Result<()> {
    execute(&[
        "--device",
        &device.id,
        "--ping-msg",
        // needs to be wrapped in quotes otherwise only the first word of the message  would be sent
        &format!("\"{}\"", message),
    ])?;

    log::debug!("notification/kde_connect: {} pinged", &device);

    Ok(())
}

/// Return `kdeconnect-cli` stdout listing all devices via the `list-devices` argument.
fn list_devices() -> Result<String> {
    log::debug!("notification/kde_connect: listing all devices");

    execute(&["--list-devices", "--id-name-only"])
}

/// Return `kdeconnect-cli` stdout listing all available devices via the `list-available` argument.
fn list_available() -> Result<String> {
    log::debug!("notification/kde_connect: listing all available devices");

    execute(&["--list-available", "--id-name-only"])
}

/// Execute `kdeconnect-cli` command with the given arguments.
///
/// Warn if any data is passed into stderr.
/// Return stdout data.
fn execute(args: &[&str]) -> Result<String> {
    let output =
        common::command(&format!("{} {}", "kdeconnect-cli", args.join(" ")))?;

    let stderr = common::slice_to_string(output.stderr.as_slice());
    if !stderr.is_empty() {
        log::warn!("kdeconnect-cli: stderr = {}", &stderr.trim());
    }

    let stdout = common::slice_to_string(output.stdout.as_slice());
    if !stdout.is_empty() {
        log::trace!("kdeconnect-cli: stdout = {}", &stdout.trim());
    }

    Ok(stdout)
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
                "KDE Connect Notifier: threshold = {}%, device_names = {}",
                self.threshold, device_names,
            )
        }
    }
}
