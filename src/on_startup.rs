use std::fs;
use serde_derive::Deserialize;
use toml;

#[derive(Deserialize)]
pub struct Data {
    config: UserConfig,
}

#[derive(Deserialize)]
struct UserConfig {
    startup_script_path: String,
    terminal: String,
    launcher: String,
    browser: String,
}

const CONFIG_FILE_NAME: &str = "config.toml";

fn get_content() {
    match fs::read_to_string(CONFIG_FILE_NAME) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Could not read file {}", CONFIG_FILE_NAME);
            panic!()
        }
    }
}

impl Data {
    pub fn run_script() {
        let content = get_content();

        match toml::from_str(&content) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from {}", CONFIG_FILE_NAME);
                panic!()
            }
        };
    }
}

// TODO: set user config below?
