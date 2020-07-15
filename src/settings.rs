use cis_client::settings::CisSettings;
use config::{Config, ConfigError, Environment, File};
use dirs::config_dir;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub cis: CisSettings,
}

impl Settings {
    pub fn new(config_file: Option<&str>) -> Result<Self, ConfigError> {
        let file: PathBuf = config_file.map(|c| PathBuf::from(c)).unwrap_or_else(|| {
            let mut p = config_dir().unwrap();
            p.push("person_cli");
            p.push("settings.json");
            p
        });
        let mut s = Config::new();
        s.merge(File::from(file))?;
        s.merge(Environment::new().separator("__"))?;
        s.try_into()
    }
}
