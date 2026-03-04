mod config;
mod prayer_madhab;
mod prayer_method;
use std::path::Path;

use salah::Configuration;

use crate::lib::config::Config;

pub fn run() {
    let config = Config::from_file("test-config.toml").unwrap();

    dbg!(config);
}
