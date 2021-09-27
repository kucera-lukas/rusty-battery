use std::process::{Command, Output};

use crate::common;

#[derive(Debug)]
pub struct KDENotifier {
    threshold: u8,
    devices: Vec<Device>,
}

impl KDENotifier {
    pub fn new(threshold: u8) -> Self {
        Self {
            threshold,
            devices: device_vec(),
        }
    }

    pub fn ping(&self) {
        self.devices.iter().for_each(|device| {
            ping(&device.id, &common::warning_message(self.threshold));
        });
        log::debug!("KDE Connect devices pinged");
    }
}

#[derive(Debug)]
pub struct Device {
    id: String,
    name: String,
}

fn device_vec() -> Vec<Device> {
    String::from_utf8_lossy(
        execute(["--list-devices", "--id-name-only"])
            .stdout
            .as_slice(),
    )
    .lines()
    .map(|line| {
        let mut data = line.split_whitespace().map(ToOwned::to_owned);
        let id = data.next().expect("KDE Connect device id missing");
        let name = data.next().expect("KDE Connect device name missing");
        Device { id, name }
    })
    .collect()
}

fn ping(id: &str, message: &str) {
    execute(["--device", id, "--ping-msg", message]);
}

fn execute<'a>(args: impl IntoIterator<Item = &'a str>) -> Output {
    Command::new("kdeconnect-cli")
        .args(args)
        .output()
        .expect("KDE Connect CLI is not available")
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
