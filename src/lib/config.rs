use crate::prayers::{MadhabLocal, MethodLocal};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Config {
    #[serde(default)]
    pub location: Location,
    #[serde(rename = "prayer-config", default)]
    pub prayer_time: PrayerConfig,
    #[serde(default)]
    pub jumuah: JumuahConfig,
    #[serde(default)]
    pub ramadan: RamadanConfig,
    #[serde(rename = "iqamah-offset", default)]
    pub iqamah_offset: IqamahOffset,
    #[serde(default)]
    pub options: Options,
    #[serde(rename = "lockdown", default)]
    pub lockdown: LockdownConfig,
}

impl Config {
    pub fn from_file(path: &PathBuf) -> Result<Config> {
        match fs::read_to_string(path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => Ok(config),
                Err(e) => {
                    eprintln!("[WARNING] Failed to parse config file: {}", e);
                    Ok(Config::default())
                }
            },
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<()> {
        let contents = toml::to_string(self)?;
        fs::write(path, contents)?;

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Options {
    #[serde(rename = "lock-screen")]
    pub lock_screen: bool,
}

impl Default for Options {
    fn default() -> Self {
        Self { lock_screen: true }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Location {
    pub long: f64,
    pub lat: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct LockdownConfig {
    #[serde(rename = "warning-before-iqamah")]
    pub warning_before_iqamah: u32,
    #[serde(rename = "lock-before-iqamah")]
    pub lock_before_iqamah: u32,
    #[serde(rename = "unlock-after-iqamah")]
    pub unlock_after_iqamah: u32,
}

impl Default for LockdownConfig {
    fn default() -> Self {
        Self {
            warning_before_iqamah: 5,
            lock_before_iqamah: 2,
            unlock_after_iqamah: 10,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(default)]
pub struct JumuahConfig {
    #[serde(rename = "early-warning")]
    pub early_warning: u32,
    #[serde(rename = "lockdown-duration")]
    pub lockdown_duration: u32,
}

impl Default for JumuahConfig {
    fn default() -> Self {
        Self {
            early_warning: 45,
            lockdown_duration: 30,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(default)]
pub struct RamadanConfig {
    pub enabled: bool,
    #[serde(rename = "isha-delay")]
    pub isha_delay: u32,
}

impl Default for RamadanConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            isha_delay: 30,
        }
    }
}
