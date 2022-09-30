//! Battery device.
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
pub struct Battery {
    pub percentage: u8,
    pub state: State,
    pub model: String,
    pub serial_number: String,

    battery: battery::Battery,
}

impl Battery {
    /// Construct a new `BatteryDevice` instance.
    pub fn new(model: &str) -> Result<Self> {
        let battery = find(model)?;

        Ok(Self {
            percentage: fetch::percentage(&battery),
            state: fetch::state(&battery),
            model: fetch::model(&battery)?,
            serial_number: fetch::serial_number(&battery)?,
            battery,
        })
    }

    /// Update attributes to current battery values.
    pub fn refresh(&mut self) -> Result<&mut Self> {
        self.battery.refresh()?;

        self.refresh_percentage();
        self.refresh_state();

        log::info!("device/battery: refreshed = {self}");

        Ok(self)
    }

    /// Refresh and return battery percentage.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn refresh_percentage(&mut self) -> u8 {
        let percentage = fetch::percentage(&self.battery);
        self.percentage = percentage;

        log::debug!("device/battery: refreshed percentage = {percentage}%");

        percentage
    }

    /// Refresh and return `BatteryState`.
    fn refresh_state(&mut self) -> State {
        let state = fetch::state(&self.battery);
        self.state = state;

        log::debug!("device/battery: refreshed state = {state}");

        state
    }
}

impl TryFrom<battery::Battery> for Battery {
    type Error = error::Battery;

    fn try_from(
        battery: battery::Battery,
    ) -> result::Result<Self, Self::Error> {
        let device = Self {
            percentage: fetch::percentage(&battery),
            state: fetch::state(&battery),
            model: fetch::model(&battery)?,
            serial_number: fetch::serial_number(&battery)?,
            battery,
        };

        log::info!(
            "device/battery: {device} created from battery \"{}\"",
            device.serial_number,
        );

        Ok(device)
    }
}

impl TryFrom<Option<&str>> for Battery {
    type Error = error::Battery;

    fn try_from(value: Option<&str>) -> result::Result<Self, Self::Error> {
        match value {
            None => {
                log::info!(
                    "device/battery: model not specified, \
                    checking whether device has only one",
                );

                Self::try_from(one()?)
            }
            Some(value) => {
                log::debug!(
                    "device/battery: searching for \
                     battery model \"{value}\""
                );

                Self::new(value)
            }
        }
    }
}

/// Print all available `BatteryDevice`s formatted in a readable way.
///
/// Acts as an high level API for the CLI `Batteries` subcommand.
pub fn print() -> Result<()> {
    common::print_slice(&all()?);

    Ok(())
}

/// Return a `Vec` of all available `BatteryDevice` instances.
fn all() -> Result<Vec<Battery>> {
    iterator()?
        .map(|battery| Battery::try_from(battery?))
        .collect()
}

/// Return `Iterator` over all available `battery::Battery` devices.
fn iterator() -> Result<
    impl Iterator<Item = result::Result<battery::Battery, battery::Error>>,
> {
    Ok(battery::Manager::new()?.batteries()?)
}

/// Return `battery::Battery` instance if it's the only one found.
fn one() -> Result<battery::Battery> {
    let mut batteries = iterator()?;

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
fn find(model: &str) -> Result<battery::Battery> {
    for battery in iterator()? {
        match battery {
            Ok(battery) => {
                let found = battery.model().map_or_else(
                    || {
                        log::debug!("device/battery: missing model name");

                        false
                    },
                    |battery_model| {
                        log::trace!(
                            "device/battery: checking battery \
                            \"{battery_model}\""
                        );

                        battery_model == model
                    },
                );

                if found {
                    log::info!("device/battery: battery \"{model}\" found");

                    return Ok(battery);
                }
            }
            Err(e) => return Err(error::Battery::from(e)),
        }
    }

    log::error!("device/battery: \"{model}\" not found");

    Err(error::Battery::NotFound {
        model: error::Model(Some(model.to_owned())),
    })
}

mod fetch {
    use super::{error, DeviceResult, State};

    /// Fetch battery percentage of the given `battery::Battery` device.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    pub fn percentage(device: &battery::Battery) -> u8 {
        let percentage = device
            .state_of_charge()
            .get::<battery::units::ratio::percent>()
            .trunc() as u8;

        log::trace!("device/battery: fetched state of charge = {percentage}%");

        percentage
    }

    /// Fetch `BatteryState` of the given `battery::Battery` device.
    pub fn state(device: &battery::Battery) -> State {
        let state = match device.state() {
            battery::State::Charging | battery::State::Full => State::Charging,
            battery::State::Discharging | battery::State::Empty => {
                State::Discharging
            }
            _ => State::Unknown,
        };

        log::trace!("device/battery: fetched state = {state}");

        state
    }

    /// Fetch battery model of the given `battery::Battery` device.
    pub fn model(device: &battery::Battery) -> DeviceResult<String> {
        let model = device
            .model()
            .ok_or(error::BatteryDevice::Model)?
            .to_owned();

        log::trace!("device/battery: fetched model = \"{model}\"");

        Ok(model)
    }

    /// Fetch serial number of the given `battery::Battery` device.
    pub fn serial_number(device: &battery::Battery) -> DeviceResult<String> {
        let serial_number = device
            .serial_number()
            .ok_or(error::BatteryDevice::SerialNumber)?
            .trim()
            .to_owned();

        log::trace!("device/battery: fetched serial number = {serial_number}");

        Ok(serial_number)
    }
} // fetch

mod std_fmt_impls {
    use std::fmt;

    use super::{Battery, State};

    impl fmt::Display for State {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Charging => write!(f, "Charging"),
                Self::Discharging => write!(f, "Discharging"),
                Self::Unknown => write!(f, "Unknown"),
            }
        }
    }

    impl fmt::Display for Battery {
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

        let display = format!("{state}");

        assert_eq!(display, "Charging");
    }

    #[test]
    fn test_battery_state_discharging_display() {
        let state = State::Discharging;

        let display = format!("{state}");

        assert_eq!(display, "Discharging");
    }

    #[test]
    fn test_battery_state_unknown_display() {
        let state = State::Unknown;

        let display = format!("{state}");

        assert_eq!(display, "Unknown");
    }
}
