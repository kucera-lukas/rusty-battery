//! Battery information.

use std::{error, fmt, io, num, process::Command};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

#[derive(Debug)]
pub enum BatteryError {
    Command(io::Error),
    ParseInt(num::ParseIntError),
    Output(fmt::Error),
}

impl error::Error for BatteryError {}

impl fmt::Display for BatteryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Command(ref err) => {
                write!(f, "Command Error: {}", err)
            }
            Self::ParseInt(ref err) => {
                write!(f, "ParseInt Error: {}", err)
            }
            Self::Output(ref err) => write!(f, "Output Error {}", err),
        }
    }
}

impl From<io::Error> for BatteryError {
    fn from(err: io::Error) -> Self {
        Self::Command(err)
    }
}

impl From<num::ParseIntError> for BatteryError {
    fn from(err: num::ParseIntError) -> Self {
        Self::ParseInt(err)
    }
}

impl From<fmt::Error> for BatteryError {
    fn from(err: fmt::Error) -> Self {
        Self::Output(err)
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    CHARGING,
    DISCHARGING,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            State::CHARGING => write!(f, "Charging"),
            State::DISCHARGING => write!(f, "Discharging"),
        }
    }
}

#[derive(Debug)]
pub struct Info {
    pub percentage: u8,
    pub state: State,
}

impl Info {
    /// Construct a new `Info` instance.
    pub fn new() -> Result<Self, BatteryError> {
        let output = upower_command()?;

        Ok(Self {
            percentage: battery_percentage(&output)?,
            state: battery_state(&output)?,
        })
    }

    /// Update attributes to current battery values.
    pub fn refresh(&mut self) -> Result<(), BatteryError> {
        let new_info = Self::new()?;

        self.percentage = new_info.percentage;
        self.state = new_info.state;

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

/// Return `UPower` command output.
fn upower_command() -> Result<String, BatteryError> {
    let output = Command::new("upower")
        .args(["-i", "/org/freedesktop/UPower/devices/battery_BAT1"])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Extract current battery percentage from `UPower` output.
fn battery_percentage(output: &str) -> Result<u8, BatteryError> {
    let caps = percentage_caps(output);
    Ok(caps_to_str(&caps)?.parse()?)
}

/// Extract current battery state as the `State` enum from `UPower` output.
fn battery_state(output: &str) -> Result<State, BatteryError> {
    let caps = status_caps(output);
    let state = caps_to_str(&caps)?;

    let result = match state {
        "charging" => State::CHARGING,
        "discharging" => State::DISCHARGING,
        _ => return Err(BatteryError::Output(fmt::Error)),
    };

    Ok(result)
}

/// Return percentage `Captures` from `UPower` output via `Regex`.
fn percentage_caps(output: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*percentage:\s+?(\d+)%.*")
            .expect("string literal is not valid regex");
    }
    RE.captures(output)
}

/// Return status `Captures` from `UPower` output via `Regex`.
fn status_caps(output: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*state:\s+?([a-z]+).*")
            .expect("string literal is not valid regex");
    }
    RE.captures(output)
}

/// Turn `Capture` into a `Result` and return it as a `str`.
fn caps_to_str<'a>(
    caps: &'a Option<Captures>,
) -> Result<&'a str, BatteryError> {
    Ok(caps
        .as_ref()
        .ok_or(fmt::Error)?
        .get(1)
        .ok_or(fmt::Error)?
        .as_str())
}
