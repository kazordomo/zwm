use std::fs;
use serde_derive::Deserialize;

use penrose::core::config::Config;

#[derive(Deserialize)]
struct BorderConfig {
    focused_color: String,
    unfocused_color: String,
}

#[derive(Deserialize)]
struct BarConfig {
    show: bool,
}

#[derive(Deserialize)]
pub struct Theme {
    border: BorderConfig,
    bar: BarConfig,
}

impl Theme {
    pub fn set(config: &Config) {
        // TODO: error loading theme
        let data = load_theme_config();

        config.builder()
            .focused_border(data.border.focused_color)
            .unwrap()
            .build()
            .expect("Failed to set focused border color.");

        config.builder()
            .unfocused_border(data.border.unfocused_color)
            .unwrap()
            .build()
            .expect("Failed to set unfocused border color.");
            
        config.builder()
            .show_bar(data.bar.show)
            .build()
            .expect("Failed to set show bar value.");
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