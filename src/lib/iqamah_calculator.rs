use crate::{config::IqamahOffset, prayers_local::prayer_local::PrayerLocal};
use salah::{DateTime, Utc};

pub struct IqamahCalculator {
    offsets: IqamahOffset,
}

impl IqamahCalculator {
    pub fn new(offsets: IqamahOffset) -> Self {
        Self { offsets }
    }

    fn get_offset(&self, current_prayer: PrayerLocal) -> Option<u8> {
        let offset = match current_prayer {
            PrayerLocal::Fajr => self.offsets.fajr,
            PrayerLocal::Dhuhr => self.offsets.dhuhr,
            PrayerLocal::Asr => self.offsets.asr,
            PrayerLocal::Maghrib => self.offsets.maghrib,
            PrayerLocal::Isha => self.offsets.isha,
            PrayerLocal::Ignored => return None,
        };

        Some(offset)
    }

    pub fn get_iqamah_time(
        &self,
        current_prayer: PrayerLocal,
        prayer_date: DateTime<Utc>,
    ) -> Option<DateTime<Utc>> {
        let offset = self.get_offset(current_prayer)?;
        Some(prayer_date + chrono::Duration::minutes(offset.into()))
    }

    pub fn time_left(
        &self,
        current_prayer: PrayerLocal,
        prayer_date: DateTime<Utc>,
    ) -> Option<chrono::Duration> {
        Some(self.get_iqamah_time(current_prayer, prayer_date)? - Utc::now())
    }
}
