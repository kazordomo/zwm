use std::fs;
use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Theme {
    default_border_color: String,
}

impl Theme {
    pub fn load() {
        let data = load_theme_config();

        println!("{}", data.default_border_color);
    }
}

fn load_theme_config() -> Theme {
    let content = match fs::read_to_string("config.toml") {
        Ok(c) => c,
        Err(e) => {
            panic!("Could not read the config file: {}", e);
        }
    };

    let data: Theme = match toml::from_str(&content) {
        Ok(d) => d,
        Err(e) => {
            panic!("Error converting config file: {}", e);
        }
    };

    data
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_from_config_file() -> std::io::Result<()> {
        load_theme_config();
        Ok(())
    }

    #[test]
    fn load_theme() -> std::io::Result<()> {
        Theme::load();
        Ok(())
    }
}