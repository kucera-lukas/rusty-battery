use crate::battery::BatteryInfo;
use crate::notification::Notifier;

#[derive(Debug)]
pub struct UserSettings {
    pub verbose: u8,
    pub threshold: u8,
}

#[derive(Debug)]
pub struct App {
    pub settings: UserSettings,
    pub battery_info: BatteryInfo,
    pub notifier: Notifier,
}

impl App {
    pub fn new(verbose: u8, threshold: u8, model: Option<&str>) -> Self {
        Self {
            settings: UserSettings { verbose, threshold },
            battery_info: BatteryInfo::new(model),
            notifier: Notifier::new(threshold),
        }
    }
}

mod std_fmt_impls {
    use std::fmt;

    use super::{App, UserSettings};

    impl fmt::Display for UserSettings {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "verbose: {}, threshold: {}",
                self.threshold, self.threshold,
            )
        }
    }

    impl fmt::Display for App {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "settings: {}, battery_info: {}, notifier: {}",
                self.settings, self.battery_info, self.notifier,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_settings_display() {
        let settings = UserSettings {
            verbose: 0,
            threshold: 0,
        };

        let display = format!("{}", settings);

        assert_eq!(display, "verbose: 0, threshold: 0");
    }
}
