use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PrayerLocal {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
    Ignored,
}

impl From<salah::Prayer> for PrayerLocal {
    fn from(prayer: salah::Prayer) -> Self {
        match prayer {
            salah::Prayer::Fajr => PrayerLocal::Fajr,
            salah::Prayer::Sunrise => PrayerLocal::Ignored,
            salah::Prayer::Dhuhr => PrayerLocal::Dhuhr,
            salah::Prayer::Asr => PrayerLocal::Asr,
            salah::Prayer::Maghrib => PrayerLocal::Maghrib,
            salah::Prayer::Isha => PrayerLocal::Isha,
            salah::Prayer::Qiyam => PrayerLocal::Ignored,
            salah::Prayer::FajrTomorrow => PrayerLocal::Fajr,
        }
    }
}

impl Display for PrayerLocal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let prayer_name = match self {
            PrayerLocal::Fajr => salah::Prayer::Fajr.name(),
            PrayerLocal::Dhuhr => salah::Prayer::Dhuhr.name(),
            PrayerLocal::Asr => salah::Prayer::Asr.name(),
            PrayerLocal::Maghrib => salah::Prayer::Maghrib.name(),
            PrayerLocal::Isha => salah::Prayer::Isha.name(),
            PrayerLocal::Ignored => "Ignored".to_string(),
        };
        write!(f, "{}", prayer_name)
    }
}
