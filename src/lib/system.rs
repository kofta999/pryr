use crate::{config::Config, prayers::PrayerManager};
use directories::BaseDirs;
use notify_rust::Notification;
use salah::{DateTime, Utc};
use std::path::PathBuf;
use tokio::time::Instant;

pub async fn notify(title: &str, body: &str) -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
    Notification::new()
        .summary(title)
        .body(body)
        .appname("pryr")
        .show_async()
        .await?;

    #[cfg(target_os = "windows")]
    Notification::new()
        .summary(title)
        .body(body)
        .appname("pryr")
        .show()?;

    #[cfg(target_os = "android")]
    {
        use tokio::process::Command;
        Command::new("su")
            .arg("-lp")
            .arg("2000")
            .arg("-c")
            .arg(format!(
                "cmd notification post -S bigtext -t '{}' pryr_tag '{}'",
                title, body
            ))
            .spawn()?
            .wait()
            .await?;
    }

    anyhow::Ok(())
}

pub async fn lock_screen() -> anyhow::Result<()> {
    #[cfg(target_os = "linux")]
    {
        use tokio::process::Command;
        Command::new("loginctl")
            .arg("lock-session")
            .spawn()?
            .wait()
            .await?;
    }

    #[cfg(target_os = "windows")]
    unsafe {
        use windows::Win32::System::Shutdown;

        Shutdown::LockWorkStation()?;
    }

    #[cfg(target_os = "android")]
    {
        use tokio::process::Command;
        let output = Command::new("dumpsys").arg("power").output().await?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        if output_str.contains("mWakefulness=Awake") {
            Command::new("input")
                .arg("keyevent")
                .arg("26")
                .spawn()?
                .wait()
                .await?;
        }
    }

    anyhow::Ok(())
}

pub async fn sleep_until_datetime(time: DateTime<Utc>) {
    let now = Utc::now();
    if time > now
        && let Ok(duration) = (time - now).to_std()
    {
        tokio::time::sleep_until(Instant::now() + duration).await;
    }
}

pub fn reload() -> (PrayerManager, Config) {
    let config_path = get_config_path().unwrap();
    let config = Config::from_file(&config_path).expect("Couldn't parse Configuration File");
    let prayer_manager = PrayerManager::new(&config);

    (prayer_manager, config)
}

pub fn get_config_path() -> Option<PathBuf> {
    #[cfg(target_os = "android")]
    return Some(PathBuf::from("/data/local/pryr/config.toml"));

    #[cfg(not(target_os = "android"))]
    Some(
        BaseDirs::new()?
            .config_dir()
            .join("pryr")
            .join("config.toml"),
    )
}

pub fn get_socket_path() -> Option<PathBuf> {
    #[cfg(target_os = "linux")]
    return Some(BaseDirs::new()?.runtime_dir()?.join("pryr.sock"));

    #[cfg(target_os = "android")]
    return Some(PathBuf::from("/data/local/tmp/pryr.sock"));
}
