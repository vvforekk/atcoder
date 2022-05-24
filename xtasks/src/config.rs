use log::warn;
use serde::{Deserialize, Serialize};

const CONFIG_PATH: &str = "/Users/roku/room/comp/atcoder/xtasks/atcoder_xtasks.toml";

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Config {
    pub cargo_compete_home_dir: String,
    pub open_tasks_print: bool,
}

pub fn read_config() -> anyhow::Result<Config> {
    let config: Result<Config, confy::ConfyError> = confy::load_path(CONFIG_PATH);
    match config {
        Ok(config) => {
            if config.cargo_compete_home_dir.is_empty() {
                warn!("cargo_compete_home_dir is not set in {}", CONFIG_PATH);
            }

            Ok(config)
        }
        Err(e) => Err(e.into()),
    }
}
