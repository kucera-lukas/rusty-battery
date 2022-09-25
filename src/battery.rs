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

        log::info!("refreshed = {}", self);

        Ok(self)
    }

    /// Refresh and return battery percentage.
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn refresh_percentage(&mut self) -> u8 {
        let percentage = fetch_percentage(&self.battery);
        self.percentage = percentage;

        self.debug(&format!("battery percentage = {}%", percentage));

        percentage
    }

    /// Refresh and return `BatteryState`.
    fn refresh_state(&mut self) -> State {
        let state = fetch_state(&self.battery);
        self.state = state;

        self.debug(&format!("state = {}", state));

        state
    }

    fn debug(&self, message: &str) {
        log::debug!("Battery Device {}: {}", self.serial_number, message);
    }
}

impl TryFrom<battery::Battery> for Device {
    type Error = error::Battery;

    fn try_from(
        device: battery::Battery,
    ) -> result::Result<Self, Self::Error> {
        let device = Self {
            percentage: fetch_percentage(&device),
            state: fetch_state(&device),
            model: fetch_model(&device)?,
            serial_number: fetch_serial_number(&device)?,
            battery: device,
        };

        log::info!("{}", device);

        Ok(device)
    }
}

impl TryFrom<Option<&str>> for Device {
    type Error = error::Battery;

    fn try_from(value: Option<&str>) -> result::Result<Self, Self::Error> {
        match value {
            None => Self::try_from(one_battery()?),
            Some(value) => Self::try_from(find_battery(value)?),
        }
    }
}
/// Print all available `BatteryDevice` instances formatted in a nice and readable way.
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

/// Return `battery::Battery` instance if it's the only one found for the current device.
fn one_battery() -> Result<battery::Battery> {
    let mut batteries = batteries()?;

    match batteries.next() {
        None => Err(error::Battery::NotFound {
            model: error::Model(None),
        }),
        Some(battery) => match batteries.next() {
            None => Ok(battery?),
            Some(_) => Err(error::Battery::NotFound {
                model: error::Model(None),
            }),
        },
    }
}

/// Return `battery::Battery` instance which matches the given model name.
fn find_battery(model: &str) -> Result<battery::Battery> {
    for battery in batteries()? {
        match battery {
            Ok(battery) => {
                if battery.model() == Some(model) {
                    return Ok(battery);
                }
            }
            Err(e) => return Err(error::Battery::from(e)),
        }
    }

    Err(error::Battery::NotFound {
        model: error::Model(Some(model.to_owned())),
    })
}

/// Return current battery percentage of the given `battery::Battery` device.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn fetch_percentage(device: &battery::Battery) -> u8 {
    device
        .state_of_charge()
        .get::<battery::units::ratio::percent>()
        .trunc() as u8
}

/// Return current `BatterState` of the given `battery::Battery` device.
fn fetch_state(device: &battery::Battery) -> State {
    match device.state() {
        battery::State::Charging | battery::State::Full => State::Charging,
        battery::State::Discharging | battery::State::Empty => {
            State::Discharging
        }
        _ => State::Unknown,
    }
}

/// Return battery model of the given `battery::Battery` device.
fn fetch_model(device: &battery::Battery) -> DeviceResult<String> {
    Ok(device
        .model()
        .ok_or(error::BatteryDevice::Model)?
        .to_owned())
}

/// Return serial number of the given `battery::Battery` device.
fn fetch_serial_number(device: &battery::Battery) -> DeviceResult<String> {
    Ok(device
        .serial_number()
        .ok_or(error::BatteryDevice::SerialNumber)?
        .trim()
        .to_owned())
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
                "Battery Device {}: percentage = {}%, state = {}, model = {}",
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
}
