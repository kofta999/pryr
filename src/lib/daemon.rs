use crate::{
    config::IqamahOffset,
    prayers::{PrayerManager, PrayerName, PrayerTime, PrayerTodaySchedule},
};
use owo_colors::OwoColorize;
use salah::{DateTime, Local, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub type UnlockTime = DateTime<Utc>;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub enum DaemonState {
    #[default]
    Calculating,
    WaitingForPrayer(PrayerName, PrayerTime),
    IqamahWarning(PrayerName, PrayerTime),
    LockdownWarning(PrayerName, PrayerTime),
    Lockdown(UnlockTime),
}

impl Display for DaemonState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn format_duration_until(future: DateTime<Utc>) -> String {
            let duration = future - Utc::now();

            if duration.num_seconds() <= 0 {
                return "now".to_string();
            }

            let hours = duration.num_hours();
            let minutes = duration.num_minutes() % 60;
            let seconds = duration.num_seconds() % 60;

            match (hours, minutes, seconds) {
                (0, 0, s) => format!("{s} Seconds"),
                (0, m, s) => format!("{m} Minutes {s} Seconds"),
                (h, m, s) => format!("{h} Hours {m} Minutes {s} Seconds"),
            }
        }

        let is_jumuah = |prayer: PrayerName| {
            let local_now = Local::now();
            use chrono::Datelike;
            local_now.weekday() == chrono::Weekday::Fri
                && prayer == crate::prayers::PrayerLocal::Dhuhr
        };

        match *self {
            Self::Calculating => write!(f, "{}", "Calculating next prayer".bright_cyan()),
            Self::WaitingForPrayer(next_prayer, prayer_time) => {
                let name = if is_jumuah(next_prayer) {
                    "Jumu'ah".to_string()
                } else {
                    next_prayer.to_string()
                };
                write!(
                    f,
                    "Next prayer is {}, in {}",
                    name.yellow(),
                    format_duration_until(prayer_time).green()
                )
            }
            Self::IqamahWarning(current_prayer, iqamah_time) => {
                let name = if is_jumuah(current_prayer) {
                    "Jumu'ah".to_string()
                } else {
                    current_prayer.to_string()
                };
                write!(
                    f,
                    "{} has started, iqamah in {}",
                    name.yellow(),
                    format_duration_until(iqamah_time).green()
                )
            }
            Self::LockdownWarning(prayer_local, lockdown_time) => {
                if is_jumuah(prayer_local) {
                    write!(
                        f,
                        "{} khutbah is soon, lockdown initiating in {}",
                        "Jumu'ah".yellow(),
                        format_duration_until(lockdown_time).red()
                    )
                } else {
                    write!(
                        f,
                        "{} iqamah is soon, lockdown initiating in {}",
                        prayer_local.yellow(),
                        format_duration_until(lockdown_time).red()
                    )
                }
            }
            Self::Lockdown(unlock_time) => write!(
                f,
                "Lockdown is active, unlocking after {}",
                format_duration_until(unlock_time).red()
            ),
        }
    }
}

#[derive(Default)]
pub struct DaemonSnapShot {
    pub current_state: DaemonState,
    pub daily_schedule: PrayerTodaySchedule,
    pub offsets: IqamahOffset,
}

impl DaemonSnapShot {
    pub fn new(
        next_event: DaemonState,
        prayer_manager: &mut PrayerManager,
        offsets: IqamahOffset,
    ) -> Self {
        Self {
            current_state: next_event,
            daily_schedule: prayer_manager.get_schedule(Local::now()),
            offsets,
        }
    }
}
