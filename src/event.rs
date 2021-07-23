use std::{thread, time};

use crate::{battery, notification};
use crate::{battery::BatteryError, notification::NotificationError};

#[derive(Debug)]
pub enum EventError {
    Battery(BatteryError),
    Notification(NotificationError),
}

impl From<BatteryError> for EventError {
    fn from(err: BatteryError) -> Self {
        EventError::Battery(err)
    }
}

impl From<NotificationError> for EventError {
    fn from(err: NotificationError) -> Self {
        EventError::Notification(err)
    }
}

#[derive(Debug)]
struct Manager {
    battery_info: battery::Info,
    threshold: i32,
}

fn sleep(secs: u64) {
    thread::sleep(time::Duration::from_secs(secs));
}

fn below_threshold(manager: &mut Manager) -> Result<(), EventError> {
    while manager.battery_info.percentage < manager.threshold {
        manager.battery_info.refresh()?;
        sleep(30);
    }

    Ok(())
}

fn above_threshold(manager: &mut Manager) -> Result<(), EventError> {
    while manager.battery_info.percentage >= manager.threshold {
        let state = &manager.battery_info.state;
        if *state == battery::State::CHARGING {
            let handle = notification::notification(manager.battery_info.percentage)?;

            // If user unplugs charger we can close notification ourselves.
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

pub fn event_loop(threshold: i32) -> Result<(), EventError> {
    let mut manager = Manager {
        battery_info: battery::Info::new()?,
        threshold,
    };

    loop {
        below_threshold(&mut manager)?;
        above_threshold(&mut manager)?;
    }
}
