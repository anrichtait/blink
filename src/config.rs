use serde::Deserialize;


#[derive(Deserialize, Debug)]
pub struct Config {
    pub fav_dirs: Vec<String>,
    pub exclude: Vec<String>,
    pub editor: Option<String>,
    pub project_markers: Option<Vec<String>>,
    pub max_depth: Option<u32>,
    pub preview_max_size: Option<u32>,
}

pub fn load_config() -> Result<Config, String> {
    let config_path = dirs::config_dir().ok_or("Failed to get config directory")?;
    let config_path = config_path.join("blink").join("config.toml");

    let config = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    toml::from_str(&config).map_err(|e| format!("Failed to parse config file: {}", e))
}

pub fn load_config_from_path(path: &std::path::Path) -> Result<Config, String> {
    let config = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read config file: {}", e))?;

    toml::from_str(&config).map_err(|e| format!("Failed to parse config file: {}", e))
}
