use crate::config::settings::OrbitConfig;
use crate::error::OrbitError;
use std::fs;
use std::path::PathBuf;

pub fn load_config() -> Result<OrbitConfig, OrbitError> {
    let local_config = PathBuf::from("orbit.toml");
    if local_config.exists() {
        return load_from_file(&local_config);
    }

    if let Some(config_dir) = dirs::config_dir() {
        let global_config = config_dir.join("orbit").join("orbit.toml");
        if global_config.exists() {
            return load_from_file(&global_config);
        }
    }

    Ok(OrbitConfig::default())
}

fn load_from_file(path: &PathBuf) -> Result<OrbitConfig, OrbitError> {
    let contents = fs::read_to_string(path).map_err(|e| OrbitError::Other(e.to_string()))?;

    toml::from_str(&contents).map_err(|e| OrbitError::Other(e.to_string()))
}

pub fn save_config(config: &OrbitConfig) -> Result<(), OrbitError> {
    let config_dir = dirs::config_dir().ok_or(OrbitError::Other(
        "Could not find config directory".to_string(),
    ))?;

    let orbit_dir = config_dir.join("orbit");

    fs::create_dir_all(&orbit_dir).map_err(|e| OrbitError::Other(e.to_string()))?;

    let config_path = orbit_dir.join("orbit.toml");

    let file_content =
        toml::to_string_pretty(config).map_err(|e| OrbitError::Other(e.to_string()))?;
    fs::write(config_path, file_content).map_err(|e| OrbitError::Other(e.to_string()))?;

    Ok(())
}
