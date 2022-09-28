use std::sync::mpsc;
use std::sync::mpsc::RecvTimeoutError;
use std::time;
use std::{process, result};

use crate::battery;
use crate::error;
use crate::notification::Notifier;

type Result<T> = result::Result<T, error::Error>;

/// Loop infinitely processing battery charge threshold events.
pub fn loop_(
    shutdown_receiver: &mpsc::Receiver<()>,
    battery_device: &mut battery::Device,
    notifier: &mut Notifier,
    refresh_secs: u64,
) -> Result<()> {
    log::info!(
        "event: starting loop with {} seconds refresh interval",
        refresh_secs,
    );

    let refresh_duration = time::Duration::from_secs(refresh_secs);

    loop {
        if battery_device.percentage >= notifier.threshold
            && battery_device.state == battery::State::Charging
        {
            notifier.notify();
        } else {
            notifier.remove();
        }

        wait_and_refresh(
            shutdown_receiver,
            battery_device,
            notifier,
            refresh_duration,
        )?;
    }
}

/// Register signal handler for SIGINT, SIGTERM and SIGHUP.
///
/// The handling thread sends a value to a channel via the given `Sender`.
pub fn set_handler(shutdown_sender: mpsc::Sender<()>) -> Result<()> {
    ctrlc::set_handler(move || {
        log::info!("event: got signal, exiting...");

        shutdown_sender.send(()).unwrap_or_else(|e| {
            log::error!("event: {}", e);

            process::exit(1);
        });
    })
    .map_err(|e| error::Error::System(error::System::Handler(e)))?;

    Ok(())
}

/// Wait on the given `Receiver` and refresh the given battery `Device`.
///
/// If `Receiver` receives a value within the given `Duration` shutdown the process.
/// If the `Receiver` times out refresh the given `Device` and return.
/// If the other half of the `Receiver` channel gets disconnected return error.
fn wait_and_refresh(
    shutdown_receiver: &mpsc::Receiver<()>,
    battery_device: &mut battery::Device,
    notifier: &mut Notifier,
    refresh_duration: time::Duration,
) -> Result<()> {
    match shutdown_receiver.recv_timeout(refresh_duration) {
        Ok(_) => {
            handle_shutdown(notifier);

            Ok(())
        }
        Err(e) => match e {
            RecvTimeoutError::Timeout => {
                log::trace!("event: {}", e);

                battery_device.refresh()?;

                Ok(())
            }
            RecvTimeoutError::Disconnected => {
                log::error!("event: {}", e);

                Err(error::Error::System(error::System::RecvTimeout(e)))
            }
        },
    }
}

/// Handle shutdown by removing all notifications and terminating the current process.
fn handle_shutdown(notifier: &mut Notifier) {
    notifier.remove();

    log::debug!("event: terminating current process");

    process::exit(0);
}
