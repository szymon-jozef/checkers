use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

use crate::settings::general_settings::{DEFAULT_URL, SettingsLike};

#[derive(Clone, Serialize, Deserialize)]
pub struct ClientSettings {
    pub server_url: SocketAddr,
    pub name: String,
}

impl Default for ClientSettings {
    fn default() -> Self {
        Self {
            server_url: DEFAULT_URL,
            name: "Player".to_string(),
        }
    }
}

impl SettingsLike for ClientSettings {
    fn get_file_path() -> Result<std::path::PathBuf, Box<dyn core::error::Error>> {
        Ok(Self::get_config_dir()?.join("client.toml"))
    }
}
