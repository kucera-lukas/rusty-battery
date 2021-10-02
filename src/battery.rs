//! Battery information.
use std::convert::TryFrom;
use std::result;

use crate::error::{BatteryError, DeviceError, Model};

type BatteryResult<T> = result::Result<T, BatteryError>;
type DeviceResult<T> = result::Result<T, DeviceError>;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BatteryState {
    Charging,
    Discharging,
    Unknown,
}

#[derive(Debug)]
pub struct BatteryDevice {
    pub percentage: u8,
    pub state: BatteryState,
    pub model: String,
    pub serial_number: String,

    battery: battery::Battery,
}

impl BatteryDevice {
    /// Construct a new `BatteryInfo` instance.
    #[allow(dead_code)]
    pub fn new(model: &str) -> BatteryResult<Self> {
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
    pub fn refresh(&mut self) -> BatteryResult<&mut Self> {
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
    fn refresh_state(&mut self) -> BatteryState {
        let state = fetch_state(&self.battery);
        self.state = state;

        self.debug(&format!("state = {}", state));

        state
    }

    fn debug(&self, message: &str) {
        log::debug!("Battery Device {}: {}", self.serial_number, message);
    }
}

impl TryFrom<battery::Battery> for BatteryDevice {
    type Error = BatteryError;

    fn try_from(device: battery::Battery) -> Result<Self, Self::Error> {
        let device = Self {
            percentage: fetch_percentage(&device),
            state: fetch_state(&device),
            model: fetch_model(&device)?,
            serial_number: fetch_serial_number(&device)?,
            battery: device,
        };

        log::info!("created = {}", device);

        Ok(device)
    }
}

impl TryFrom<Option<&str>> for BatteryDevice {
    type Error = BatteryError;

    fn try_from(value: Option<&str>) -> Result<Self, Self::Error> {
        match value {
            None => Self::try_from(one_battery()?),
            Some(value) => Self::try_from(find_battery(value)?),
        }
    }
}
/// Print all available `BatteryDevice` instances formatted in a nice and readable way.
///
/// Acts as an high level API for the CLI `Batteries` subcommand.
pub fn print_devices() -> BatteryResult<()> {
    devices()?
        .iter()
        .for_each(|battery| println!("{}", battery));
    Ok(())
}

/// Return `Vec` of all available `battery::Battery` devices.
fn devices() -> BatteryResult<Vec<BatteryDevice>> {
    batteries()?.map(BatteryDevice::try_from).collect()
}

/// Return `Iterator` over all available `battery::Battery` devices.
fn batteries() -> BatteryResult<impl Iterator<Item = battery::Battery>> {
    Ok(battery::Manager::new()?
        .batteries()?
        .take_while(Result::is_ok)
        .flatten())
}

/// Return `battery::Battery` instance if it's the only one found for the current device.
fn one_battery() -> BatteryResult<battery::Battery> {
    let mut batteries = batteries()?;

    match batteries.next() {
        None => Err(BatteryError::NotFound { model: Model(None) }),
        Some(battery) => match batteries.next() {
            None => Ok(battery),
            Some(_) => Err(BatteryError::NotFound { model: Model(None) }),
        },
    }
}

/// Return `battery::Battery` instance which matches the given model name.
fn find_battery(model: &str) -> BatteryResult<battery::Battery> {
    match batteries()?.find(|battery| battery.model() == Some(model)) {
        None => Err(BatteryError::NotFound {
            model: Model(Some(model.to_owned())),
        }),
        Some(battery) => Ok(battery),
    }
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
fn fetch_state(device: &battery::Battery) -> BatteryState {
    match device.state() {
        battery::State::Charging | battery::State::Full => {
            BatteryState::Charging
        }
        battery::State::Discharging | battery::State::Empty => {
            BatteryState::Discharging
        }
        _ => BatteryState::Unknown,
    }
}

/// Return battery model of the given `battery::Battery` device.
fn fetch_model(device: &battery::Battery) -> DeviceResult<String> {
    Ok(device.model().ok_or(DeviceError::Model)?.to_owned())
}

/// Return serial number of the given `battery::Battery` device.
fn fetch_serial_number(device: &battery::Battery) -> DeviceResult<String> {
    Ok(device
        .serial_number()
        .ok_or(DeviceError::SerialNumber)?
        .trim()
        .to_owned())
}

mod std_fmt_impls {
    use std::fmt;

    use super::{BatteryDevice, BatteryState};

    impl fmt::Display for BatteryState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Charging => write!(f, "Charging"),
                Self::Discharging => write!(f, "Discharging"),
                Self::Unknown => write!(f, "Unknown"),
            }
        }
    }

    impl fmt::Display for BatteryDevice {
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
        let state = BatteryState::Charging;

        let display = format!("{}", state);

        assert_eq!(display, "Charging");
    }

    #[test]
    fn test_battery_state_discharging_display() {
        let state = BatteryState::Discharging;

        let display = format!("{}", state);

        assert_eq!(display, "Discharging");
    }
}
