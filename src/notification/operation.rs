use crate::common;
use crate::notification::PlatformNotifier;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Operation {
    Notify,
    Remove,
}

pub(super) fn notify<N>(notifier: &mut Option<N>)
where
    N: PlatformNotifier,
{
    operation(notifier, Operation::Notify);
}

pub(super) fn remove<N>(notifier: &mut Option<N>)
where
    N: PlatformNotifier,
{
    operation(notifier, Operation::Remove);
}

fn operation<N>(notifier: &mut Option<N>, op: Operation)
where
    N: PlatformNotifier,
{
    if let Some(notifier) = notifier {
        common::warn_on_err(
            "notification",
            match op {
                Operation::Notify => notifier.notify(),
                Operation::Remove => notifier.remove(),
            },
        );
    }
}
