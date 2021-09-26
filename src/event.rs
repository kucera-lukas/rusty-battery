use std::thread;
use std::time;

use crate::application::App;
use crate::battery::{BatteryInfo, BatteryState};
use crate::notification::Notifier;

/// Loop infinitely processing battery charge threshold events.
pub fn event_loop(app: &mut App) {
    loop {
        below_threshold(app.settings.threshold, &mut app.battery_info);
        above_threshold(
            app.settings.threshold,
            &mut app.battery_info,
            &mut app.notifier,
        );
    }
}

/// Loop for as long as battery percentage is lower than threshold.
///
/// `BatteryInfo` is refreshed every 30 seconds to check updated values.
fn below_threshold(threshold: u8, battery_info: &mut BatteryInfo) {
    let threshold = threshold;

    while battery_info.percentage < threshold {
        log::info!("battery is below the {}% threshold", threshold);

        sleep_and_refresh(30, battery_info);
    }
}

/// Loop for as long as battery percentage is higher than threshold.
///
/// `BatteryInfo` is refreshed every 30 seconds to check updated values.
///
/// Desktop notification is shown in every iteration while battery
/// state is `CHARGING`.
fn above_threshold(
    threshold: u8,
    battery_info: &mut BatteryInfo,
    notifier: &mut Notifier,
) {
    while battery_info.percentage >= threshold {
        log::info!("battery is above the {}% threshold", &threshold);

        if battery_info.state == BatteryState::Charging {
            notifier.desktop.show();
        }

        sleep_and_refresh(30, battery_info);
    }
}

/// Refresh given `BatteryInfo` instance and sleep for the given amount of seconds.
fn sleep_and_refresh(secs: u64, battery_info: &mut BatteryInfo) {
    sleep(secs);
    battery_info.refresh();
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
