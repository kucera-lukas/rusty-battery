//! Battery information.

use std::result;

use crate::error::BatteryError;

pub type Result<T> = result::Result<T, BatteryError>;

#[derive(Debug, PartialEq)]
pub enum BatteryState {
    Charging,
    Discharging,
}

pub trait ProvideBatteryData {
    /// Return current battery percentage.
    fn percentage(&self) -> u8;
    /// Return current battery state as the `BatteryState` enum.
    fn state(&self) -> Result<BatteryState>;
}

#[derive(Debug)]
pub struct BatteryDataProvider {
    device: battery::Battery,
}

impl ProvideBatteryData for BatteryDataProvider {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn percentage(&self) -> u8 {
        let percentage = self
            .device
            .state_of_charge()
            .get::<battery::units::ratio::percent>()
            .trunc() as u8;

        log::debug!("current battery percentage = {}%", percentage);

        percentage
    }

    fn state(&self) -> Result<BatteryState> {
        let state = self.device.state();

        let result = match state {
            battery::State::Charging | battery::State::Full => {
                Ok(BatteryState::Charging)
            }
            battery::State::Discharging | battery::State::Empty => {
                Ok(BatteryState::Discharging)
            }
            _ => Err(BatteryError::UnknownState { state }),
        };

        if let Ok(state) = &result {
            log::debug!("current battery state = {}", state);
        };

        result
    }
}

impl BatteryDataProvider {
    /// Create a new instance of `BatteryDataProvider` via the `battery::Battery` device object.
    pub fn new() -> Result<Self> {
        let manager = battery::Manager::new()?;
        let device = manager
            .batteries()?
            .next()
            .ok_or(BatteryError::DeviceError)??;

        Ok(Self { device })
    }
}

#[derive(Debug)]
pub struct BatteryInfo<B: ProvideBatteryData> {
    pub percentage: u8,
    pub state: BatteryState,

    provider: B,
}

impl<B> BatteryInfo<B>
where
    B: ProvideBatteryData,
{
    /// Construct a new `Info` instance.
    pub fn new(battery_provider: B) -> Result<Self> {
        let info = Self {
            percentage: battery_provider.percentage(),
            state: battery_provider.state()?,
            provider: battery_provider,
        };
        Ok(info)
    }

    /// Update attributes to current battery values.
    pub fn refresh(&mut self) -> Result<&mut Self> {
        self.percentage = self.provider.percentage();
        self.state = self.provider.state()?;

        log::info!("refreshed: {}", self);

        Ok(self)
    }
}

mod std_fmt_impls {
    use std::fmt;

    use super::{BatteryInfo, BatteryState, ProvideBatteryData};

    impl fmt::Display for BatteryState {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                Self::Charging => write!(f, "Charging"),
                Self::Discharging => write!(f, "Discharging"),
            }
        }
    }

    impl<B> fmt::Display for BatteryInfo<B>
    where
        B: ProvideBatteryData,
    {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "battery percentage: {}%, state: {}",
                self.percentage, self.state,
            )
        }
    }
}
