use std::process::{Command, Output};

use crate::common;
use crate::error::{KDEConnectDeviceError, KDEConnectResult};

#[derive(Debug)]
pub struct KDENotifier {
    threshold: u8,
    devices: Vec<Device>,
}

impl KDENotifier {
    pub fn new(threshold: u8) -> KDEConnectResult<Self> {
        Ok(Self {
            threshold,
            devices: device_vec()?,
        })
    }

    pub fn ping(&self) -> KDEConnectResult<()> {
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

pub fn print_devices() -> KDEConnectResult<()> {
    device_vec()?
        .iter()
        .for_each(|device| println!("{}", device));
    Ok(())
}

fn device_vec() -> KDEConnectResult<Vec<Device>> {
    String::from_utf8_lossy(
        execute(&["--list-devices", "--id-name-only"])?
            .stdout
            .as_slice(),
    )
    .lines()
    .map(|line| {
        let mut data = line.split_whitespace().map(ToOwned::to_owned);
        let id = data.next().ok_or(KDEConnectDeviceError::ID)?;
        let name = data.next().ok_or(KDEConnectDeviceError::Name)?;
        Ok(Device { id, name })
    })
    .collect()
}

fn ping(id: &str, message: &str) -> KDEConnectResult<()> {
    execute(&["--device", id, "--ping-msg", message])?;
    log::debug!("pinged {}", id);
    Ok(())
}

fn execute(args: &[&str]) -> KDEConnectResult<Output> {
    let output = Command::new("kdeconnect-cli").args(args).output()?;

    log::debug!("kdeconnect-cli: args = {:?}, output = {:?}", args, &output);

    Ok(output)
}

mod std_fmt_impls {
    use std::fmt;

    use super::{Device, KDENotifier};

    impl fmt::Display for Device {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "KDE Connect device: name = {}, id = {}",
                self.name, self.id
            )
        }
    }

    impl fmt::Display for KDENotifier {
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
