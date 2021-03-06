//! Implementation of menus for `Window`

extern crate nfd;

use super::Window;

use minifb;
use core::view_plugins::ViewPlugins;
use core::backend_plugin::BackendPlugins;
use core::session::{Session, Sessions};
use menu::*;
use imgui_sys::Imgui;
use prodbg_api::Ui;
use prodbg_api::events;
use self::nfd::Response as NfdResponse;


fn render_unix_menu(ui: &Ui, menu: &minifb::UnixMenu) -> Option<usize> {
    let mut res = None;
    if ui.begin_menu(&menu.name, true) {
        for item in menu.items.iter() {
            if let Some(ref sub_menu) = item.sub_menu {
                res = res.or(render_unix_menu(ui, sub_menu));
            } else {
                if item.label.is_empty() {
                    ui.separator();
                } else {
                    if ui.menu_item(&item.label, false, true) {
                        res = Some(item.id);
                    }
                }
            }
        }
        ui.end_menu();
    }
    res
}

impl Window {
    pub fn update_menus(&mut self,
                        view_plugins: &mut ViewPlugins,
                        sessions: &mut Sessions,
                        backend_plugins: &mut BackendPlugins) {
        let current_session = sessions.get_current();

        let menu_id = match self.show_unix_menus().or_else(|| self.win.is_menu_pressed()) {
            Some(id) => id,
            None => return,
        };

        match menu_id {
            MENU_DEBUG_CONFIGURE => {
                self.config_backend = current_session.backend;
            }
            MENU_DEBUG_STEP_IN => current_session.action_step(),
            MENU_DEBUG_STEP_OVER => current_session.action_step_over(),
            MENU_DEBUG_STOP => current_session.action_stop(),
            MENU_DEBUG_START => current_session.action_run(),
            MENU_FILE_OPEN_SOURCE => self.browse_source_file(view_plugins, current_session),
            MENU_DEBUG_TOGGLE_BREAKPOINT => {
                let writer = current_session.get_current_writer();
                writer.event_begin(events::EVENT_TOGGLE_BREAKPOINT_CURRENT_LINE as u16);
                writer.event_end();
            }
            _ => {
                if menu_id >= MENU_FILE_BACKEND_START && menu_id < MENU_FILE_BACKEND_END {
                    let backend_index = menu_id - MENU_FILE_BACKEND_START;
                    // TODO: the correct way here is to fetch the menu name instead of doing
                    // this as new plugins may have been added to the menu and this will miss-match
                    // right now there is no way to do that in minifb so this will have to be
                    // ok for now
                    let names = backend_plugins.get_plugin_names();
                    let backend_name = &names[backend_index];
                    // TODO: Default values for backend
                    self.config_backend = backend_plugins.create_instance(backend_name, &None);

                    // TODO: Need to remove the menu when switching backend
                    if let Some(menu) = backend_plugins.get_menu(self.config_backend.unwrap(), self.menu_id_offset) {
                        self.win.add_menu(&(*menu));
                        self.menu_id_offset += 1000;
                    }
                } else {
                    current_session.send_menu_id(menu_id as u32, backend_plugins);
                }
            }
        }
    }

    fn show_unix_menus(&mut self) -> Option<usize> {
        // TODO: process unix menus shortcuts
        let mut res = None;
        if let Some(menus) = self.win.get_unix_menus() {
            let ui = Imgui::get_ui();
            if ui.begin_main_menu_bar() {
                for menu in menus {
                    res = res.or(render_unix_menu(&ui, menu));
                }
                self.custom_menu_height = ui.get_window_size().y;
                ui.end_main_menu_bar();
            }
        }
        res
    }

    fn browse_source_file(&mut self, view_plugins: &mut ViewPlugins, session: &mut Session) {
        match nfd::dialog().open() {
            Ok(NfdResponse::Cancel) => return,
            Err(e) => println!("Failed to open file dialog {:?}", e),
            Ok(NfdResponse::Okay(file)) => self.open_source_file(&file, view_plugins, session),
            _ => (),
        }
    }
}
