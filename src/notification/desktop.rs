use std::result;

use notify_rust::{Hint, Notification, NotificationHandle, Urgency};

pub type Result<T> = result::Result<T, notify_rust::error::Error>;

pub trait ProvideDesktopNotification {
    /// Show a new desktop `Notification`.
    fn notification(&self, body: &str) -> Result<NotificationHandle>;
    /// Notify the user that battery percentage reached the given threshold.
    ///
    /// If showing notification succeeds, return its `NotificationHandle`.
    fn above_threshold(
        &self,
        battery_percentage: u8,
    ) -> Option<NotificationHandle>;
}

#[derive(Debug)]
pub struct DesktopNotificationProvider;

impl ProvideDesktopNotification for DesktopNotificationProvider {
    fn notification(&self, body: &str) -> Result<NotificationHandle> {
        let notification = Notification::new()
            .summary("Charge limit warning")
            .body(body)
            .icon("administration")
            .appname("rusty-battery")
            .hint(Hint::Category("device".to_owned()))
            .urgency(Urgency::Critical)
            .timeout(0)
            .finalize();

        notification.show()
    }

    fn above_threshold(
        &self,
        battery_percentage: u8,
    ) -> Option<NotificationHandle> {
        let body = format!(
            "Battery percentage already at {}%, you might want to unplug your charger",
            battery_percentage,
        );

        match self.notification(&body) {
            Ok(handle) => {
                log::info!(
                    "showing desktop notification with battery percentage = {}%",
                    battery_percentage,
                );
                Some(handle)
            }
            Err(e) => {
                log::warn!("showing desktop notification failed: {}", e);
                None
            }
        }
    }
}

impl DesktopNotificationProvider {
    /// Create a new `DesktopNotificationProvider` instance.
    pub const fn new() -> Self {
        Self {}
    }
}
