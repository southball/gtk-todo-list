use crate::state::AppState;

use gtk::prelude::*;
use std::sync::atomic::Ordering::Relaxed;

const SRC_ADD_TODO: &'static str = include_str!("./add_todo.glade");

pub trait AddToDoUI {
    fn handle_add_todo_button(&self, button: &gtk::Button);
}

impl AddToDoUI for AppState {
    fn handle_add_todo_button(&self, button: &gtk::Button) {
        let store = self.store.clone();
        let dirty = self.dirty.clone();

        button.connect_clicked(move |_event| {
            let store = store.clone();

            let builder = gtk::Builder::from_string(SRC_ADD_TODO);
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

                    window.hide();
                }
            });

            window.show_all();
        });
    }
}
