use notify_rust::{Notification, Timeout};

pub struct NotificationService;

impl NotificationService {
    pub fn send_timer_complete() -> Result<(), notify_rust::error::Error> {
        Notification::new()
            .summary("Ramen Timer")
            .body("🍜 Your ramen is ready!")
            .icon("dialog-information")
            .timeout(Timeout::Never)
            .show()?;
        Ok(())
    }
}