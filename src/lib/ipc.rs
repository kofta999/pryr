use crate::{config::IqamahOffset, daemon::DaemonState, prayer_manager::PrayerTodaySchedule};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum IpcRequest {
    GetStatus,
    GetTodaySchedule,
    ReloadConfig,
}

#[derive(Serialize, Deserialize)]
pub enum IpcResponse {
    CurrentState(DaemonState),          // Response for GetStatus
    DailySchedule(PrayerTodaySchedule), // Response for GetTodaySchedule
    ConfigOffsets(IqamahOffset),        // Response for GetSettings
    Success,                            // Response for ReloadConfig / TriggerLockdown
    Error(String),
}

#[derive(Default)]
pub struct DaemonSnapShot {
    pub current_state: DaemonState,
    pub daily_schedule: PrayerTodaySchedule,
    pub offsets: IqamahOffset,
}
