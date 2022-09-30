use std::collections::HashSet;
use std::result;

pub use desktop::Notifier as DesktopNotifier;
pub use kde_connect::Notifier as KDEConnectNotifier;

use crate::common;
use crate::error;

mod desktop;
mod kde_connect;

type Result<T> = result::Result<T, error::Error>;

pub trait PlatformNotifier {
    type Error: std::error::Error;

    fn notify(&mut self) -> result::Result<(), Self::Error> {
        Ok(())
    }

    fn remove(&mut self) -> result::Result<(), Self::Error> {
        Ok(())
    }
}

#[derive(Debug)]
pub struct Notifier {
    pub threshold: u8,

    desktop: Option<DesktopNotifier>,
    kde_connect: Option<KDEConnectNotifier>,
}

impl Notifier {
    /// Create a new `Notifier` instance.
    pub fn new(
        threshold: u8,
        kde_connect_names: Option<HashSet<String>>,
        disable_desktop: bool,
    ) -> Result<Self> {
        log::info!("notification: threshold set to {threshold}%");

        let desktop = if disable_desktop {
            log::info!("notification: desktop notifications disabled");

            None
        } else {
            log::info!("notification: desktop notifications enabled");

            Some(DesktopNotifier::new(threshold))
        };

        let kde_connect: Result<Option<KDEConnectNotifier>> = kde_connect_names
            .map_or_else(
                || {
                    log::info!(
                        "notification: KDE Connect notifications disabled"
                    );

                    Ok(None)
                },
                |names| {
                    log::info!(
                        "notification: KDE Connect notifications enabled"
                    );

                    Ok(Some(KDEConnectNotifier::new(threshold, names)?))
                },
            );

        Ok(Self {
            threshold,
            desktop,
            kde_connect: kde_connect?,
        })
    }

    /// Send notification to every supported platform.
    pub fn notify(&mut self) {
        do_notify(&mut self.desktop);
        do_notify(&mut self.kde_connect);

        log::info!("notification: all sent");
    }

    /// Remove notification on every supported platform.
    pub fn remove(&mut self) {
        do_remove(&mut self.desktop);
        do_remove(&mut self.kde_connect);

        log::info!("notification: all removed");
    }
}

fn do_notify(notifier: &mut Option<impl PlatformNotifier>) {
    if let Some(notifier) = notifier {
        common::warn_on_err("notification", notifier.notify());
    }
}

fn do_remove(notifier: &mut Option<impl PlatformNotifier>) {
    if let Some(notifier) = notifier {
        common::warn_on_err("notification", notifier.remove());
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
                "Notifier: Desktop = {}, KDE Connect = {}",
                common::format_option(&self.desktop),
                common::format_option(&self.kde_connect),
            )
        }
    }
} // std_fmt_impls
