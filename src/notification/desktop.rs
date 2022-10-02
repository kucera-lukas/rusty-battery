use std::result;

use notify_rust::{Notification, NotificationHandle, Timeout, Urgency};

use crate::error;
use crate::notification::{Message, PlatformNotifier};

const APP_NAME: &str = "rusty-battery";
const ICON: &str = "battery";

type Result<T> = result::Result<T, error::Notification>;

#[derive(Debug)]
pub struct Notifier {
    handle: Option<NotificationHandle>,
}

impl PlatformNotifier for Notifier {
    type Error = error::Notification;

    fn notify(&mut self, message: &Message) -> result::Result<(), Self::Error> {
        self.show(message)?;

        Ok(())
    }

    fn remove(&mut self) -> result::Result<(), Self::Error> {
        self.close();

        Ok(())
    }
}

impl Notifier {
    /// Return a new `DesktopNotifier` instance.
    pub const fn new() -> Self {
        Self { handle: None }
    }

    /// Show a desktop notification that the battery threshold has been reached.
    ///
    /// If this is the first time this function is called
    /// a completely new notification is created and cached.
    ///
    /// If this function has previously been used there is no need
    /// to create a new `Notification` as we can just show the previously
    /// created one via it's `update` method.
    ///
    /// Return a reference to the current `NotificationHandle`.
    fn show(&mut self, message: &Message) -> Result<&NotificationHandle> {
        if let Some(handle) = &mut self.handle {
            handle.summary(&message.summary);
            handle.body(&message.body);

            handle.update();

            log::debug!("notification/desktop: cached notification shown");
        } else {
            self.handle = Some(
                create_notification(&message.summary, &message.body).show()?,
            );

            log::debug!("notification/desktop: notification shown and cached");
        }

        Ok(match self.handle.as_ref() {
            Some(handle) => handle,
            None => unreachable!(),
        })
    }

    /// Close the current desktop notification if it exists.
    ///
    /// If the `NotificationHandle` has not yet been created this is a noop.
    ///
    /// Return a bool whether the notification was closed.
    fn close(&mut self) -> bool {
        self.handle.take().map_or_else(
            || {
                log::debug!("notification/desktop: handle not yet created");

                false
            },
            |handle| {
                handle.close();

                log::debug!("notification/desktop: cached notification closed");

                true
            },
        )
    }
}

/// Create a new desktop notification with the given summary and body.
fn create_notification(summary: &str, body: &str) -> Notification {
    log::trace!(
        "notification/desktop: creating notification with \
        summary = \"{}\" and body = \"{}\"",
        summary,
        body,
    );

    Notification::new()
        .appname(APP_NAME)
        .summary(summary)
        .body(body)
        .icon(ICON)
        .timeout(Timeout::Never)
        .urgency(Urgency::Critical)
        .finalize()
}

mod std_fmt_impls {
    use std::fmt;

    use notify_rust::NotificationHandle;

    use crate::common;

    use super::Notifier;

    impl fmt::Display for Notifier {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "Desktop Notifier: handle = {}",
                common::format_option(
                    self.handle.as_ref().map(NotificationHandle::id)
                )
            )
        }
    }
} // std_fmt_impls

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use notify_rust::Hint;

    use super::*;

    fn assert_notification(
        notification: &Notification,
        summary: &str,
        body: &str,
    ) {
        let mut hints = HashSet::new();
        hints.insert(Hint::Urgency(Urgency::Critical));

        assert_eq!(notification.appname, APP_NAME);
        assert_eq!(notification.summary, summary);
        assert_eq!(notification.body, body);
        assert_eq!(notification.icon, ICON);
        assert_eq!(notification.timeout, Timeout::Never);
        assert_eq!(notification.hints, hints);
    }

    #[test]
    fn test_notifier_empty_handle() {
        let notifier = Notifier::new();

        assert!(notifier.handle.is_none());
    }

    #[test]
    fn test_notifier_display_none_handle() {
        let notifier = Notifier::new();

        let result = notifier.to_string();

        assert_eq!(result, "Desktop Notifier: handle = None");
    }

    #[test]
    fn test_create_notification() {
        let summary = "test-summary";
        let body = "test-body";

        let notification = create_notification(summary, body);

        assert_notification(&notification, summary, body);
    }
} // tests
