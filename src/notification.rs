use std::result;

use notify_rust::{Hint, Notification, NotificationHandle, Urgency};

pub type Result<T> = result::Result<T, notify_rust::error::Error>;

/// Show a desktop notification.
///
/// Notification lets the user know that battery percentage
/// already surpassed the specified threshold.
///
/// Return the handle to the new notification.
pub fn notification(battery_percentage: u8) -> Result<NotificationHandle> {
    let handle = Notification::new()
        .summary("Charge limit warning")
        .body(&format!(
            "Battery percentage already at {}%, you might want to unplug your charger",
            battery_percentage,
        ))
        .icon("administration")
        .appname("rusty-battery")
        .hint(Hint::Category("device".to_owned()))
        .urgency(Urgency::Critical)
        .timeout(0)
        .finalize()
        .show()?;

    log::info!(
        "created desktop notification with percentage = {}%",
        battery_percentage,
    );

    Ok(handle)
}
