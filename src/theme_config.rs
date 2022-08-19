pub struct ThemeConfig {
    pub default_border_color: String,
    pub focused_border_color: String,
}

impl ThemeConfig {
    pub fn load() {
        println!("Theme config calling!");
    }
}

// impl Default for ThemeConfig
