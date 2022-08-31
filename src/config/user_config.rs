use serde_derive::Deserialize;
use std::fs;

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

impl Data {
    pub fn load() -> Data {
        let content = match fs::read_to_string("config.toml") {
            Ok(c) => c,
            Err(e) => {
                panic!("Could not read the config file: {}", e);
            }
        };

        let data: Data = match toml::from_str(&content) {
            Ok(d) => d,
            Err(e) => {
                panic!("Error converting config file: {}", e);
            }
        };

        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_config_data() -> std::io::Result<()> {
        Data::load();
        Ok(())
    }
}