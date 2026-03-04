mod config;
mod iqamah_calculator;
mod prayers_local;

use salah::{Configuration, Coordinates, Local, PrayerSchedule};

use crate::lib::{config::Config, iqamah_calculator::IqamahCalculator};

pub fn run() {
    let config = Config::from_file("test-config.toml").unwrap();

    let location = Coordinates::new(config.location.lat, config.location.long);
    let params = Configuration::with(
        config.prayer_time.method.into(),
        config.prayer_time.madhab.into(),
    );

    let prayers = PrayerSchedule::new()
        .for_location(location)
        .with_configuration(params)
        .on(Local::now().date_naive())
        .calculate()
        .unwrap();

    let calc = IqamahCalculator::new(config.iqamah_offset);

    dbg!(prayers.next());

    dbg!(calc.time_left(prayers.next().into(), prayers.time(prayers.next())));
}
