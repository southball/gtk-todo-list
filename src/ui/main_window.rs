use crate::state::AppState;
use gtk::prelude::*;

use crate::client::*;
use crate::ui::*;

pub trait MainWindowUI {
    fn handle_main_window(&self, builder: &gtk::Builder);
}

impl MainWindowUI for AppState {
    fn handle_main_window(&self, builder: &gtk::Builder) {
        let app_state = self.clone();
        let main_window = self.main_window.as_ref().clone();

        // Handle TreeView
        let treeview_todo: gtk::TreeView = builder.get_object("treeview-todo").unwrap();
        app_state.populate_treeview_todo(&treeview_todo);

        // Handle Adding To Do
        let button_add_todo: gtk::Button = builder.get_object("button-add-todo").unwrap();
        app_state.handle_add_todo_button(&button_add_todo);

        // Handle Deleting Selection
        let button_delete_selection: gtk::Button =
            builder.get_object("button-delete-selection").unwrap();
        app_state.handle_delete_selection_button(&button_delete_selection, &treeview_todo);

        // Handle menu
        let _menu_button_new: gtk::MenuItem = builder.get_object("menu-button-new").unwrap();
        let _menu_button_open: gtk::MenuItem = builder.get_object("menu-button-open").unwrap();
        let menu_button_save: gtk::MenuItem = builder.get_object("menu-button-save").unwrap();
        let _menu_button_save_as: gtk::MenuItem =
            builder.get_object("menu-button-save-as").unwrap();

        menu_button_save.connect_activate({
            let app_state = app_state.clone();
            move |_button| {
                app_state.save(None);
            }
        });

        // Handle Quitting Application
        main_window.connect_delete_event({
            let app_state = app_state.clone();
            move |_window, _event| {
                app_state.save_and_quit();
                Inhibit(true)
            }
        });

        main_window.show_all();
    }
}
