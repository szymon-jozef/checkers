use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::settings::general_settings::SettingsLike;

// TODO! More settings!
#[derive(Serialize, Deserialize)]
pub struct ServerSettings {
    pub addr: SocketAddr,
    pub max_connections: usize,
    pub allow_spectators: bool,
}

impl SettingsLike for ServerSettings {
    fn get_file_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        Ok(Self::get_config_dir()?.join("server.toml"))
    }
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            addr: SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 6767),
            max_connections: 2, // Maybe this should be a const somewhere in the server code? It
            // makes no sense to have different value than this
            allow_spectators: false,
        }
    }
}
