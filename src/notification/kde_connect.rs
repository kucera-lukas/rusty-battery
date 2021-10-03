use std::process::{Command, Output};
use std::result;

use crate::common;
use crate::error;

type Result<T> = result::Result<T, error::KDEConnect>;

#[derive(Debug)]
pub struct Notifier {
    threshold: u8,
    devices: Vec<Device>,
}

impl Notifier {
    pub fn new(threshold: u8) -> Result<Self> {
        Ok(Self {
            threshold,
            devices: device_vec()?,
        })
    }

    pub fn ping(&self) -> Result<()> {
        self.devices.iter().try_for_each(|device| {
            ping(&device.id, &common::warning_message(self.threshold))
        })?;

        log::debug!("KDE Connect devices pinged");

        Ok(())
    }
}

#[derive(Debug)]
pub struct Device {
    id: String,
    name: String,
}

pub fn print_devices() -> Result<()> {
    common::print_slice(&device_vec()?);
    Ok(())
}

fn device_vec() -> Result<Vec<Device>> {
    String::from_utf8_lossy(
        execute(&["--list-devices", "--id-name-only"])?
            .stdout
            .as_slice(),
    )
    .lines()
    .map(|line| {
        let mut data = line.split_whitespace().map(ToOwned::to_owned);
        let id = data.next().ok_or(error::KDEConnectDevice::ID)?;
        let name = data.next().ok_or(error::KDEConnectDevice::Name)?;
        Ok(Device { id, name })
    })
    .collect()
}

fn ping(id: &str, message: &str) -> Result<()> {
    execute(&["--device", id, "--ping-msg", message])?;
    log::debug!("pinged {}", id);
    Ok(())
}

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
                "KDENotifier: threshold = {}, devices = [{}]",
                self.threshold, devices
            )
        }
    }
}
