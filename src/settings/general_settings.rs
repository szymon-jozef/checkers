use core::error;
use std::path::PathBuf;

use log::warn;
use serde::{Serialize, de::DeserializeOwned};

// maybe consider some better name for this
pub trait SettingsLike: Serialize + DeserializeOwned + Default {
    fn get_config_dir() -> Result<PathBuf, Box<dyn error::Error>> {
        Ok(dirs::config_local_dir()
            .ok_or("Couldn't get config dir!")?
            .join("checkers"))
    }

    fn get_file_path() -> Result<PathBuf, Box<dyn error::Error>>;

    fn save_to_file(&self) -> Result<(), Box<dyn error::Error>> {
        let path: PathBuf = Self::get_file_path()?;

        if let Some(parent_dir) = path.parent() {
            std::fs::create_dir_all(parent_dir)?;
        }

        let toml = toml::to_string(&self)?;
        std::fs::write(path, toml)?;

        Ok(())
    }

    fn read_from_file() -> Result<Self, Box<dyn error::Error>> {
        Ok(toml::from_slice(&std::fs::read(Self::get_file_path()?)?)?) // i love rust
    }

    fn new() -> Self {
        match Self::read_from_file() {
            Ok(settings) => return settings,
            Err(e) => warn!("Could not load settings from file, because: {}", e),
        }

        let settings = Self::default();

        if let Err(e) = settings.save_to_file() {
            warn!("Could not save server settings to the file, because: {}", e);
        }

        settings
    }
}
