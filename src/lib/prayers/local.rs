use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MadhabLocal {
    Shafi = 1,
    Hanafi = 2,
}

impl From<MadhabLocal> for salah::Madhab {
    fn from(madhab: MadhabLocal) -> salah::Madhab {
        match madhab {
            MadhabLocal::Shafi => salah::Madhab::Shafi,
            MadhabLocal::Hanafi => salah::Madhab::Hanafi,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub enum MethodLocal {
    MuslimWorldLeague,
    Egyptian,
    Karachi,
    UmmAlQura,
    Dubai,
    MoonsightingCommittee,
    NorthAmerica,
    Kuwait,
    Qatar,
    Singapore,
    Tehran,
    Turkey,
    Other,
}

impl From<MethodLocal> for salah::Method {
    fn from(method: MethodLocal) -> salah::Method {
        match method {
            MethodLocal::MuslimWorldLeague => salah::Method::MuslimWorldLeague,
            MethodLocal::Egyptian => salah::Method::Egyptian,
            MethodLocal::Karachi => salah::Method::Karachi,
            MethodLocal::UmmAlQura => salah::Method::UmmAlQura,
            MethodLocal::Dubai => salah::Method::Dubai,
            MethodLocal::MoonsightingCommittee => salah::Method::MoonsightingCommittee,
            MethodLocal::NorthAmerica => salah::Method::NorthAmerica,
            MethodLocal::Kuwait => salah::Method::Kuwait,
            MethodLocal::Qatar => salah::Method::Qatar,
            MethodLocal::Singapore => salah::Method::Singapore,
            MethodLocal::Tehran => salah::Method::Tehran,
            MethodLocal::Turkey => salah::Method::Turkey,
            MethodLocal::Other => salah::Method::Other,
        }
    }
}

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
