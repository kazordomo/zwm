use std::fs;
use serde_derive::Deserialize;

use penrose::core::config::Config;

#[derive(Deserialize)]
pub struct BorderConfig {
    pub focused_color: String,
    pub unfocused_color: String,
}

#[derive(Deserialize)]
pub struct BarConfig {
    pub show: bool,
}

#[derive(Deserialize)]
pub struct Theme {
    pub border: BorderConfig,
    pub bar: BarConfig,
}

impl Theme {
    pub fn set(config: &Config) {
        // TODO: error loading theme
        let data = load_theme_config();

        if let Err(e) = config.builder()
            .focused_border(data.border.focused_color)
            .unwrap()
            .build() {
                log::error!("Could not read the config file: {}", e);
                std::process::exit(1);
            }
    }
}

fn load_theme_config() -> Theme {
    let content = match fs::read_to_string("theme.toml") {
        Ok(c) => c,
        Err(e) => {
            log::error!("Could not read the config file: {}", e);
            std::process::exit(1);
        }
    };

    let data: Theme = match toml::from_str(&content) {
        Ok(d) => d,
        Err(e) => {
            log::error!("Could not convert the config file: {}", e);
            std::process::exit(1);
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
    fn set_theme() -> std::io::Result<()> {
        Theme::set(&Config::default());
        Ok(())
    }
}