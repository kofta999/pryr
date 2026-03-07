use crate::{
    config::{Config, IqamahOffset},
    prayers_local::prayer_local::PrayerLocal,
};
use chrono::{DateTime, Utc};
use salah::{Configuration, Coordinates, PrayerSchedule};
use serde::{Deserialize, Serialize};

pub type PrayerName = PrayerLocal;
pub type PrayerTime = DateTime<Utc>;
pub type IqamahTime = DateTime<Utc>;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PrayerTodaySchedule {
    pub fajr: PrayerTime,
    pub dhuhr: PrayerTime,
    pub asr: PrayerTime,
    pub maghrib: PrayerTime,
    pub isha: PrayerTime,
}

pub enum ActionableEvent {
    WaitForPrayer(PrayerName, PrayerTime),
    WaitForIqamah(PrayerName, IqamahTime),
    Skip,
}

pub struct PrayerManager {
    offsets: IqamahOffset,
    scheduler: PrayerSchedule,
}

impl PrayerManager {
    pub fn new(config: &Config) -> Self {
        let location = Coordinates::new(config.location.lat, config.location.long);
        let params = Configuration::with(
            config.prayer_time.method.into(),
            config.prayer_time.madhab.into(),
        );

        let mut scheduler = PrayerSchedule::new();
        scheduler.for_location(location).with_configuration(params);

        Self {
            offsets: config.iqamah_offset,
            scheduler,
        }
    }

    pub fn get_schedule(&mut self, now: DateTime<Utc>) -> PrayerTodaySchedule {
        let today_schedule = self.scheduler.on(now.date_naive()).calculate().unwrap();

        PrayerTodaySchedule {
            fajr: today_schedule.time(salah::Prayer::Fajr),
            dhuhr: today_schedule.time(salah::Prayer::Dhuhr),
            asr: today_schedule.time(salah::Prayer::Asr),
            maghrib: today_schedule.time(salah::Prayer::Maghrib),
            isha: today_schedule.time(salah::Prayer::Isha),
        }
    }

    pub fn get_next_actionable_event(&mut self, now: DateTime<Utc>) -> ActionableEvent {
        let today_schedule = self.scheduler.on(now.date_naive()).calculate().unwrap();

        // TODO: Fork salah and update current to use an Option or smth
        let fajr_time = today_schedule.time(salah::Prayer::Fajr);
        if now < fajr_time {
            return ActionableEvent::WaitForPrayer(PrayerLocal::Fajr, fajr_time);
        }

        let current_prayer = today_schedule.current();
        let current_prayer_time = today_schedule.time(current_prayer);
        let current_iqamah_time = self
            .get_iqamah_time(current_prayer.into(), current_prayer_time)
            .unwrap_or_default();

        let next_prayer = today_schedule.next();
        let next_prayer_time = today_schedule.time(next_prayer);

        if now > current_prayer_time && now < current_iqamah_time {
            return ActionableEvent::WaitForIqamah(current_prayer.into(), current_iqamah_time);
        }

        if now > current_prayer_time && now < next_prayer_time {
            return ActionableEvent::WaitForPrayer(next_prayer.into(), next_prayer_time);
        }

        ActionableEvent::Skip
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

    pub fn time_left_for_iqamah(
        &self,
        current_prayer: PrayerLocal,
        prayer_time: DateTime<Utc>,
    ) -> Option<chrono::Duration> {
        Some(self.get_iqamah_time(current_prayer, prayer_time)? - Utc::now())
    }
}
