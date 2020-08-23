extern crate gtk;
extern crate serde;
extern crate serde_json;

use gio::prelude::*;
use gtk::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{Arc, RwLock, Mutex};

const SRC_MAIN_WINDOW: &'static str = include_str!("./ui/main_window.glade");
const SRC_ADD_NEW_TODO: &'static str = include_str!("./ui/add_new_todo.glade");

const TITLE_COLUMN_ID: i32 = 0i32;
const DESCRIPTION_COLUMN_ID: i32 = 1i32;

#[derive(Debug, Serialize, Deserialize)]
struct ToDo {
    pub title: Option<String>,
    pub description: Option<String>,
}

fn get_todos(store: &gtk::ListStore) -> Vec<ToDo> {
    let mut todos: Vec<ToDo> = vec![];
    store.foreach(|model, _path, iter| {
        let title: Option<String> = model
            .get_value(iter, TITLE_COLUMN_ID)
            .get::<String>()
            .unwrap();
        let description: Option<String> = model
            .get_value(iter, DESCRIPTION_COLUMN_ID)
            .get::<String>()
            .unwrap();
        todos.push(ToDo { title, description });
        false
    });
    todos
}

fn save_to_new_file(
    parent: &gtk::Window,
    content: &str,
    callback: Option<Arc<Mutex<Box<dyn Fn(bool) -> ()>>>>,
) {
    let dialog = gtk::FileChooserDialog::new(
        Some("Choose Save Location"),
        Some(parent),
        gtk::FileChooserAction::Save,
    );
    dialog.set_filename("untitled.json");

    dialog.add_button("Cancel", gtk::ResponseType::Cancel);
    dialog.add_button("Save", gtk::ResponseType::Ok);

    dialog.connect_response({
        let callback = callback.map(|callback: Arc<_>| callback.clone());
        let content = content.to_owned();
        let parent = parent.clone();

        move |dialog, event| {
            match event {
                gtk::ResponseType::Ok => {
                    let file = dialog.get_file().unwrap();

                    if let Some(path) = file.get_path() {
                        if let Ok(_) = std::fs::write(&path, &content) {
                            dialog.hide();
                            callback.as_ref().and_then(|callback| { 
                                let callback = callback.lock().unwrap();
                                callback(true);
                                Some(())
                            });
                        } else {
                            let dialog = gtk::MessageDialog::new::<gtk::Window>(
                                Some(&parent),
                                gtk::DialogFlags::MODAL | gtk::DialogFlags::USE_HEADER_BAR,
                                gtk::MessageType::Error,
                                gtk::ButtonsType::Ok,
                                "Error saving file.",
                            );
                            dialog.set_title("Error");
                            dialog.get_message_area().and_then(|message_area| {
                                message_area.set_valign(gtk::Align::Center);
                                Some(())
                            });
                            dialog.connect_response(|dialog, response| {
                                if response == gtk::ResponseType::Ok {
                                    dialog.hide();
                                }
                            });
                            dialog.show_all();
                        }
                    }
                }
                gtk::ResponseType::Cancel => {
                    dialog.hide();
                }
                _ => {
                    // Cancelled
                }
            }
        }
    });

    dialog.show_all();
}

fn save_to_old_file(
    parent: &gtk::Window,
    path: &Path,
    content: &str,
    callback: Option<Arc<Mutex<Box<dyn Fn(bool) -> ()>>>>
) {
    if let Ok(_) = std::fs::write(path, content) {
        callback.and_then(|callback| { 
            let callback = callback.lock().unwrap();
            callback(true); Some(())
        });
    } else {
        let dialog = gtk::MessageDialog::new(
            Some(parent),
            gtk::DialogFlags::MODAL | gtk::DialogFlags::USE_HEADER_BAR,
            gtk::MessageType::Error,
            gtk::ButtonsType::Ok,
            "Error saving file.",
        );
        dialog.set_title("Error");
        dialog.get_message_area().and_then(|message_area| {
            message_area.set_valign(gtk::Align::Center);
            Some(())
        });
        dialog.connect_response(|dialog, response| {
            if response == gtk::ResponseType::Ok {
                dialog.hide();
            }
        });
        dialog.show();
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let builder = gtk::Builder::from_string(SRC_MAIN_WINDOW);

    let window: gtk::ApplicationWindow = builder.get_object("window-main").unwrap();

    // Application State
    let current_file: Arc<RwLock<Option<PathBuf>>> = Arc::new(RwLock::new(None));
    let dirty: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    // Application Store
    let store_types = [String::static_type(), String::static_type()];
    let store = gtk::ListStore::new(&store_types);

    // Handle TreeView
    let treeview_todo: gtk::TreeView = builder.get_object("treeview-todo").unwrap();
    {
        let title_cell = gtk::CellRendererText::new();
        let description_cell = gtk::CellRendererText::new();

        let title_column = gtk::TreeViewColumnBuilder::new()
            .title("Title")
            .sizing(gtk::TreeViewColumnSizing::Autosize)
            .resizable(true)
            .sort_column_id(TITLE_COLUMN_ID)
            .build();
        let description_column = gtk::TreeViewColumnBuilder::new()
            .title("Description")
            .sizing(gtk::TreeViewColumnSizing::Autosize)
            .resizable(true)
            .sort_column_id(DESCRIPTION_COLUMN_ID)
            .build();

        title_column.pack_start(&title_cell, true);
        description_column.pack_start(&description_cell, true);

        title_column.add_attribute(&title_cell, "text", TITLE_COLUMN_ID);
        description_column.add_attribute(&description_cell, "text", DESCRIPTION_COLUMN_ID);

        treeview_todo.append_column(&title_column);
        treeview_todo.append_column(&description_column);

        treeview_todo
            .get_selection()
            .set_mode(gtk::SelectionMode::Multiple);
        treeview_todo.set_model(Some(&store));
    }

    // Handle Adding To Do
    let button_add_todo: gtk::Button = builder.get_object("button-add-todo").unwrap();
    button_add_todo.connect_clicked({
        let store = store.clone();
        let dirty = dirty.clone();

        move |_event| {
            let store = store.clone();

            let builder = gtk::Builder::from_string(SRC_ADD_NEW_TODO);
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
        }
    });

    // Handle Deleting Selection
    let button_delete_selection: gtk::Button =
        builder.get_object("button-delete-selection").unwrap();
    button_delete_selection.connect_clicked({
        let store = store.clone();
        let treeview_todo: gtk::TreeView = builder.get_object("treeview-todo").unwrap();

        move |_button| {
            let (paths, model) = treeview_todo.get_selection().get_selected_rows();
            for path in paths.iter().rev() {
                store.remove(&model.get_iter(path).unwrap());
            }
        }
    });

    let save = {
        let current_file = current_file.clone();
        let store = store.clone();
        let dirty = dirty.clone();
        let window = window.clone();

        Arc::new(Box::new(move |quit_after_save: bool| {
            let dirty = dirty.load(Relaxed);
            let window = window.clone();

            if dirty {
                let todos = get_todos(&store);
                let json = serde_json::to_string(&todos).unwrap();

                let callback = Arc::new(Mutex::new(Box::new(move |saved: bool| {
                    if saved {
                        gtk::main_quit();
                    }
                }) as Box<dyn Fn(bool) -> ()>));

                if let Some(path) = current_file.read().unwrap().clone() {
                    save_to_old_file(&window.upcast(), &path, &json, Some(callback));
                } else {
                    save_to_new_file(&window.upcast(), &json, Some(callback));
                }
            } else {
                if quit_after_save {
                    gtk::main_quit();
                }
            }
        }))
    };

    let save_on_quit = {
        let save = save.clone();
        let dirty = dirty.clone();
        let window = window.clone();

        Arc::new(Box::new(move || {
            let save = save.clone();
            let dirty = dirty.load(Relaxed);
            let window = window.clone();

            let dialog = gtk::MessageDialog::new::<gtk::Window>(
                Some(&window.upcast()),
                gtk::DialogFlags::MODAL | gtk::DialogFlags::USE_HEADER_BAR,
                gtk::MessageType::Question,
                gtk::ButtonsType::YesNo,
                "You have unsaved changes. Do you want to save?",
            );

            dialog.get_message_area().and_then(|message_area| {
                message_area.set_valign(gtk::Align::Center);
                Some(())
            });

            dialog.connect_response(move |dialog, response| {
                match response {
                    gtk::ResponseType::Yes => {
                        dialog.hide();
                        save(true);
                    }
                    gtk::ResponseType::No => {
                        gtk::main_quit();
                    }
                    _ => {
                        // Cancelled
                    }
                }
            });

            if dirty {
                dialog.show();
            } else {
                gtk::main_quit();
            }
        }))
    };

    // Handle menu
    let menu_button_new: gtk::MenuItem = builder.get_object("menu-button-new").unwrap();
    let menu_button_open: gtk::MenuItem = builder.get_object("menu-button-open").unwrap();
    let menu_button_save: gtk::MenuItem = builder.get_object("menu-button-save").unwrap();

    menu_button_save.connect_select({
        let save = save.clone();
        let window = window.clone();

        move |_button| {
            save(false);
        }
    });

    // Handle Quitting Application
    window.connect_delete_event({
        let save_on_quit = save_on_quit.clone();
        let window = window.clone();

        move |_window, _event| {
            save_on_quit();
            Inhibit(true)
        }
    });

    window.show_all();

    gtk::main();
}
