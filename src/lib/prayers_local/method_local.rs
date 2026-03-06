use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum MethodLocal {
    /// Muslim World League. Standard Fajr time with an angle of 18°.
    /// Earlier Isha time with an angle of 17°.
    MuslimWorldLeague,

    /// Egyptian General Authority of Survey. Early Fajr time using an angle 19.5°
    /// and a slightly earlier Isha time using an angle of 17.5°.
    Egyptian,

    /// University of Islamic Sciences, Karachi. A generally applicable method that
    /// uses standard Fajr and Isha angles of 18°.
    Karachi,

    /// Umm al-Qura University, Makkah. Uses a fixed interval of 90 minutes
    /// from maghrib to calculate Isha. And a slightly earlier Fajr time with
    /// an angle of 18.5°. Note: you should add a +30 minute custom adjustment
    /// for Isha during Ramadan.
    UmmAlQura,

    /// Used in the UAE. Slightly earlier Fajr time and slightly later Isha
    /// time with angles of 18.2° for Fajr and Isha in addition to 3 minute
    /// offsets for sunrise, Dhuhr, Asr, and Maghrib.
    Dubai,

    /// Method developed by Khalid Shaukat, founder of Moonsighting Committee Worldwide.
    /// Uses standard 18° angles for Fajr and Isha in addition to seasonal adjustment values.
    /// This method automatically applies the 1/7 approximation rule for locations above 55°
    /// latitude. Recommended for North America and the UK.
    MoonsightingCommittee,

    /// Also known as the ISNA method. Can be used for North America,
    /// but the moonsightingCommittee method is preferable. Gives later Fajr times and early.
    /// Isha times with angles of 15°.
    NorthAmerica,

    /// Standard Fajr time with an angle of 18°. Slightly earlier Isha time with an angle of 17.5°.
    Kuwait,

    /// Same Isha interval as `ummAlQura` but with the standard Fajr time using an angle of 18°.
    Qatar,

    /// Used in Singapore, Malaysia, and Indonesia. Early Fajr time with an angle of 20°
    /// and standard Isha time with an angle of 18°.
    Singapore,

    /// Institute of Geophysics, University of Tehran. Early Isha time with an angle of 14°.
    /// Slightly later Fajr time with an angle of 17.7°. Calculates Maghrib based on the sun
    /// reaching an angle of 4.5° below the horizon.
    Tehran,

    /// An approximation of the Diyanet method used in Turkey.
    /// This approximation is less accurate outside the region of Turkey.
    Turkey,

    /// Defaults to angles of 0°, should generally be used for making a custom method
    /// and setting your own values.
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
