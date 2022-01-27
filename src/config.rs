use std::fs::File;

use anyhow::Result;
use once_cell::sync::OnceCell;
use serde::Deserialize;

pub static CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Deserialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub limit: Option<usize>,
    pub from: Option<String>,
    pub color: Option<bool>,
}

pub fn load_config() -> Result<()> {
    if let Some(config_dir) = dirs::config_dir() {
        let config_path = config_dir.join("spt_cli.json");

        if config_path.exists() {
            let file = File::open(config_path)?;
            let config: Config = serde_json::from_reader(file)?;

            CONFIG.set(config).unwrap();
        }
    }

    if let None = CONFIG.get() {
        CONFIG.set(Config::default()).unwrap();
    }

    Ok(())
}
