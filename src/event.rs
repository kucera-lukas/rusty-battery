use std::result;
use std::thread;
use std::time;

use crate::battery;
use crate::error;
use crate::notification::Notifier;

type Result<T> = result::Result<T, error::Battery>;

/// Loop infinitely processing battery charge threshold events.
pub fn loop_(
    threshold: u8,
    battery_device: &mut battery::Device,
    notifier: &mut Notifier,
) -> Result<()> {
    loop {
        if battery_device.percentage >= threshold
            && battery_device.state == battery::State::Charging
        {
            notifier.notify();
        }

        sleep_and_refresh(30, battery_device)?;
    }
}

/// Refresh given `BatteryInfo` instance and sleep for the given amount of seconds.
fn sleep_and_refresh(
    secs: u64,
    battery_device: &mut battery::Device,
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
