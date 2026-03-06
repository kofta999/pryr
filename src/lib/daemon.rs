use crate::prayer_manager::{PrayerName, PrayerTime};
use salah::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type UnlockTime = DateTime<Utc>;

#[derive(Serialize, Deserialize, Default, Clone, Copy)]
pub enum DaemonState {
    #[default]
    Calculating,
    WaitingForPrayer(PrayerName, PrayerTime),
    WaitingForIqamah(PrayerName, PrayerTime),
    Lockdown(UnlockTime),
}
