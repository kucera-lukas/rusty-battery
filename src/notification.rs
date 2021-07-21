use notify_rust::{Hint, Notification};
use std::fmt;

#[derive(Debug)]
pub enum NotificationError {
    Notify(notify_rust::error::Error),
}

impl fmt::Display for NotificationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NotificationError::Notify(ref err) => write!(f, "NotifyRust Error: {}", err),
        }
    }
}

impl From<notify_rust::error::Error> for NotificationError {
    fn from(err: notify_rust::error::Error) -> Self {
        NotificationError::Notify(err)
    }
}

pub fn notify(battery_percentage: i32) -> Result<(), NotificationError> {
    Notification::new()
        .summary("Charge limit warning")
        .body(&format!(
            "Battery percentage already at {}%, you might want to unplug your charger",
            battery_percentage.to_string()
        ))
        .icon("administration")
        .appname("rusty-battery")
        .hint(Hint::Category("device".to_owned()))
        .timeout(0)
        .show()?;

    Ok(())
}
