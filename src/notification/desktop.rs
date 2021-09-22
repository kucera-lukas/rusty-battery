use notify_rust::{Hint, Notification, NotificationHandle, Urgency};

pub trait ProvideDesktopNotification {
    fn notify_above_threshold(
        &self,
        battery_percentage: u8,
    ) -> Option<NotificationHandle>;
}

pub struct DesktopNotificationProvider;

impl ProvideDesktopNotification for DesktopNotificationProvider {
    /// Notify the user that battery percentage reached the given threshold.
    ///
    /// Return a handle to the newly created desktop notification.
    fn notify_above_threshold(
        &self,
        battery_percentage: u8,
    ) -> Option<NotificationHandle> {
        let notification = Notification::new()
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
            .finalize();

        let handle: Option<NotificationHandle> = match notification.show() {
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
        };

        handle
    }
}

impl DesktopNotificationProvider {
    /// Create a new `DesktopNotificationProvider` instance.
    pub const fn new() -> Self {
        Self {}
    }
}
