use std::{fmt, io, num, process::Command};

use regex::{Captures, Regex};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum BatteryError {
    Command(io::Error),
    ParseInt(num::ParseIntError),
    Regex(regex::Error),
    Output(fmt::Error),
}

impl fmt::Display for BatteryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            BatteryError::Command(ref err) => write!(f, "Command Error: {}", err),
            BatteryError::ParseInt(ref err) => write!(f, "ParseInt Error: {}", err),
            BatteryError::Regex(ref err) => write!(f, "Regex Error: {}", err),
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

impl From<regex::Error> for BatteryError {
    fn from(err: regex::Error) -> Self {
        BatteryError::Regex(err)
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

fn output_captures<'a>(output: &'a str, re: &str) -> Result<Captures<'a>, BatteryError> {
    let re = Regex::new(re)?;
    let caps = re.captures(output).ok_or_else(|| fmt::Error)?;

    Ok(caps)
}

fn battery_percentage(output: &str) -> Result<i32, BatteryError> {
    let caps = output_captures(output, r".*percentage:\s+?(\d+)%.*")?;
    Ok(caps.get(1).ok_or_else(|| fmt::Error)?.as_str().parse()?)
}

fn battery_state(output: &String) -> Result<State, BatteryError> {
    let caps = output_captures(output, r".*state:\s+?([a-z]+).*")?;
    let state = caps.get(1).ok_or_else(|| fmt::Error)?.as_str();

    let result = match state {
        "charging" => State::CHARGING,
        "discharging" => State::DISCHARGING,
        _ => return Err(BatteryError::Output(fmt::Error)),
    };

    Ok(result)
}
