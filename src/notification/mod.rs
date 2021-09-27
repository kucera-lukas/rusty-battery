mod desktop;
pub mod kde_connect;

#[derive(Debug)]
pub struct Notifier {
    desktop: desktop::DesktopNotifier,
    kde_connect: kde_connect::KDENotifier,
}

impl Notifier {
    /// Create a new `Notifier` instance.
    pub fn new(threshold: u8) -> Self {
        Self {
            desktop: desktop::DesktopNotifier::new(threshold),
            kde_connect: kde_connect::KDENotifier::new(threshold),
        }
    }

    /// Send notification on every platform.
    pub fn notify(&mut self) {
        self.desktop.show();
        self.kde_connect.ping();
        log::info!("all notifications sent");
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
