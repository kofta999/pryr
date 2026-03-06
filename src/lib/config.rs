use crate::prayers_local::{madhab_local::MadhabLocal, method_local::MethodLocal};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Config {
    pub location: Location,
    #[serde(rename = "prayer-config")]
    pub prayer_time: PrayerConfig,
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

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct PrayerConfig {
    pub method: MethodLocal,
    pub madhab: MadhabLocal,
}

impl Default for PrayerConfig {
    fn default() -> Self {
        Self {
            method: MethodLocal::Egyptian,
            madhab: MadhabLocal::Shafi,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct IqamahOffset {
    pub fajr: u8,
    pub dhuhr: u8,
    pub asr: u8,
    pub maghrib: u8,
    pub isha: u8,
}

impl Default for IqamahOffset {
    fn default() -> Self {
        Self {
            fajr: 20,
            dhuhr: 15,
            asr: 15,
            maghrib: 10,
            isha: 15,
        }
    }
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub struct Options {
    #[serde(rename = "lock-screen")]
    pub lock_screen: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { lock_screen: true }
    }
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Location {
    pub long: f64,
    pub lat: f64,
}
