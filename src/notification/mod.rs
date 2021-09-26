mod desktop;

#[derive(Debug)]
pub struct Notifier {
    pub desktop: desktop::DesktopNotifier,
}

impl Notifier {
    /// Create a new `Notifier` instance.
    pub const fn new(threshold: u8) -> Self {
        Self {
            desktop: desktop::DesktopNotifier::new(threshold),
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifier_new() {
        let threshold = 0;
        let notifier = Notifier::new(threshold);

        assert_eq!(notifier.desktop.threshold, threshold);
    }

    #[test]
    fn test_notifier_display() {
        let threshold = 0;
        let notifier = Notifier::new(threshold);

        let display = format!("{}", notifier);

        assert_eq!(display, "desktop: threshold: 0, handle: None");
    }
}
