pub use self::battery::{Battery, State as BatteryState};
pub use self::kde_connect::KDEConnect;
use crate::{common, error};
use std::fmt::Display;

pub mod battery;
pub mod kde_connect;

#[derive(Debug, Copy, Eq, PartialEq, Clone)]
pub enum Type {
    Battery,
    KDEConnect,
}

impl Type {
    /// Print all available Devices formatted in a readable way.
    ///
    /// Acts as an high level API for the CLI
    /// `Batteries` and `KDEConnectDevices` subcommands.
    pub fn print(self) -> Result<(), error::Error> {
        match self {
            Self::Battery => print_devices(self, &battery::all()?),
            Self::KDEConnect => print_devices(
                self,
                &kde_connect::map::all()?
                    .into_values()
                    .collect::<Vec<KDEConnect>>(),
            ),
        };

        Ok(())
    }
}

fn print_devices<D>(title: Type, slice: &[D])
where
    D: Display,
{
    println!();
    println!("{title} Devices");
    println!();

    common::print_slice(slice);
}

mod std_fmt_impls {
    use std::fmt;

    use super::Type;

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Battery => write!(f, "Battery"),
                Self::KDEConnect => write!(f, "KDE Connect"),
            }
        }
    }
} // std_fmt_impls

#[cfg(test)]
mod tests {
    use super::Type;

    #[test]
    fn test_type_battery_display() {
        let state = Type::Battery;

        let display = format!("{state}");

        assert_eq!(display, "Battery");
    }

    #[test]
    fn test_type_kdeconnect_display() {
        let state = Type::KDEConnect;

        let display = format!("{state}");

        assert_eq!(display, "KDE Connect");
    }
} // tests
