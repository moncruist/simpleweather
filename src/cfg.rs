use std::env;
use std::path::{Path, PathBuf};
use std::ffi::OsString;
use std::io;
use std::fs;

extern crate ini;
use ini::Ini;

const XDG_CONFIG_HOME_VAR: &str = "XDG_CONFIG_HOME";
const DEFAULT_CONFIG_LOCATION: &str = ".config";
const APP_NAME: &str = "simpleweather";
const CFG_FILE_NAME: &str = "config.ini";
const APP_SECTION: &str = "application";
const API_KEY_CONF: &str = "api_key";

pub enum ConfigError {
    FileError(io::Error),
    Missing
}

pub struct AppConfig {
    api_key: String
}

impl AppConfig {
    pub fn new(api_key: &str) -> AppConfig {
        AppConfig { api_key: String::from(api_key) }
    }

    pub fn load() -> Result<AppConfig, ConfigError> {
        let user_config_path = AppConfig::get_config_path();
        let config_path = AppConfig::get_file_path(&user_config_path);

        let conf = match Ini::load_from_file(config_path) {
            Ok(conf) => conf,
            Err(error) => match error {
                ini::ini::Error::Io(io_err) => return Err(ConfigError::FileError(io_err)),
                ini::ini::Error::Parse(_) => return Err(ConfigError::Missing)
            }
        };

        let section = match conf.section(Some(APP_SECTION)) {
            Some(section) => section,
            None => return Err(ConfigError::Missing)
        };

        let api_key = match section.get(API_KEY_CONF) {
            Some(api_key) => api_key,
            None => return Err(ConfigError::Missing)
        };

        Ok(AppConfig { api_key: api_key.to_string() })
    }

    pub fn save(&self) -> io::Result<()> {
        let user_config_path = AppConfig::get_config_path();
        let config_path = AppConfig::get_file_path(&user_config_path);

        let parent = config_path.parent().unwrap();

        // suppress warning of unused result
        let _ = fs::create_dir_all(parent);

        let mut conf = Ini::new();
        conf.with_section(Some(APP_SECTION)).set(API_KEY_CONF, &self.api_key);

        conf.write_to_file(config_path)
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    fn get_config_path() -> OsString {
        if let Ok(env_path) = env::var(XDG_CONFIG_HOME_VAR) {
            OsString::from(Path::new(&env_path))
        } else {
            let home_path = env::var("HOME").unwrap();
            OsString::from(Path::new(&home_path).join(DEFAULT_CONFIG_LOCATION))
        }
    }

    fn get_file_path(folder_path: &OsString) -> PathBuf {
        Path::new(folder_path).join(APP_NAME).join(CFG_FILE_NAME)
    }
}