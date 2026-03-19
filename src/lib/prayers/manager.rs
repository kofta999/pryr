use crate::{
    config::{Config, IqamahOffset, RamadanConfig},
    prayers::{IqamahTime, PrayerEntry, PrayerLocal, PrayerName, PrayerTime, PrayerTodaySchedule},
};
use chrono::{DateTime, Utc};
use salah::{Configuration, Coordinates, Local, PrayerSchedule, PrayerTimes};

pub enum ActionableEvent {
    WaitForPrayer(PrayerName, PrayerTime),
    WaitForIqamah(PrayerName, IqamahTime),
    Skip,
}

pub struct PrayerManager {
    offsets: IqamahOffset,
    scheduler: PrayerSchedule,
    ramadan: RamadanConfig,
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
            ramadan: config.ramadan,
        }
    }

    pub fn get_schedule(&mut self, now: DateTime<Local>) -> PrayerTodaySchedule {
        let today_schedule = self.scheduler.on(now.date_naive()).calculate().unwrap();

        let entry = |prayer: salah::Prayer| {
            let mut prayer_time = today_schedule.time(prayer);
            if matches!(prayer, salah::Prayer::Isha) && self.ramadan.enabled {
                prayer_time =
                    prayer_time + chrono::Duration::minutes(self.ramadan.isha_delay.into());
            }

            PrayerEntry {
                prayer_time,
                iqamah_time: self
                    .get_iqamah_time(prayer.into(), prayer_time)
                    .unwrap_or(prayer_time),
            }
        };

        PrayerTodaySchedule {
            fajr: entry(salah::Prayer::Fajr),
            dhuhr: entry(salah::Prayer::Dhuhr),
            asr: entry(salah::Prayer::Asr),
            maghrib: entry(salah::Prayer::Maghrib),
            isha: entry(salah::Prayer::Isha),
        }
    }

    pub fn get_next_actionable_event(&mut self, now: DateTime<Utc>) -> ActionableEvent {
        let today_schedule = self.scheduler.on(now.date_naive()).calculate().unwrap();

        let time_for = |prayer: salah::Prayer, schedule: PrayerTimes| {
            let mut pt = schedule.time(prayer);
            if matches!(prayer, salah::Prayer::Isha) && self.ramadan.enabled {
                pt = pt + chrono::Duration::minutes(self.ramadan.isha_delay.into());
            }
            pt
        };

        // TODO: Fork salah and update current to use an Option or smth
        let fajr_time = time_for(salah::Prayer::Fajr, today_schedule);
        if now < fajr_time {
            return ActionableEvent::WaitForPrayer(PrayerLocal::Fajr, fajr_time);
        }

        let current_prayer = today_schedule.current();
        let current_prayer_time = time_for(current_prayer, today_schedule);
        let current_iqamah_time = self
            .get_iqamah_time(current_prayer.into(), current_prayer_time)
            .unwrap_or_default();

        let next_prayer = today_schedule.next();
        let next_prayer_time = time_for(next_prayer, today_schedule);

        let tmrw_fajr_time = time_for(salah::Prayer::FajrTomorrow, today_schedule);
        if matches!(next_prayer.into(), PrayerLocal::Ignored) && now < tmrw_fajr_time {
            return ActionableEvent::WaitForPrayer(PrayerLocal::Fajr, tmrw_fajr_time);
        }

        if now > current_prayer_time && now < current_iqamah_time {
            return ActionableEvent::WaitForIqamah(current_prayer.into(), current_iqamah_time);
        }

        if now > current_prayer_time && now < next_prayer_time {
            return ActionableEvent::WaitForPrayer(next_prayer.into(), next_prayer_time);
        }

        ActionableEvent::Skip
    }

    fn get_offset(&self, current_prayer: PrayerName) -> Option<u8> {
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
        prayer_time: PrayerTime,
    ) -> Option<DateTime<Utc>> {
        let offset = self.get_offset(current_prayer)?;
        Some(prayer_time + chrono::Duration::minutes(offset.into()))
    }

    pub fn time_left_for_iqamah(
        &self,
        current_prayer: PrayerLocal,
        prayer_time: PrayerTime,
    ) -> Option<chrono::Duration> {
        Some(self.get_iqamah_time(current_prayer, prayer_time)? - Utc::now())
    }
}
