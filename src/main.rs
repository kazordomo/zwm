#[macro_use]
extern crate penrose;

use penrose::{
    core::{
        bindings::MouseEvent, config::Config, helpers::index_selectors, hooks::Hooks, hooks::Hook,
        manager::WindowManager, xconnection::XConn,
    },
    logging_error_handler,
    spawn,
    xcb::new_xcb_backed_window_manager,
    Backward, Forward, Less, More, Result, XcbConnection,
};

use simplelog::{LevelFilter, SimpleLogger};

pub struct StartupScript {
    path: String,
}

impl StartupScript {
    pub fn new(s: impl Into<String>) -> Self {
        Self { path: s.into() }
    } 
}

impl <X: XConn> Hook<X> for StartupScript {
    fn startup(&mut self, _: &mut WindowManager<X>) -> Result<()> {
        spawn!(&self.path)
    }
}

const TERMINAL: &str = "alacritty";
const LAUNCHER: &str = "dmenu_run";
const STARTUP_HOOK_PATH: &str = "$HOME/zwm";

fn main() -> Result<()> {
    if let Err(e) = SimpleLogger::init(LevelFilter::Info, simplelog::Config::default()) {
        panic!("unable to set log level: {}", e)
    };

    let config = Config::default();
    let hooks: Hooks<XcbConnection> = vec![Box::new(StartupScript::new(STARTUP_HOOK_PATH))];

    let key_bindings = gen_keybindings! {
        "M-j" => run_internal!(cycle_client, Forward);
        "M-k" => run_internal!(cycle_client, Backward);
        "M-S-j" => run_internal!(drag_client, Forward);
        "M-S-k" => run_internal!(drag_client, Backward);
        "M-S-q" => run_internal!(kill_client);
        "M-Tab" => run_internal!(toggle_workspace);
        "M-bracketright" => run_internal!(cycle_screen, Forward);
        "M-bracketleft" => run_internal!(cycle_screen, Backward);
        "M-S-bracketright" => run_internal!(drag_workspace, Forward);
        "M-S-bracketleft" => run_internal!(drag_workspace, Backward);
        "M-grave" => run_internal!(cycle_layout, Forward);
        "M-S-grave" => run_internal!(cycle_layout, Backward);
        "M-A-Up" => run_internal!(update_max_main, More);
        "M-A-Down" => run_internal!(update_max_main, Less);
        "M-A-Right" => run_internal!(update_main_ratio, More);
        "M-A-Left" => run_internal!(update_main_ratio, Less);
        "M-A-Escape" => run_internal!(exit);
        "M-Return" => run_external!(TERMINAL);
        "M-p" => run_external!(LAUNCHER);

        map: { "1", "2", "3", "4", "5", "6", "7", "8", "9" } to index_selectors(9) => {
            "M-{}" => focus_workspace (REF);
            "M-S-{}" => client_to_workspace (REF);
        };
    };

    let mouse_bindings = gen_mousebindings! {
        Press Right + [Meta] => |wm: &mut WindowManager<_>, _: &MouseEvent| wm.cycle_workspace(Forward),
        Press Left + [Meta] => |wm: &mut WindowManager<_>, _: &MouseEvent| wm.cycle_workspace(Backward)
    };

    let mut wm = new_xcb_backed_window_manager(config, hooks, logging_error_handler())?;
    wm.grab_keys_and_run(key_bindings, mouse_bindings)?;

    Ok(())
}
