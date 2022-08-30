pub mod user_config {
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
}

pub mod keybindings {
    use penrose::{
        core::{
            bindings::MouseEvent, helpers::index_selectors, hooks::Hooks, config::Config,
            manager::WindowManager,
        },
        logging_error_handler,
        xcb::new_xcb_backed_window_manager,
        Backward, Forward, Less, More, Result, XcbConnection,
    };

    pub struct Keybindings {
        pub next_client: String,
        pub prev_client: String,
        pub move_client_next: String,
        pub move_client_prev: String,
        pub close_client: String,
        pub increase_client_size: String,
        pub decrease_client_size: String,
        pub open_launcher: String,
        pub open_terminal: String,
        pub open_browser: String,
        pub exit: String,
    }

    impl Keybindings {
        pub fn set_keybindings(config:Config, hooks: Hooks<XcbConnection>) -> Result<()> {
            let user_data = crate::user_config::Data::load(); 

            // const TERMINAL: &str = "alacritty";
            let mut TERMINAL = user_data.programs.terminal;
            const LAUNCHER: &str = "dmenu_run";
            const BROWSER: &str = "google-chrome";

            println!("{}", &user_data.programs.terminal);

            let keybindings = gen_keybindings! {
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
                "M-b" => run_external!(BROWSER);

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
            wm.grab_keys_and_run(keybindings, mouse_bindings)?;

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn load_user_config() -> std::io::Result<()> {
            let user_data = crate::user_config::Data::load(); 

            println!("{}", user_data.programs.terminal);
            Ok(())
        }
    }
}

pub mod on_startup {
    use penrose::{
        core::{
            hooks::Hook,
            manager::WindowManager, xconnection::XConn,
        },
        spawn,
        Result,
    };

    pub struct StartupScript {
        pub path: String,
    }

    impl StartupScript {
        pub fn new(s: impl Into<String>) -> Self {
            Self { path: s.into() }
        }
    }

    impl<X: XConn> Hook<X> for StartupScript {
        fn startup(&mut self, _: &mut WindowManager<X>) -> Result<()> {
            spawn!(&self.path)
        }
    }

}