use std::process::Command;

use regex;

fn upower_command() -> String {
    let output = Command::new("upower")
        .args(["-i", "/org/freedesktop/UPower/devices/battery_BAT1"])
        .output()
        .unwrap();
    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn battery_percentage(output: &String) -> i32 {
    let re = regex::Regex::new(r".*percentage:\s+?(\d+)%.*").unwrap();
    let caps = re.captures(output).unwrap();
    caps.get(1).unwrap().as_str().parse().unwrap()
}

fn main() {
    let output = upower_command();
    let percentage = battery_percentage(&output);
    println!("{:?}", percentage)
}
