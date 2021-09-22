use std::result;

use crate::battery::{BatteryInfo, ProvideBatteryData};
use crate::error::AppError;
use crate::notification::desktop::ProvideDesktopNotification;
use crate::notification::Notifier;

pub type Result<T> = result::Result<T, AppError>;

pub struct App<B, D>
where
    B: ProvideBatteryData,
    D: ProvideDesktopNotification,
{
    pub settings: UserSettings,
    pub battery: BatteryInfo<B>,
    pub notifier: Notifier<D>,
}

impl<B, D> App<B, D>
where
    B: ProvideBatteryData,
    D: ProvideDesktopNotification,
{
    pub fn new(
        verbose: u8,
        threshold: u8,
        battery_provider: B,
        desktop_notifier: D,
    ) -> Result<Self> {
        let app = Self {
            settings: UserSettings { verbose, threshold },
            battery: BatteryInfo::new(battery_provider)?,
            notifier: Notifier::new(desktop_notifier),
        };

        Ok(app)
    }
}

pub struct UserSettings {
    pub verbose: u8,
    pub threshold: u8,
}
