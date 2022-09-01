#[macro_use]
extern crate penrose;
use penrose::{
    core::{
        config::Config, hooks::Hooks,
    },
    Result, XcbConnection,
};
use simplelog::{LevelFilter, SimpleLogger};

mod config;
mod scripts;

use config::{keybindings, theme};
use scripts::on_startup;

fn main() -> Result<()> {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e)
    };

    let config = Config::default();
    let hooks: Hooks<XcbConnection> = vec![Box::new(on_startup::StartupScript::new("/usr/local/scripts/zwm-startup.sh"))];

    theme::Theme::set(&config);
    keybindings::Keybindings::set(config, hooks);

    Ok(())
}