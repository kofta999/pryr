use salah::{DateTime, Utc};

use crate::lib::{config::IqamahOffset, prayers_local::prayer_local::PrayerLocal};

pub struct IqamahCalculator {
    offsets: IqamahOffset, // LOGIC:
                           // fn: gets current prayer and calculates how much time left before iqamah
}

impl IqamahCalculator {
    pub fn new(offsets: IqamahOffset) -> Self {
        Self { offsets }
    }

    pub fn time_left(&self, current_prayer: PrayerLocal, prayer_date: DateTime<Utc>) -> i64 {
        let offset = match current_prayer {
            PrayerLocal::Fajr => self.offsets.fajr,
            PrayerLocal::Dhuhr => self.offsets.dhuhr,
            PrayerLocal::Asr => self.offsets.asr,
            PrayerLocal::Maghrib => self.offsets.maghrib,
            PrayerLocal::Isha => self.offsets.isha,
        };

        let iqamah_date = prayer_date + chrono::Duration::minutes(offset.into());

        (iqamah_date - Utc::now()).num_minutes()
    }
}
