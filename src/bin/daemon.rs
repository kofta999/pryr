use pryr::{
    config::Config, iqamah_calculator::IqamahCalculator, prayers_local::prayer_local::PrayerLocal,
};
use salah::{Configuration, Coordinates, DateTime, Local, PrayerSchedule, Utc};
use std::time::Duration;
use tokio::{process::Command, time::Instant};

#[tokio::main]
async fn main() {
    let config = Config::from_file("test-config.toml").expect("Couldn't parse Configuration File");

    tokio::spawn(daemon_loop(config)).await.unwrap();
}

async fn daemon_loop(config: Config) {
    let location = Coordinates::new(config.location.lat, config.location.long);
    let params = Configuration::with(
        config.prayer_time.method.into(),
        config.prayer_time.madhab.into(),
    );
    let iqamah_calculator = IqamahCalculator::new(config.iqamah_offset);

    // Needs date then calc
    let mut prayer_scheduler = PrayerSchedule::new();
    let prayer_scheduler = prayer_scheduler
        .for_location(location)
        .with_configuration(params);
    let mut state = DaemonState::Calculating;

    loop {
        state = match state {
            DaemonState::Calculating => {
                let schedule = prayer_scheduler
                    .on(Local::now().date_naive())
                    .calculate()
                    .unwrap();
                let next_prayer = schedule.next();

                DaemonState::WaitingForPrayer(schedule.time(next_prayer), next_prayer.into())
            }
            DaemonState::WaitingForPrayer(time, prayer) => {
                println!("Current prayer is {prayer:?} at {time}",);
                println!("Sleeping until prayer");

                let now = Utc::now();
                if time > now {
                    if let Ok(duration) = (time - now).to_std() {
                        println!("Sleeping for {duration:?}");
                        tokio::time::sleep_until(Instant::now() + duration).await;
                    }
                }

                // Fire a notification

                println!("Woke up for prayer");
                DaemonState::WaitingForIqamah(
                    iqamah_calculator.get_iqamah_time(prayer, time).unwrap(),
                )
            }
            DaemonState::WaitingForIqamah(time) => {
                let now = Utc::now();
                if time > now {
                    if let Ok(duration) = (time - now).to_std() {
                        println!("Sleeping for {duration:?}");
                        tokio::time::sleep_until(
                            Instant::now() + (duration - Duration::from_secs(2 * 60)),
                        )
                        .await;
                    }
                }

                println!("Initating lockdown");

                DaemonState::Lockdown(now + Duration::from_secs(10 * 60))
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
    // PrayerTime, PrayerName
    WaitingForPrayer(DateTime<Utc>, PrayerLocal),
    // IqamahTime
    WaitingForIqamah(DateTime<Utc>),
    // UnlockTime
    Lockdown(DateTime<Utc>),
}
