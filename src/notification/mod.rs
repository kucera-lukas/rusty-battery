pub mod desktop;
pub mod kde_connect;

use crate::common;
use crate::error::Result;

#[derive(Debug)]
pub struct Notifier {
    desktop: Option<desktop::Notifier>,
    kde_connect: Option<kde_connect::Notifier>,
}

impl Notifier {
    /// Create a new `Notifier` instance.
    pub fn new(
        threshold: u8,
        kde_connect_names: Option<Vec<String>>,
    ) -> Result<Self> {
        let desktop = Some(desktop::Notifier::new(threshold));
        let kde_connect = {
            if let Some(names) = kde_connect_names {
                Some(kde_connect::Notifier::new(threshold, &names)?)
            } else {
                None
            }
        };

        let result = Self {
            desktop,
            kde_connect,
        };

        log::info!("{}", result);

        Ok(result)
    }

    /// Send notification on every platform.
    pub fn notify(&mut self) {
        if let Some(desktop) = &mut self.desktop {
            common::warn_on_err(desktop.show());
        }
        if let Some(kde_connect) = &self.kde_connect {
            common::warn_on_err(kde_connect.ping());
        }

        log::info!("all notifications sent");
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
