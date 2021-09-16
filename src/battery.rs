//! Battery information.

use std::{fmt, result};

use battery::{units, Battery, Manager, State as BatteryState};
use thiserror::Error;

pub type Result<T> = result::Result<T, BatteryError>;

#[derive(Error, Debug)]
pub enum BatteryError {
    #[error("could not find any battery device")]
    DeviceError,
    #[error("battery information failure")]
    SystemError(#[from] battery::Error),
    #[error("unknown battery state: {state}")]
    UnknownState { state: BatteryState },
}

#[derive(Debug, PartialEq)]
pub enum State {
    Charging,
    Discharging,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::Charging => write!(f, "Charging"),
            State::Discharging => write!(f, "Discharging"),
        }
    }
}

#[derive(Debug)]
pub struct Info {
    pub percentage: u8,
    pub state: State,

    device: Battery,
}

impl Info {
    /// Construct a new `Info` instance.
    pub fn new() -> Result<Self> {
        let device = battery_device()?;

        Ok(Self {
            percentage: battery_percentage(&device),
            state: battery_state(&device)?,
            device,
        })
    }

    /// Update attributes to current battery values.
    pub fn refresh(&mut self) -> Result<()> {
        self.percentage = battery_percentage(&self.device);
        self.state = battery_state(&self.device)?;

        Ok(())
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Battery percentage: {}%, State: {}",
            self.percentage, self.state,
        )
    }
}

/// Return `Battery` device object providing information about the battery.
fn battery_device() -> Result<Battery> {
    let manager = Manager::new()?;
    let device = manager
        .batteries()?
        .next()
        .ok_or(BatteryError::DeviceError)?;
    Ok(device?)
}

/// Return current battery percentage.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn battery_percentage(device: &Battery) -> u8 {
    device
        .state_of_charge()
        .get::<units::ratio::percent>()
        .trunc() as u8
}

/// Return current battery state as the `State` enum.
fn battery_state(device: &Battery) -> Result<State> {
    let state = device.state();

    let result = match state {
        BatteryState::Charging | BatteryState::Full => State::Charging,
        BatteryState::Discharging | BatteryState::Empty => State::Discharging,
        _ => return Err(BatteryError::UnknownState { state }),
    };

    Ok(result)
}
