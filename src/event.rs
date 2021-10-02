use std::result;
use std::thread;
use std::time;

use crate::application::App;
use crate::battery::{BatteryDevice, BatteryState};
use crate::error::BatteryError;
use crate::notification::Notifier;

type Result<T> = result::Result<T, BatteryError>;

/// Loop infinitely processing battery charge threshold events.
pub fn event_loop(app: &mut App) -> Result<()> {
    loop {
        below_threshold(app.settings.threshold, &mut app.battery_device)?;
        above_threshold(
            app.settings.threshold,
            &mut app.battery_device,
            &mut app.notifier,
        )?;
    }
}

/// Loop for as long as battery percentage is lower than threshold.
///
/// `BatteryInfo` is refreshed every 30 seconds to check updated values.
fn below_threshold(
    threshold: u8,
    battery_device: &mut BatteryDevice,
) -> Result<()> {
    let threshold = threshold;

    while battery_device.percentage < threshold {
        log::info!("battery is below the {}% threshold", threshold);

        sleep_and_refresh(30, battery_device)?;
    }

    Ok(())
}

/// Loop for as long as battery percentage is higher than threshold.
///
/// `BatteryInfo` is refreshed every 30 seconds to check updated values.
///
/// Desktop notification is shown in every iteration while battery
/// state is `CHARGING`.
fn above_threshold(
    threshold: u8,
    battery_device: &mut BatteryDevice,
    notifier: &mut Notifier,
) -> Result<()> {
    while battery_device.percentage >= threshold {
        log::info!("battery is above the {}% threshold", &threshold);

        if battery_device.state == BatteryState::Charging {
            notifier.notify();
        }

        sleep_and_refresh(30, battery_device)?;
    }

    Ok(())
}

/// Refresh given `BatteryInfo` instance and sleep for the given amount of seconds.
fn sleep_and_refresh(
    secs: u64,
    battery_device: &mut BatteryDevice,
) -> Result<()> {
    sleep(secs);
    battery_device.refresh()?;
    Ok(())
}

/// Put the current thread to sleep for the specified amount of seconds.
fn sleep(secs: u64) {
    log::debug!("sleeping for {} seconds", secs);
    thread::sleep(time::Duration::from_secs(secs));
}

#[cfg(test)]
mod tests {
    use std::time;

    use super::*;

    #[test]
    fn test_sleep() {
        let now = time::Instant::now();
        let secs = 1;

        sleep(secs);

        assert_eq!(now.elapsed().as_secs(), secs);
    }
}
