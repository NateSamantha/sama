use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub listeners: Vec<SocketAddr>,
    pub catalog: PathBuf,
}

impl Config {
    pub async fn from_toml(f: impl AsRef<Path>) -> eyre::Result<Config> {
        let s = tokio::fs::read_to_string(f).await?;
        Ok(toml::from_str(&s)?)
    }
}
