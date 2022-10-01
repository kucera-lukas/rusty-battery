use crate::common;
use crate::notification::{Message, PlatformNotifier};

pub(super) fn notify<N>(notifier: &mut Option<N>, message: &Message)
where
    N: PlatformNotifier,
{
    if let Some(notifier) = notifier {
        common::warn_on_err("notification", notifier.notify(message));
    }
}

pub(super) fn remove<N>(notifier: &mut Option<N>)
where
    N: PlatformNotifier,
{
    if let Some(notifier) = notifier {
        common::warn_on_err("notification", notifier.remove());
    }
}
