use std::result;
use std::thread;
use std::time;

use crate::application::App;
use crate::battery::{BatteryInfo, BatteryState, ProvideBatteryData};
use crate::error::AppError;
use crate::notification::desktop::ProvideDesktopNotification;

pub type Result<T> = result::Result<T, AppError>;

/// Put the current thread to sleep for the specified amount of seconds.
fn sleep(secs: u64) {
    log::debug!("sleeping for {} seconds", secs);
    thread::sleep(time::Duration::from_secs(secs));
}

/// Refresh given `Manager` instance and sleep for the given amount of seconds.
fn sleep_and_refresh(
    battery: &mut BatteryInfo<impl ProvideBatteryData>,
    secs: u64,
) -> Result<()> {
    sleep(secs);
    battery.refresh()?;
    Ok(())
}

/// Loop for as long as battery percentage is lower than threshold.
///
/// `Manager` is refreshed every 30 seconds to check updated values.
fn below_threshold(
    app: &mut App<impl ProvideBatteryData, impl ProvideDesktopNotification>,
) -> Result<()> {
    while app.battery.percentage < app.settings.threshold {
        log::info!(
            "battery is below the threshold {}%",
            app.settings.threshold
        );

        sleep_and_refresh(&mut app.battery, 30)?;
    }

    Ok(())
}

/// Loop for as long as battery percentage is higher than threshold.
///
/// `Manager` is refreshed every 30 seconds to check updated values.
///
/// Desktop notification is shown in every iteration while battery
/// state is `CHARGING`.
fn above_threshold(
    app: &mut App<impl ProvideBatteryData, impl ProvideDesktopNotification>,
) -> Result<()> {
    while app.battery.percentage >= app.settings.threshold {
        log::info!(
            "battery is above the threshold {}%",
            &app.settings.threshold
        );

        let state = &app.battery.state;

        if *state == BatteryState::Charging {
            app.notifier.desktop.above_threshold(app.settings.threshold);
        }

        sleep_and_refresh(&mut app.battery, 30)?;
    }

    Ok(())
}

/// Loop to take care of battery charge threshold events.
pub fn event_loop(
    app: &mut App<impl ProvideBatteryData, impl ProvideDesktopNotification>,
) -> Result<()> {
    loop {
        below_threshold(app)?;
        above_threshold(app)?;
    }
}
