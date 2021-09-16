use std::{error, fmt, thread, time};

use crate::{battery, notification};
use crate::{battery::BatteryError, notification::NotificationError};

#[derive(Debug)]
pub enum EventError {
    Battery(BatteryError),
    Notification(NotificationError),
}

impl error::Error for EventError {}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Battery(ref err) => {
                write!(f, "Battery Error: {:?}", err)
            }
            Self::Notification(ref err) => {
                write! {f, "Event Error: {}", err}
            }
        }
    }
}

impl From<BatteryError> for EventError {
    fn from(err: BatteryError) -> Self {
        Self::Battery(err)
    }
}

impl From<NotificationError> for EventError {
    fn from(err: NotificationError) -> Self {
        Self::Notification(err)
    }
}

#[derive(Debug)]
struct Manager {
    battery_info: battery::Info,
    threshold: u8,
}

/// Put the current thread to sleep for the specified amount of seconds.
fn sleep(secs: u64) {
    thread::sleep(time::Duration::from_secs(secs));
}

/// Loop for as long as battery percentage is lower than threshold.
///
/// `Manager` is refreshed every 30 seconds to check updated values.
fn below_threshold(manager: &mut Manager) -> Result<(), EventError> {
    while manager.battery_info.percentage < manager.threshold {
        manager.battery_info.refresh()?;
        sleep(30);
    }

    Ok(())
}

/// Loop for as long as battery percentage is higher than threshold.
///
/// `Manager` is refreshed every 30 seconds to check updated values.
///
/// Desktop notification is shown in every iteration while battery
/// state is `CHARGING`.
fn above_threshold(manager: &mut Manager) -> Result<(), EventError> {
    while manager.battery_info.percentage >= manager.threshold {
        let state = &manager.battery_info.state;
        if *state == battery::State::CHARGING {
            let handle =
                notification::notification(manager.battery_info.percentage)?;

            // If user unplugs charger we can close notification.
            sleep(5);
            if *state == battery::State::DISCHARGING {
                handle.close();
            }
        }

        manager.battery_info.refresh()?;
        sleep(30);
    }

    Ok(())
}

/// Loop to take care of battery charge threshold events.
pub fn event_loop(threshold: u8) -> Result<(), EventError> {
    let mut manager = Manager {
        battery_info: battery::Info::new()?,
        threshold,
    };

    loop {
        below_threshold(&mut manager)?;
        above_threshold(&mut manager)?;
    }
}
