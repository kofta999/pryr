use crate::{daemon::DaemonState, prayers::PrayerTodaySchedule};
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
    Success,                            // Response for ReloadConfig
    Error(String),
}
