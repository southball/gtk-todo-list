use crate::state::AppState;

use gtk::prelude::*;
use crate::client::*;

pub trait MainMenuUI {
    fn handle_menu_button_new(&self, button: &gtk::MenuItem);
    fn handle_menu_button_open(&self, button: &gtk::MenuItem);
    fn handle_menu_button_save(&self, button: &gtk::MenuItem);
    fn handle_menu_button_save_as(&self, button: &gtk::MenuItem);
}

impl MainMenuUI for AppState {
    fn handle_menu_button_new(&self, button: &gtk::MenuItem) {
        button.connect_activate({
            let app_state = self.clone();
            move |_button| {
                app_state.new(None);
            }
        });
    }

    fn handle_menu_button_open(&self, button: &gtk::MenuItem) {
        button.connect_activate({
            let app_state = self.clone();
            move |_button| {
                app_state.open(None);
            }
        });
    }

    fn handle_menu_button_save(&self, button: &gtk::MenuItem) {
        button.connect_activate({
            let app_state = self.clone();
            move |_button| {
                app_state.save(None);
            }
        });
    }

    fn handle_menu_button_save_as(&self, button: &gtk::MenuItem) {
        button.connect_activate({
            let app_state = self.clone();
            move |_button| {
                app_state.save_as(None);
            }
        });
    }
}
