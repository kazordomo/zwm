use serde_derive::Deserialize;
use std::fs;

// TODO: default config created - cp -> config.toml and alter that file for user preferences

#[derive(Deserialize)]
pub struct Programs {
    pub terminal: String,
    pub launcher: String,
    pub browser: String,
}

#[derive(Deserialize)]
pub struct Keybindings {
    pub exit: String,
}

#[derive(Deserialize)]
pub struct Theme {
    pub default_border_color: String,
}


#[derive(Deserialize)]
pub struct Data {
    pub programs: Programs,
    pub keybindings: Keybindings,
    pub theme: Theme,
}

const CONFIG_FILE_NAME: &str = "config.toml";

impl Data {
    pub fn load() -> Data {
        let content = match fs::read_to_string(CONFIG_FILE_NAME) {
            Ok(c) => c,
            Err(_) => {
                panic!("Could not read the config file");
            }
        };

        let data: Data = match toml::from_str(&content) {
            Ok(d) => d,
            Err(e) => {
                panic!("Error converting config file - {}", e);
            }
        };

        data
    }
}

#[cfg(test)]
mod tests {
    use std::io;
    use super::*;

    #[test]
    fn get_config_data() -> io::Result<()> {
        Data::load();
        Ok(())
    }
}