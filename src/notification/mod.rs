pub mod desktop;
pub mod kde_connect;

use std::convert::TryFrom;

use crate::common;
use crate::error::{Error, Result};

#[derive(Debug)]
pub struct Notifier {
    desktop: desktop::Notifier,
    kde_connect: kde_connect::Notifier,
}

impl Notifier {
    /// Create a new `Notifier` instance.
    pub fn new(threshold: u8) -> Result<Self> {
        Ok(Self {
            desktop: desktop::Notifier::new(threshold),
            kde_connect: kde_connect::Notifier::new(threshold)?,
        })
    }

    /// Send notification on every platform.
    pub fn notify(&mut self) {
        common::warn_on_err(self.desktop.show());
        common::warn_on_err(self.kde_connect.ping());

        log::info!("all notifications sent");
    }
}

impl TryFrom<u8> for Notifier {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        Self::new(value)
    }
}

mod std_fmt_impls {
    use std::fmt;

    use super::Notifier;

    impl fmt::Display for Notifier {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "desktop: {}", self.desktop)
        }
    }
}
