use std::{fs, path::Path};

use anyhow::Result;
use serde::Deserialize;

use crate::lib::{prayer_madhab::PrayerMadhab, prayer_method::PrayerMethod};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub location: Location,
    #[serde(rename = "prayer-time")]
    pub prayer_time: PrayerTime,
    #[serde(rename = "iqamah-offset")]
    pub iqamah_offset: IqamahOffset,
    pub options: Options,
}

impl Config {
    pub fn from_file(path: &str) -> Result<Config> {
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }
}

#[derive(Deserialize, Debug)]
pub struct PrayerTime {
    pub method: PrayerMethod,
    pub madhab: PrayerMadhab,
}

#[derive(Deserialize, Debug)]
pub struct IqamahOffset {
    pub fajr: u8,
    pub dhuhr: u8,
    pub asr: u8,
    pub maghrib: u8,
    pub isha: u8,
}

#[derive(Deserialize, Debug)]
pub struct Options {
    #[serde(rename = "lock-screen")]
    lock_screen: bool,
}

#[derive(Deserialize, Debug)]
pub struct Location {
    pub long: f32,
    pub lat: f32,
}
