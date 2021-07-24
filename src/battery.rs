use std::fmt::Formatter;
use std::{fmt, io, num, process::Command};

use lazy_static::lazy_static;
use regex::{Captures, Regex};

#[derive(Debug)]
pub enum BatteryError {
    Command(io::Error),
    ParseInt(num::ParseIntError),
    Output(fmt::Error),
}

impl fmt::Display for BatteryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BatteryError::Command(ref err) => write!(f, "Command Error: {}", err),
            BatteryError::ParseInt(ref err) => write!(f, "ParseInt Error: {}", err),
            BatteryError::Output(ref err) => write!(f, "Output Error {}", err),
        }
    }
}

impl From<io::Error> for BatteryError {
    fn from(err: io::Error) -> Self {
        BatteryError::Command(err)
    }
}

impl From<num::ParseIntError> for BatteryError {
    fn from(err: num::ParseIntError) -> Self {
        BatteryError::ParseInt(err)
    }
}

impl From<fmt::Error> for BatteryError {
    fn from(err: fmt::Error) -> Self {
        BatteryError::Output(err)
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    CHARGING,
    DISCHARGING,
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            State::CHARGING => write!(f, "Charging"),
            State::DISCHARGING => write!(f, "Discharging"),
        }
    }
}

#[derive(Debug)]
pub struct Info {
    pub percentage: i32,
    pub state: State,
}

impl Info {
    pub fn new() -> Result<Info, BatteryError> {
        let output = upower_command()?;

        Ok(Info {
            percentage: battery_percentage(&output)?,
            state: battery_state(&output)?,
        })
    }

    pub fn refresh(&mut self) -> Result<(), BatteryError> {
        let new_info = Info::new()?;

        self.percentage = new_info.percentage;
        self.state = new_info.state;

        Ok(())
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Battery percentage: {}, State: {}",
            self.percentage, self.state,
        )
    }
}

fn upower_command() -> Result<String, BatteryError> {
    let output = Command::new("upower")
        .args(["-i", "/org/freedesktop/UPower/devices/battery_BAT1"])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn percentage_caps(output: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*percentage:\s+?(\d+)%.*").unwrap();
    }
    RE.captures(output)
}

fn status_caps(output: &str) -> Option<Captures> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r".*state:\s+?([a-z]+).*").unwrap();
    }
    RE.captures(output)
}

fn caps_to_str<'a>(caps: &'a Option<Captures>) -> Result<&'a str, BatteryError> {
    Ok(caps
        .as_ref()
        .ok_or_else(|| fmt::Error)?
        .get(1)
        .ok_or_else(|| fmt::Error)?
        .as_str())
}

fn battery_percentage(output: &str) -> Result<i32, BatteryError> {
    let caps = percentage_caps(output);
    Ok(caps_to_str(&caps)?.parse()?)
}

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
