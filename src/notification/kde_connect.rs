use std::collections::HashSet;
use std::result;

use crate::common;
use crate::device::{kde_connect, KDEConnect};
use crate::error;
use crate::notification::PlatformNotifier;

type Result<T> = result::Result<T, error::KDEConnect>;

/// KDE Connect Notifier.
#[derive(Debug)]
pub struct Notifier {
    /// Charge threshold used to create warning message.
    threshold: u8,
    /// Names of KDE Connect devices which should be pinged.
    ///
    /// If this value is `None` every available KDE Connect device will pinged.
    device_names: Option<HashSet<String>>,
}

impl PlatformNotifier for Notifier {
    type Error = error::KDEConnect;

    fn notify(&mut self) -> result::Result<(), Self::Error> {
        self.ping()?;

        Ok(())
    }

    fn remove(&mut self) -> result::Result<(), Self::Error> {
        log::trace!("notification/kde_connect: remove noop");

        Ok(())
    }
}

impl Notifier {
    /// Create a new `KDEConnect` instance.
    pub fn new(threshold: u8, device_names: HashSet<String>) -> Result<Self> {
        let notifier = Self {
            threshold,
            device_names: if device_names.is_empty() {
                log::info!(
                    "notification/kde_connect: no device names specified, \
                    all available devices will be pinged",
                );

                None
            } else {
                log::info!(
                    "notification/kde_connect: will ping devices with names {}",
                    common::format_string_set(&device_names),
                );

                Some(device_names)
            },
        };

        // check KDE Connect CLI availability
        // also warns if some specified devices aren't available
        notifier.find_available()?;

        Ok(notifier)
    }

    /// Ping all available `Device` instances.
    fn ping(&self) -> Result<()> {
        self.find_available()?.iter().try_for_each(|device| {
            device.ping(&common::warning_message(self.threshold))
        })?;

        log::debug!("notification/kde_connect: available devices pinged");

        Ok(())
    }

    /// Return all `Device` instances which are currently available.
    ///
    /// If no `device_names` were specified at the creation,
    /// all available devices will be returned.
    fn find_available(&self) -> Result<Vec<KDEConnect>> {
        let mut devices = kde_connect::map::available()?;

        Ok(match &self.device_names {
            None => devices.into_values().collect(),
            Some(names) => kde_connect::find::all(&mut devices, names),
        })
    }
}

mod std_fmt_impls {
    use std::borrow::Borrow;
    use std::fmt;

    use super::Notifier;

    impl fmt::Display for Notifier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let device_names = self.device_names.as_ref().map_or_else(
                || "All".into(),
                |names| {
                    format!(
                        "[{}]",
                        names
                            .iter()
                            .map(Borrow::borrow)
                            .collect::<Vec<&str>>()
                            .join(", ")
                    )
                },
            );

            write!(
                f,
                "KDE Connect Notifier: \
                threshold = {}%, device_names = {device_names}",
                self.threshold,
            )
        }
    }
} // std_fmt_impls
