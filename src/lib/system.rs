use notify_rust::Notification;
use tokio::process::Command;

pub struct System {}

impl System {
    pub async fn notify(title: &str, body: &str) -> anyhow::Result<()> {
        Notification::new()
            .summary(title)
            .body(body)
            .show_async()
            .await?;

        anyhow::Ok(())
    }

    pub async fn lock_screen() -> anyhow::Result<()> {
        #[cfg(target_os = "linux")]
        Command::new("loginctl")
            .arg("lock-session")
            .spawn()?
            .wait()
            .await?;

        anyhow::Ok(())
    }
}
