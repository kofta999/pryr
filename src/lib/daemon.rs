use crate::{
    config::IqamahOffset,
    prayer_manager::{PrayerManager, PrayerName, PrayerTime, PrayerTodaySchedule},
};
use salah::{DateTime, Local, Utc};
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
