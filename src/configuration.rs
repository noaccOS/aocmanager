use color_eyre::eyre;

use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::templates::Template;

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct Configuration {
    pub days: Vec<DayConfiguration>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone)]
pub struct DayConfiguration {
    pub day: u8,
    pub template: Template,
}

impl Configuration {
    fn config_file(aoc_root: &Path) -> PathBuf {
        aoc_root.join(".aoc/config.toml")
    }

    pub fn read(aoc_root: &Path) -> eyre::Result<Self> {
        let config_file = Self::config_file(aoc_root);
        let config = fs::read_to_string(config_file)?;
        let config: Configuration = toml::from_str(&config)?;

        Ok(config)
    }

    pub fn for_day(&self, day: u8) -> Option<DayConfiguration> {
        self.days
            .iter()
            .find(|&d| d.day == day)
            .map(|d| (*d).clone())
    }

    pub fn write(&self, aoc_root: &Path) -> eyre::Result<()> {
        let config_file = Self::config_file(aoc_root);
        let config = toml::to_string(self)?;
        fs::write(config_file, config)?;

        Ok(())
    }
}
