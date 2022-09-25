pub mod desktop;
pub mod kde_connect;

use std::collections::HashSet;

use crate::common;

#[derive(Debug)]
pub struct Notifier {
    pub threshold: u8,

    desktop: Option<desktop::Notifier>,
    kde_connect: Option<kde_connect::Notifier>,
}

impl Notifier {
    /// Create a new `Notifier` instance.
    pub fn new(
        threshold: u8,
        kde_connect_names: Option<HashSet<String>>,
        disable_desktop: bool,
    ) -> Self {
        Self {
            threshold,
            desktop: if disable_desktop {
                log::info!(
                    "notification/Notifier: desktop notifications disabled",
                );

                None
            } else {
                log::info!(
                    "notification/Notifier: desktop notifications enabled",
                );

                Some(desktop::Notifier::new(threshold))
            },
            kde_connect: kde_connect_names
                .map(|names| kde_connect::Notifier::new(threshold, names)),
        }
    }

    /// Send notification to every supported platform.
    pub fn notify(&mut self) {
        if let Some(desktop) = &mut self.desktop {
            common::warn_on_err(desktop.show());
        }
        if let Some(kde_connect) = &self.kde_connect {
            common::warn_on_err(kde_connect.ping());
        }

        log::info!("notification/Notifier: all notifications sent");
    }

    /// Remove notification on every supported platform.
    ///
    /// Currently only desktop notifier is supported.
    ///
    /// KDE Connect notifier is not supported as we can't remove the pinged notification from the
    /// device.
    pub fn remove(&mut self) {
        if let Some(desktop) = &mut self.desktop {
            desktop.close();
        }

        log::info!("notification/Notifier: all notifications removed");
    }
}

mod std_fmt_impls {
    use std::fmt;

    use crate::common;

    use super::Notifier;

    impl fmt::Display for Notifier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "Notifier: desktop = {}, KDE Connect = {}",
                common::format_option(&self.desktop),
                common::format_option(&self.kde_connect),
            )
        }
    }
}
