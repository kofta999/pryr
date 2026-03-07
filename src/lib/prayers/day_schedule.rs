use crate::prayers::{IqamahTime, PrayerTime};
use chrono::{DateTime, Utc};
use owo_colors::OwoColorize;
use salah::Local;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PrayerEntry {
    pub prayer_time: PrayerTime,
    pub iqamah_time: IqamahTime,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct PrayerTodaySchedule {
    pub fajr: PrayerEntry,
    pub dhuhr: PrayerEntry,
    pub asr: PrayerEntry,
    pub maghrib: PrayerEntry,
    pub isha: PrayerEntry,
}

impl Display for PrayerTodaySchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let now = Utc::now();

        fn format_row(name: &str, entry: &PrayerEntry, now: DateTime<Utc>) -> String {
            let prayer_local = entry.prayer_time.with_timezone(&Local);
            let iqamah_local = entry.iqamah_time.with_timezone(&Local);
            let indicator = if now > entry.prayer_time {
                "✓".green().to_string()
            } else {
                "○".yellow().to_string()
            };
            format!(
                "  {} {:<10} {:>8} {:>14}",
                indicator,
                name.bold(),
                prayer_local.format("%I:%M %p").blue(),
                iqamah_local.format("%I:%M %p").purple(),
            )
        }

        writeln!(f, "{}", "┌─ Today's Prayer Schedule".bright_cyan().bold())?;
        writeln!(
            f,
            " {:>2} {:<10} {:>8} {:>14}",
            "", "Prayer", "Adhan", "Iqamah"
        )?;
        writeln!(f, "  {}", "─────────────────────────────────────".dimmed())?;
        writeln!(f, "{}", format_row("Fajr", &self.fajr, now))?;
        writeln!(f, "{}", format_row("Dhuhr", &self.dhuhr, now))?;
        writeln!(f, "{}", format_row("Asr", &self.asr, now))?;
        writeln!(f, "{}", format_row("Maghrib", &self.maghrib, now))?;
        writeln!(f, "{}", format_row("Isha", &self.isha, now))?;
        write!(f, "{}", "└───────────────────────────────────────".dimmed())
    }
}
