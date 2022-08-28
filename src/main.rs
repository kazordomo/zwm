#[macro_use]
extern crate penrose;
use penrose::{
    core::{
        config::Config, hooks::Hooks,
    },
    Result, XcbConnection,
};
use simplelog::{LevelFilter, SimpleLogger};

mod on_startup;
mod keybindings;
mod user_config;

fn main() -> Result<()> {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e)
    };

    // TODO: log config data and add user keybindings
    let config_data = user_config::Data::load();

    println!("{}", config_data.keybindings.exit);

    let config = Config::default();
    let hooks: Hooks<XcbConnection> = vec![Box::new(on_startup::StartupScript::new("/usr/local/scripts/zwm-startup.sh"))];

    keybindings::Keybindings::set_keybindings(config, hooks)
}
