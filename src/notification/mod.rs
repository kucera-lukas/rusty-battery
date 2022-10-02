use std::collections::HashSet;
use std::result;

pub use desktop::Notifier as DesktopNotifier;
pub use kde_connect::Notifier as KDEConnectNotifier;
pub use message::Message;

use crate::error;

mod desktop;
mod kde_connect;
mod message;
mod operation;

type Result<T> = result::Result<T, error::Error>;

pub trait PlatformNotifier {
    type Error: std::error::Error;

    fn notify(&mut self, message: &Message) -> result::Result<(), Self::Error>;

    fn remove(&mut self) -> result::Result<(), Self::Error>;
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

            Some(DesktopNotifier::new())
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

                    Ok(Some(KDEConnectNotifier::new(names)?))
                },
            );

        Ok(Self {
            threshold,
            desktop,
            kde_connect: kde_connect?,
        })
    }

    /// Send notification to every supported platform.
    pub fn notify(&mut self, message: &Message) {
        operation::notify(&mut self.desktop, message);
        operation::notify(&mut self.kde_connect, message);

        log::info!("notification: all sent");
    }

    /// Remove notification on every supported platform.
    pub fn remove(&mut self) {
        operation::remove(&mut self.desktop);
        operation::remove(&mut self.kde_connect);

        log::info!("notification: all removed");
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
                common::format_option(self.desktop.as_ref()),
                common::format_option(self.kde_connect.as_ref()),
            )
        }
    }
} // std_fmt_impls
