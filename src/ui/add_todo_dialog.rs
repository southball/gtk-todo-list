use crate::state::AppState;

use gtk::prelude::*;
use crate::ui::MainWindowUI;
use std::sync::atomic::Ordering::Relaxed;

const SRC_ADD_TODO_DIALOG: &'static str = include_str!("./add_todo_dialog.glade");

pub trait AddToDoDialogUI {
    fn handle_add_todo_button(&self, button: &gtk::Button);
}

impl AddToDoDialogUI for AppState {
    fn handle_add_todo_button(&self, button: &gtk::Button) {
        let store = self.store.clone();
        let dirty = self.dirty.clone();
        let app_state = self.clone();

        button.connect_clicked(move |_event| {
            let store = store.clone();
            let app_state = app_state.clone();

            let builder = gtk::Builder::from_string(SRC_ADD_TODO_DIALOG);
            let window: gtk::Window = builder.get_object("window-add-todo").unwrap();
            let button_confirm_add_todo: gtk::Button =
                builder.get_object("button-confirm-add-todo").unwrap();
            let entry_title: gtk::Entry = builder.get_object("entry-title").unwrap();
            let textview_description: gtk::TextView =
                builder.get_object("textview-description").unwrap();

            button_confirm_add_todo.connect_clicked({
                let window = window.clone();
                let dirty = dirty.clone();

                move |_button| {
                    let title = entry_title.get_text();
                    let description = {
                        let buffer = textview_description.get_buffer().unwrap();
                        buffer
                            .get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), true)
                            .unwrap()
                    };
                    dirty.store(true, Relaxed);
                    store.insert_with_values(None, &[0, 1], &[&title, &description]);
                    app_state.update_subtitle();
                    
                    window.hide();
                }
            });

            window.show_all();
        });
    }
}
