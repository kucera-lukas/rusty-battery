//! Battery information.

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BatteryState {
    Charging,
    Discharging,
}

#[derive(Debug)]
pub struct BatteryInfo {
    pub percentage: u8,
    pub state: BatteryState,

    device: battery::Battery,
}

impl BatteryInfo {
    /// Construct a new `BatteryInfo` instance.
    pub fn new(model: Option<&str>) -> Self {
        let device = device(model);

        Self {
            percentage: percentage(&device),
            state: state(&device),
            device,
        }
    }

    /// Update attributes to current battery values.
    pub fn refresh(&mut self) -> &mut Self {
        self.refresh_percentage().refresh_state()
    }

    /// Update battery percentage to current value.
    fn refresh_percentage(&mut self) -> &mut Self {
        self.percentage = percentage(&self.device);
        self
    }

    /// Update `BatteryState` to current state.
    fn refresh_state(&mut self) -> &mut Self {
        self.state = state(&self.device);
        self
    }
}

/// Print all available batteries formatted in a nice and readable way.
///
/// Acts as an high level API for the CLI `Batteries` subcommand.
pub fn print_batteries() {
    batteries().enumerate().for_each(|(idx, battery)| {
        println!(
            "Battery Device {}: model = {}, percentage = {}, state = {}",
            idx + 1,
            battery.model().unwrap(),
            percentage(&battery),
            state(&battery),
        )
    })
}

/// Return `Iterator` over all available `battery::Battery` devices.
fn batteries() -> impl Iterator<Item = battery::Battery> {
    battery::Manager::new()
        // this never panics on linux
        .unwrap()
        .batteries()
        .expect("failed to read the /sys/class/power_supply directory")
        .take_while(Result::is_ok)
        .flatten()
}

/// Return an instance of `battery::Battery`.
///
/// # Panics
///
/// If no `battery::Battery` was found.
/// If `model` was `None` and more than one `battery::Battery` exists.
/// If `model` was `Some` and the method `model` of each `battery::Battery` device did not match it.
fn device(model: Option<&str>) -> battery::Battery {
    let mut batteries = batteries();

    match model {
        None => match batteries.next() {
            None => panic!("no battery devices found"),
            Some(battery) => match batteries.next() {
                // only one battery device found so specifying model would not change anything
                // this should be the most common case
                None => battery,
                Some(_) => panic!(
                    "multiple battery devices found, please specify which model to use"
                )
            },
        },
        Some(model) => match batteries
            .find(|battery| battery.model() == Some(model))
        {
            None => panic!(
                "no battery device with model = \"{}\" was found",
                model
            ),
            Some(battery) => battery,
        },
    }
}

/// Return current battery percentage of the given `battery::Battery` device.
#[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
fn percentage(device: &battery::Battery) -> u8 {
    let percentage = device
        .state_of_charge()
        .get::<battery::units::ratio::percent>()
        .trunc() as u8;

    log::debug!("current battery percentage = {}%", percentage);

    percentage
}

/// Return current `BatterState` of the given `battery::Battery` device.
///
/// # Panics
///
/// If the `state` method of the given `battery::Battery` device
/// returns `battery::State::Unknown`.
fn state(device: &battery::Battery) -> BatteryState {
    let state = device.state();

    log::debug!("current battery state = {}", state);

    match state {
        battery::State::Charging | battery::State::Full => {
            BatteryState::Charging
        }
        battery::State::Discharging | battery::State::Empty => {
            BatteryState::Discharging
        }
        _ => panic!("unknown battery state: {}", state),
    }
}

mod std_fmt_impls {
    use std::fmt;

    use super::{BatteryInfo, BatteryState};

    impl fmt::Display for BatteryState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Charging => write!(f, "Charging"),
                Self::Discharging => write!(f, "Discharging"),
            }
        }
    }

    impl fmt::Display for BatteryInfo {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "battery percentage: {}%, state: {}",
                self.percentage, self.state,
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
