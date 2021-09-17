use std::{result, thread, time};

use crate::error::EventError;
use crate::{battery, notification};

pub type Result<T> = result::Result<T, EventError>;

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
fn below_threshold(manager: &mut Manager) -> Result<()> {
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
fn above_threshold(manager: &mut Manager) -> Result<()> {
    while manager.battery_info.percentage >= manager.threshold {
        let state = &manager.battery_info.state;
        if *state == battery::State::Charging {
            let handle =
                notification::notification(manager.battery_info.percentage)?;

            // If user unplugs charger we can close notification.
            sleep(5);
            if *state == battery::State::Discharging {
                handle.close();
            }
        }

        manager.battery_info.refresh()?;
        sleep(30);
    }

    Ok(())
}

/// Loop to take care of battery charge threshold events.
pub fn event_loop(threshold: u8) -> Result<()> {
    let mut manager = Manager {
        battery_info: battery::Info::new()?,
        threshold,
    };

    loop {
        below_threshold(&mut manager)?;
        above_threshold(&mut manager)?;
    }
}
