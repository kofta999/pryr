mod day_schedule;
mod local;
mod manager;

pub use day_schedule::*;
pub use local::*;
pub use manager::*;
use salah::{DateTime, Utc};

pub type PrayerName = PrayerLocal;
pub type PrayerTime = DateTime<Utc>;
pub type IqamahTime = DateTime<Utc>;
