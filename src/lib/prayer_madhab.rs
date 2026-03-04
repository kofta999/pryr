use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum PrayerMadhab {
    Shafi = 1,
    Hanafi = 2,
}
