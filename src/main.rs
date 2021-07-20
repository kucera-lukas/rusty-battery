use std::process::Command;

use regex;
use std::io;

fn upower_command() -> Result<String, io::Error> {
    let output = Command::new("upower")
        .args(["-i", "/org/freedesktop/UPower/devices/battery_BAT1"])
        .output()?;
    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

fn battery_percentage(output: &String) -> Option<i32> {
    let re = regex::Regex::new(r".*percentage:\s+?(\d+)%.*").unwrap();
    let caps = re.captures(output)?;
    let result = match caps.get(1)?.as_str().parse() {
        Ok(n) => Some(n),
        Err(..) => None,
    };

    result
}

fn main() {
    let output = upower_command().expect("cannot get battery data");
    let percentage = battery_percentage(&output).expect("could not process battery output");
    println!("{:?}", percentage)
}
