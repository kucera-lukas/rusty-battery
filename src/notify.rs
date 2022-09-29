use std::convert::TryFrom;
use std::sync::mpsc;

use clap::error::ErrorKind;
use clap::CommandFactory;

use crate::battery::Device;
use crate::{cli, common, error, event, notification};

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
    );

    let mut battery_device = Device::try_from(model)?;
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
) {
    if disable_desktop && kde_connect_names.is_none() {
        let mut cmd = cli::Cli::command();

        cmd.error(
            ErrorKind::ValueValidation,
            "Both desktop and KDE connect notifications are disabled",
        )
        .exit();
    };
}
