use crate::cli::args::DEFAULT_PASSWORD_FILENAME;
use config::{Config, File};

enum ConfigName {
    StorePath,
}

impl ConfigName {
    fn as_str(&self) -> &'static str {
        match self {
            ConfigName::StorePath => "STORE_PATH",
        }
    }

    fn default(&self) -> &'static str {
        match self {
            ConfigName::StorePath => DEFAULT_PASSWORD_FILENAME,
        }
    }
}

#[derive(Debug)]
pub struct LockboxConfig {
    pub store_path: String,
}

impl LockboxConfig {
    pub fn read_config(config_name: ConfigName) -> String {
        let settings = Config::builder()
            .add_source(File::with_name("config").required(false)) // Use config file (e.g., toml, json, etc.)
            .build()
            .unwrap();

        let value = settings
            .get_string(config_name.as_str())
            .unwrap_or_else(|_| config_name.default().to_string());

        value
    }

    // Function to read the entire config
    pub fn read() -> Result<Self, config::ConfigError> {
        let store_path = LockboxConfig::read_config(ConfigName::StorePath);

        Ok(LockboxConfig { store_path })
    }
}
