pub mod desktop;

pub struct Notifier<D: desktop::ProvideDesktopNotification> {
    pub desktop: D,
}

impl<D> Notifier<D>
where
    D: desktop::ProvideDesktopNotification,
{
    pub fn new(desktop_notifier: D) -> Self {
        Self {
            desktop: desktop_notifier,
        }
    }
}
