use pryr::{
    config::Config,
    prayer_manager::{ActionableEvent, PrayerManager, PrayerName, PrayerTime},
};
use salah::{DateTime, Utc};
use std::time::Duration;
use tokio::{process::Command, time::Instant};

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
                    ActionableEvent::WaitForIqamah(_, time) => DaemonState::WaitingForIqamah(time),
                    ActionableEvent::Skip => panic!("Shouldn't happen"),
                }
            }
            DaemonState::WaitingForPrayer(prayer, time) => {
                println!("Current prayer is {prayer:?} at {time}",);
                println!("Sleeping until prayer");

                sleep_until_datetime(time).await;

                // Fire a notification
                println!("Woke up for prayer");

                let iqamah_time = prayer_manager.get_iqamah_time(prayer, time).unwrap();

                DaemonState::WaitingForIqamah(iqamah_time)
            }
            DaemonState::WaitingForIqamah(time) => {
                let five_min_before_iqamah = time - Duration::from_secs(5 * 60);
                let two_min_before_iqamah = time - Duration::from_secs(2 * 60);

                sleep_until_datetime(five_min_before_iqamah).await;
                println!("Noti: 5 min before iqamah, lockdown in 3m");

                sleep_until_datetime(two_min_before_iqamah).await;
                println!("Noti: 2 min before iqamah, lockdown now");

                println!("Initating lockdown");
                DaemonState::Lockdown(time + Duration::from_secs(10 * 60))
            }
            DaemonState::Lockdown(unlock_time) => {
                Command::new("loginctl")
                    .arg("lock-session")
                    .spawn()
                    .unwrap();

                println!("Lockdown finished");

                DaemonState::Calculating
            }
        }
    }
}

enum DaemonState {
    Calculating,
    WaitingForPrayer(PrayerName, PrayerTime),
    WaitingForIqamah(PrayerTime),
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
