//! Battery information.
use std::convert::TryFrom;
use std::result;

use crate::common;
use crate::error;

type Result<T> = result::Result<T, error::Battery>;
type DeviceResult<T> = result::Result<T, error::BatteryDevice>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum State {
    Charging,
    Discharging,
    Unknown,
}

#[derive(Debug)]
pub struct Device {
    pub percentage: u8,
    pub state: State,
    pub model: String,
    pub serial_number: String,

    battery: battery::Battery,
}

impl Device {
    /// Construct a new `BatteryInfo` instance.
    #[allow(dead_code)]
    pub fn new(model: &str) -> Result<Self> {
        let battery = find_battery(model)?;

        Ok(Self {
            percentage: fetch_percentage(&battery),
            state: fetch_state(&battery),
            model: fetch_model(&battery)?,
            serial_number: fetch_serial_number(&battery)?,
            battery,
        })
    }

    /// Update attributes to current battery values.
    pub fn refresh(&mut self) -> Result<&mut Self> {
        self.battery.refresh()?;

        self.refresh_percentage();
        self.refresh_state();

        log::info!("battery: refreshed state = {}", self);

        Ok(self)
    }

    /// Refresh and return battery percentage.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn refresh_percentage(&mut self) -> u8 {
        let percentage = fetch_percentage(&self.battery);
        self.percentage = percentage;

        self.debug(&format!("refreshed percentage = {}%", percentage));

        percentage
    }

    /// Refresh and return `BatteryState`.
    fn refresh_state(&mut self) -> State {
        let state = fetch_state(&self.battery);
        self.state = state;

        self.debug(&format!("refreshed state = {}", state));

        state
    }

    fn debug(&self, message: &str) {
        log::debug!("battery: \"{}\" {}", self.serial_number, message);
    }
}

impl TryFrom<battery::Battery> for Device {
    type Error = error::Battery;

    fn try_from(
        battery: battery::Battery,
    ) -> result::Result<Self, Self::Error> {
        let device = Self {
            percentage: fetch_percentage(&battery),
            state: fetch_state(&battery),
            model: fetch_model(&battery)?,
            serial_number: fetch_serial_number(&battery)?,
            battery,
        };

        log::info!(
            "battery: device {} created from battery \"{}\"",
            device,
            device.serial_number,
        );

        Ok(device)
    }
}

impl TryFrom<Option<&str>> for Device {
    type Error = error::Battery;

    fn try_from(value: Option<&str>) -> result::Result<Self, Self::Error> {
        match value {
            None => {
                log::info!(
                    "battery: model not specified, \
                    checking whether device has only one",
                );

                Self::try_from(one_battery()?)
            }
            Some(value) => {
                log::debug!(
                    "battery: searching for battery model \"{}\"",
                    value,
                );

                Self::try_from(find_battery(value)?)
            }
        }
    }
}

/// Print all available `BatteryDevice`s formatted in a readable way.
///
/// Acts as an high level API for the CLI `Batteries` subcommand.
pub fn print_devices() -> Result<()> {
    common::print_slice(&devices()?);

    Ok(())
}

/// Return `Vec` of all available `battery::Battery` devices.
fn devices() -> Result<Vec<Device>> {
    batteries()?
        .map(|battery| Device::try_from(battery?))
        .collect()
}

/// Return `Iterator` over all available `battery::Battery` devices.
fn batteries() -> Result<
    impl Iterator<Item = result::Result<battery::Battery, battery::Error>>,
> {
    Ok(battery::Manager::new()?.batteries()?)
}

/// Return `Battery` instance if it's the only one found for the current device.
fn one_battery() -> Result<battery::Battery> {
    let mut batteries = batteries()?;

    match batteries.next() {
        None => {
            log::error!("battery/one: 0 batteries found");

            Err(error::Battery::NotFound {
                model: error::Model(None),
            })
        }
        Some(battery) => match batteries.next() {
            None => {
                log::info!("battery/one: single battery found");

                Ok(battery?)
            }
            Some(_) => {
                log::error!("battery/one: more than 1 battery found");

                Err(error::Battery::NotFound {
                    model: error::Model(None),
                })
            }
        },
    }
}

/// Return `battery::Battery` instance which matches the given model name.
fn find_battery(model: &str) -> Result<battery::Battery> {
    for battery in batteries()? {
        match battery {
            Ok(battery) => {
                let found = battery.model().map_or_else(
                    || {
                        log::debug!("battery/find: missing model name");

                        false
                    },
                    |battery_model| {
                        log::trace!(
                            "battery/find: checking battery \"{}\"",
                            battery_model
                        );

                        battery_model == model
                    },
                );

                if found {
                    log::info!("battery/find: battery \"{}\" found", model);

                    return Ok(battery);
                }
            }
            Err(e) => return Err(error::Battery::from(e)),
        }
    }

    log::error!("battery/find: battery \"{}\" not found", model);

    Err(error::Battery::NotFound {
        model: error::Model(Some(model.to_owned())),
    })
}

/// Return current battery percentage of the given `battery::Battery` device.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn fetch_percentage(device: &battery::Battery) -> u8 {
    let percentage = device
        .state_of_charge()
        .get::<battery::units::ratio::percent>()
        .trunc() as u8;

    log::trace!("battery: fetched state of charge = {}%", percentage);

    percentage
}

/// Return current `BatterState` of the given `battery::Battery` device.
fn fetch_state(device: &battery::Battery) -> State {
    let state = match device.state() {
        battery::State::Charging | battery::State::Full => State::Charging,
        battery::State::Discharging | battery::State::Empty => {
            State::Discharging
        }
        _ => State::Unknown,
    };

    log::trace!("battery: fetched battery state = {}", state);

    state
}

/// Return battery model of the given `battery::Battery` device.
fn fetch_model(device: &battery::Battery) -> DeviceResult<String> {
    let model = device
        .model()
        .ok_or(error::BatteryDevice::Model)?
        .to_owned();

    log::trace!("battery: fetched model = \"{}\"", model);

    Ok(model)
}

/// Return serial number of the given `battery::Battery` device.
fn fetch_serial_number(device: &battery::Battery) -> DeviceResult<String> {
    let serial_number = device
        .serial_number()
        .ok_or(error::BatteryDevice::SerialNumber)?
        .trim()
        .to_owned();

    log::trace!("battery: fetched serial number = {}", serial_number);

    Ok(serial_number)
}

mod std_fmt_impls {
    use std::fmt;

    use super::{Device, State};

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Charging => write!(f, "Charging"),
                Self::Discharging => write!(f, "Discharging"),
                Self::Unknown => write!(f, "Unknown"),
            }
        }
    }

    impl fmt::Display for Device {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Battery Device {}: percentage = {}%, \
                state = {}, model = \"{}\"",
                self.serial_number, self.percentage, self.state, self.model,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_battery_state_charging_display() {
        let state = State::Charging;

        let display = format!("{}", state);

        assert_eq!(display, "Charging");
    }

    #[test]
    fn test_battery_state_discharging_display() {
        let state = State::Discharging;

        let display = format!("{}", state);

        assert_eq!(display, "Discharging");
    }

    #[test]
    fn test_battery_state_unknown_display() {
        let state = State::Unknown;

        let display = format!("{}", state);

        assert_eq!(display, "Unknown");
    }
}
