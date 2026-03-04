pub enum PrayerLocal {
    Fajr,
    Dhuhr,
    Asr,
    Maghrib,
    Isha,
}

impl From<salah::Prayer> for PrayerLocal {
    fn from(prayer: salah::Prayer) -> Self {
        match prayer {
            salah::Prayer::Fajr => PrayerLocal::Fajr,
            salah::Prayer::Sunrise => PrayerLocal::Dhuhr,
            salah::Prayer::Dhuhr => PrayerLocal::Dhuhr,
            salah::Prayer::Asr => PrayerLocal::Asr,
            salah::Prayer::Maghrib => PrayerLocal::Maghrib,
            salah::Prayer::Isha => PrayerLocal::Isha,
            salah::Prayer::Qiyam => PrayerLocal::Isha,
            salah::Prayer::FajrTomorrow => PrayerLocal::Fajr,
        }
    }
}
