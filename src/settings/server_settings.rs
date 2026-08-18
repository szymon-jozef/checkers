use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::settings::general_settings::{DEFAULT_URL, SettingsLike};

// TODO! More settings!
#[derive(Serialize, Deserialize)]
pub struct ServerSettings {
    pub addr: SocketAddr,
    pub max_connections: usize,
    pub allow_spectators: bool,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            addr: DEFAULT_URL,
            max_connections: 2, // Maybe this should be a const somewhere in the server code? It
            // makes no sense to have different value than this
            allow_spectators: false,
        }
    }
}

impl SettingsLike for ServerSettings {
    fn get_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(Self::get_config_dir()?.join("server.toml"))
    }
}
