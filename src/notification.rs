use notify_rust::{Hint, Notification, NotificationHandle, Urgency};
use std::{error, fmt};

#[derive(Debug)]
pub enum NotificationError {
    Notify(notify_rust::error::Error),
}

impl error::Error for NotificationError {}

impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Notify(ref err) => {
                write!(f, "NotifyRust Error: {}", err)
            }
        }
    }
}

impl From<notify_rust::error::Error> for NotificationError {
    fn from(err: notify_rust::error::Error) -> Self {
        Self::Notify(err)
    }
}

pub fn notification(
    battery_percentage: i32,
) -> Result<NotificationHandle, NotificationError> {
    let handle = Notification::new()
        .summary("Charge limit warning")
        .body(&format!(
            "Battery percentage already at {}%, you might want to unplug your charger",
            battery_percentage.to_string()
        ))
        .icon("administration")
        .appname("rusty-battery")
        .hint(Hint::Category("device".to_owned()))
        .urgency(Urgency::Critical)
        .timeout(0)
        .finalize()
        .show()?;

    Ok(handle)
}
