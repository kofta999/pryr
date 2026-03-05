use notify_rust::Notification;
use pryr::{
    config::Config,
    prayer_manager::{ActionableEvent, PrayerManager, PrayerName, PrayerTime},
};
use salah::{DateTime, Utc};
use std::time::Duration;
use tokio::{process::Command, time::Instant};

const LOCKDOWN_POLL_SECONDS: u64 = 10;
const LOCKDOWN_DURATION_SECONDS: u64 = 10 * 60;

#[tokio::main]
async fn main() {
    let config = Config::from_file("test-config.toml").expect("Couldn't parse Configuration File");

    tokio::spawn(daemon_loop(config)).await.unwrap();
}

async fn daemon_loop(config: Config) {
    let mut prayer_manager = PrayerManager::new(config);

    let mut state = DaemonState::Calculating;

    loop {
        state = match state {
            DaemonState::Calculating => {
                let now = Utc::now();
                let event = prayer_manager.get_next_actionable_event(now);

                match event {
                    ActionableEvent::WaitForPrayer(name, time) => {
                        DaemonState::WaitingForPrayer(name, time)
                    }
                    ActionableEvent::WaitForIqamah(name, time) => {
                        DaemonState::WaitingForIqamah(name, time)
                    }
                    ActionableEvent::Skip => panic!("Shouldn't happen"),
                }
            }
            DaemonState::WaitingForPrayer(prayer, time) => {
                println!("Next prayer is {prayer:?} at {time}",);
                println!("Sleeping until prayer");

                sleep_until_datetime(time).await;

                // Fire a notification
                println!("Woke up for prayer");

                Notification::new()
                    .summary(&format!("Prayer {prayer} has started"))
                    .body(&format!(
                        "Iqamah in {} minutes",
                        prayer_manager
                            .time_left_for_iqamah(prayer, time)
                            .unwrap()
                            .num_minutes()
                    ))
                    .show_async()
                    .await
                    .unwrap();

                let iqamah_time = prayer_manager.get_iqamah_time(prayer, time).unwrap();

                DaemonState::WaitingForIqamah(prayer, iqamah_time)
            }
            DaemonState::WaitingForIqamah(prayer, time) => {
                let five_min_before_iqamah = time - Duration::from_secs(5 * 60);
                let two_min_before_iqamah = time - Duration::from_secs(2 * 60);
                let one_half_min_before_iqamah = time - Duration::from_secs(1);

                sleep_until_datetime(five_min_before_iqamah).await;
                Notification::new()
                    .summary(&format!("{prayer} Iqamah in 5 minutes"))
                    .body("Get ready! Lockdown in 3 minutes!")
                    .show_async()
                    .await
                    .unwrap();

                sleep_until_datetime(two_min_before_iqamah).await;
                Notification::new()
                    .summary(&format!("{prayer} Iqamah in 2 minutes"))
                    .body("Get ready! Lockdown in 30 seconds!")
                    .show_async()
                    .await
                    .unwrap();

                sleep_until_datetime(one_half_min_before_iqamah).await;
                println!("Initiating lockdown");

                DaemonState::Lockdown(time + Duration::from_secs(LOCKDOWN_DURATION_SECONDS))
            }
            DaemonState::Lockdown(unlock_time) => {
                while Utc::now() < unlock_time {
                    Command::new("loginctl")
                        .arg("lock-session")
                        .spawn()
                        .unwrap();

                    tokio::time::sleep(Duration::from_secs(LOCKDOWN_POLL_SECONDS)).await;
                }

                println!("Lockdown finished");

                DaemonState::Calculating
            }
        }
    }
}

enum DaemonState {
    Calculating,
    WaitingForPrayer(PrayerName, PrayerTime),
    WaitingForIqamah(PrayerName, PrayerTime),
    // UnlockTime
    Lockdown(DateTime<Utc>),
}

async fn sleep_until_datetime(time: DateTime<Utc>) {
    let now = Utc::now();
    if time > now {
        if let Ok(duration) = (time - now).to_std() {
            println!("Sleeping for {duration:?}");
            tokio::time::sleep_until(Instant::now() + duration).await;
        }
    }
}
