use std::path::PathBuf;

use crate::{config::Config, prayer_manager::PrayerManager};
use directories::BaseDirs;
use notify_rust::Notification;
use salah::{DateTime, Utc};
use tokio::{process::Command, time::Instant};

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

pub async fn sleep_until_datetime(time: DateTime<Utc>) {
    let now = Utc::now();
    if time > now {
        if let Ok(duration) = (time - now).to_std() {
            println!("Sleeping for {duration:?}");
            tokio::time::sleep_until(Instant::now() + duration).await;
        }
    }
}

pub fn reload() -> (PrayerManager, Config) {
    let config_path = get_config_path().unwrap();
    let config = Config::from_file(config_path).expect("Couldn't parse Configuration File");
    let prayer_manager = PrayerManager::new(&config);

    (prayer_manager, config)
}

pub fn get_config_path() -> Option<PathBuf> {
    Some(
        BaseDirs::new()?
            .config_dir()
            .join("pryr")
            .join("config.toml"),
    )
}

pub fn get_socket_path() -> Option<PathBuf> {
    Some(BaseDirs::new()?.runtime_dir()?.join("pryr.sock"))
}
