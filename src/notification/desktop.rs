use notify_rust::{Notification, NotificationHandle, Urgency};

#[derive(Debug)]
pub struct DesktopNotifier {
    pub threshold: u8,

    handle: Option<NotificationHandle>,
}

impl DesktopNotifier {
    /// Return a new `DesktopNotifier` instance.
    pub const fn new(threshold: u8) -> Self {
        Self {
            threshold,
            handle: None,
        }
    }

    pub fn show(&mut self) -> Option<&NotificationHandle> {
        if let Some(handle) = &mut self.handle {
            // No need to create new `Notification` as we can just show
            // the previously created one via it's `update` method.
            handle.update();
            log::debug!("reused previously defined desktop notification.");
        } else {
            match self.notification().show() {
                Ok(handle) => {
                    self.handle = Some(handle);
                }
                Err(e) => {
                    log::warn!("showing desktop notification failed: {}", e);
                }
            }
        }

        if self.handle.is_some() {
            log::info!("desktop notification shown.");
        }

        self.handle.as_ref()
    }

    fn notification(&self) -> Notification {
        create_notification("Charge limit warning", &format!(
            "Battery percentage reached the {}% threshold, please unplug your charger",
            &self.threshold,
        ))
    }
}

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

    use super::DesktopNotifier;

    impl fmt::Display for DesktopNotifier {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "threshold: {}, handle: {:?}",
                self.threshold, self.handle
            )
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
        let notifier = DesktopNotifier::new(0);

        assert!(notifier.handle.is_none());
    }

    #[test]
    fn test_desktop_notifier_notification() {
        let notifier = DesktopNotifier::new(0);
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

    #[test]
    fn test_desktop_notifier_display() {
        let notifier = DesktopNotifier::new(0);

        let display = format!("{}", notifier);

        assert_eq!(display, "threshold: 0, handle: None");
    }
}
