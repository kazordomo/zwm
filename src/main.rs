#[macro_use]
extern crate penrose;
extern crate simplelog;
extern crate log;

use penrose::{
    core::{
        config::Config, hooks::Hooks,
    },
    Result, XcbConnection,
};
use simplelog::{LevelFilter, WriteLogger};

mod config;
mod scripts;

use config::{keybindings, theme};
use scripts::on_startup;

fn main() -> Result<()> {
    let _ = WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), std::fs::File::create("error.log").unwrap());

    let config = Config::default();
    let hooks: Hooks<XcbConnection> = vec![Box::new(on_startup::StartupScript::new("/usr/local/scripts/zwm-startup.sh"))];

    // TODO: test with all config in here
    // TODO: write error to file
    theme::Theme::set(&config);
    keybindings::Keybindings::set(config, hooks);

    Ok(())
}