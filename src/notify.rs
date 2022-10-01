use std::convert::TryFrom;
use std::sync::mpsc;

use crate::device::Battery;
use crate::{common, error, event, notification};

pub fn notify(
    threshold: u8,
    model: Option<&str>,
    refresh_secs: u64,
    kde_connect_names: Option<Vec<String>>,
    disable_desktop: bool,
) -> error::Result<()> {
    validate_input(
        threshold,
        model,
        refresh_secs,
        &kde_connect_names,
        disable_desktop,
    )?;

    let mut battery_device = Battery::try_from(model)?;
    let mut notifier = notification::Notifier::new(
        threshold,
        kde_connect_names.map(common::vec_to_set),
        disable_desktop,
    )?;

    let (shutdown_sender, shutdown_receiver) = mpsc::channel();

    event::set_handler(shutdown_sender)?;

    event::loop_(
        &shutdown_receiver,
        &mut battery_device,
        &mut notifier,
        refresh_secs,
    )?;

    Ok(())
}

fn validate_input(
    _threshold: u8,
    _model: Option<&str>,
    _refresh_secs: u64,
    kde_connect_names: &Option<Vec<String>>,
    disable_desktop: bool,
) -> error::Result<()> {
    if disable_desktop && kde_connect_names.is_none() {
        return Err(error::Error::from(error::Notification::Config {
            kind: "both desktop and KDE connect can't be disabled".into(),
        }));
    };

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notify_notifications_disabled_kde_disabled() {
        let result = notify(0, None, 0, None, true);

        assert!(result.is_err());
        result.unwrap_or_else(|e| {
            assert!(matches!(
                e,
                error::Error::Notification(error::Notification::Config { .. })
            ));
        });
    }

    #[test]
    fn test_validate_validate_input_desktop_enabled_kde_disabled() {
        let result = validate_input(0, None, 0, &None, false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_validate_input_desktop_enabled_kde_empty() {
        let result = validate_input(0, None, 0, &Some(vec![]), false);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_validate_input_desktop_enabled_kde_populated() {
        let result = validate_input(
            0,
            None,
            0,
            &Some(vec!["a".into(), "5".into()]),
            false,
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_validate_input_desktop_disabled_kde_disabled() {
        let result = validate_input(0, None, 0, &None, true);

        assert!(result.is_err());
        result.unwrap_or_else(|e| {
            assert!(matches!(
                e,
                error::Error::Notification(error::Notification::Config { .. }),
            ));
        });
    }

    #[test]
    fn test_validate_validate_input_desktop_disabled_kde_empty() {
        let result = validate_input(0, None, 0, &Some(vec![]), true);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_validate_input_desktop_disabled_kde_populated() {
        let result = validate_input(
            0,
            None,
            0,
            &Some(vec!["a".into(), "5".into()]),
            true,
        );

        assert!(result.is_ok());
    }
}
