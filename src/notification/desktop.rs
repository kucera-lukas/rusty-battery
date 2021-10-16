use std::result;

use notify_rust::{Notification, NotificationHandle, Urgency};

use crate::common;
use crate::error;

type Result<T> = result::Result<T, error::Notification>;

#[derive(Debug)]
pub struct Notifier {
    threshold: u8,
    handle: Option<NotificationHandle>,
}

impl Notifier {
    /// Return a new `DesktopNotifier` instance.
    pub const fn new(threshold: u8) -> Self {
        Self {
            threshold,
            handle: None,
        }
    }

    /// Show a desktop notification alerting the user that the battery threshold has been reached.
    ///
    /// If this is the first time this function is called a completely new notification is created
    /// and cached.
    ///
    /// If this function has previously been used there is no need to create a new `Notification` as
    /// we can just show the previously created one via it's `update` method.
    ///
    /// Return a reference to the current `NotificationHandle`.
    pub fn show(&mut self) -> Result<&NotificationHandle> {
        if let Some(handle) = &mut self.handle {
            handle.update();

            log::debug!("cached desktop notification shown");
        } else {
            self.handle = Some(self.notification().show()?);

            log::debug!("desktop notification shown and cached");
        }
        Ok(self.handle.as_ref().expect("cached notification missing"))
    }

    /// Create a new desktop notification based on the battery threshold of the current instance.
    fn notification(&self) -> Notification {
        create_notification(
            "Charge limit warning",
            &common::warning_message(self.threshold),
        )
    }
}

/// Create a new desktop notification with the given summary and body.
fn create_notification(summary: &str, body: &str) -> Notification {
    Notification::new()
        .appname("rusty-battery")
        .summary(summary)
        .body(body)
        .icon("battery")
        .timeout(0)
        .urgency(Urgency::Critical)
        .finalize()
}

mod std_fmt_impls {
    use std::fmt;

    use super::Notifier;

    impl fmt::Display for Notifier {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Desktop: threshold = {}", self.threshold,)
        }
    }
}

#[cfg(test)]
mod tests {
    use notify_rust::{Hint, Timeout};

    use super::*;
    use std::collections::HashSet;

    fn assert_notification(
        notification: &Notification,
        summary: &str,
        body: &str,
    ) {
        let mut hints = HashSet::new();
        hints.insert(Hint::Urgency(Urgency::Critical));

        assert_eq!(notification.appname, "rusty-battery");
        assert_eq!(notification.summary, summary);
        assert_eq!(notification.body, body);
        assert_eq!(notification.icon, "battery");
        assert_eq!(notification.timeout, Timeout::Never);
        assert_eq!(notification.hints, hints);
    }

    #[test]
    fn test_desktop_notifier_empty_handle() {
        let notifier = Notifier::new(0);

        assert!(notifier.handle.is_none());
    }

    #[test]
    fn test_desktop_notifier_notification() {
        let notifier = Notifier::new(0);
        let notification = notifier.notification();

        assert_notification(&notification, "Charge limit warning", &format!(
            "Battery percentage reached the {}% threshold, please unplug your charger",
            notifier.threshold,
        ));
    }

    #[test]
    fn test_create_notification() {
        let summary = "test-summary";
        let body = "test-body";

        let notification = create_notification(summary, body);

        assert_notification(&notification, summary, body);
    }
}
