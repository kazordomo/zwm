use std::fs;
use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Data {
    config: UserConfig,
}

const CONFIG_FILE_NAME: &str = "config.toml";

pub struct ThemeConfig {
    pub default_border_color: String,
    pub focused_border_color: String,
}

impl ThemeConfig {
    pub fn load() {
        let content = fs::read_to_string(CONFIG_FILE_NAME).unwrap();

        match toml::from_str(&content) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from {}", CONFIG_FILE_NAME);
                panic!()
            }
        }
    }
}

// impl Default for ThemeConfig
