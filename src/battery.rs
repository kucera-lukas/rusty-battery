use std::{fmt, io, num, process::Command};

use regex::Regex;
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

fn upower_command() -> Result<String, BatteryError> {
    let output = Command::new("upower")
        .args(["-i", "/org/freedesktop/UPower/devices/battery_BAT1"])
        .output()?;

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn battery_percentage(output: &String) -> Result<i32, BatteryError> {
    let re = Regex::new(r".*percentage:\s+?(\d+)%.*")?;
    let caps = re.captures(output).ok_or_else(|| fmt::Error)?;

    Ok(caps.get(1).ok_or_else(|| fmt::Error)?.as_str().parse()?)
}

pub fn get_battery_percentage() -> Result<i32, BatteryError> {
    let output = upower_command()?;
    let result = battery_percentage(&output)?;

    Ok(result)
}
