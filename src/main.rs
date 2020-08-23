extern crate gtk;
extern crate serde;
extern crate serde_json;

use gio::prelude::*;
use gtk::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering::Relaxed};
use std::sync::{Arc, RwLock};

const GLADE_SRC: &'static str = include_str!("./ui.glade");

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

fn save_to_new_file(content: &str, quit_after_save: bool) {
    let dialog = gtk::FileChooserDialogBuilder::new()
        .title("Choose Save Location")
        .action(gtk::FileChooserAction::Save)
        .name("untitled.json")
        .build();

    dialog.add_button("Cancel", gtk::ResponseType::Cancel);
    dialog.add_button("Save", gtk::ResponseType::Ok);

    dialog.connect_response({
        let content = content.to_owned();

        move |dialog, event| {
            match event {
                gtk::ResponseType::Ok => {
                    let file = dialog.get_file().unwrap();

                    if let Some(path) = file.get_path() {
                        if let Ok(_) = std::fs::write(&path, &content) {
                            dialog.hide();
                            if quit_after_save {
                                gtk::main_quit();
                            }
                        } else {
                            let dialog = gtk::MessageDialogBuilder::new()
                                .message_type(gtk::MessageType::Error)
                                .title("Error Saving File")
                                .build();
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

fn save_to_old_file(path: &Path, content: &str, quit_after_save: bool) {
    if quit_after_save {
        gtk::main_quit();
    }
}

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let builder = gtk::Builder::from_string(GLADE_SRC);

    let window: gtk::ApplicationWindow = builder.get_object("window-main").unwrap();
    let current_file: Arc<RwLock<Option<PathBuf>>> = Arc::new(RwLock::new(None));
    let dirty: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

    let store_types = [String::static_type(), String::static_type()];
    let title_column_id = 0;
    let description_column_id = 1;
    let store = gtk::ListStore::new(&store_types);

    // Handle TreeView
    {
        let treeview_todo: gtk::TreeView = builder.get_object("treeview-todo").unwrap();

        let title_cell = gtk::CellRendererText::new();
        let description_cell = gtk::CellRendererText::new();

        let title_column = gtk::TreeViewColumnBuilder::new()
            .title("Title")
            .sizing(gtk::TreeViewColumnSizing::Autosize)
            .resizable(true)
            .sort_column_id(title_column_id)
            .build();
        let description_column = gtk::TreeViewColumnBuilder::new()
            .title("Description")
            .sizing(gtk::TreeViewColumnSizing::Autosize)
            .resizable(true)
            .sort_column_id(description_column_id)
            .build();

        title_column.pack_start(&title_cell, true);
        description_column.pack_start(&description_cell, true);

        title_column.add_attribute(&title_cell, "text", title_column_id);
        description_column.add_attribute(&description_cell, "text", description_column_id);

        treeview_todo.append_column(&title_column);
        treeview_todo.append_column(&description_column);

        treeview_todo
            .get_selection()
            .set_mode(gtk::SelectionMode::Multiple);
        treeview_todo.set_model(Some(&store));
    }

    // Handle Adding To Do
    let button_add_todo: gtk::Button = builder.get_object("button-add-todo").unwrap();
    {
        let store = store.clone();
        let dirty = dirty.clone();
        button_add_todo.connect_clicked(move |_event| {
            let store = store.clone();

            let builder = gtk::Builder::from_string(GLADE_SRC);
            let window: gtk::Window = builder.get_object("window-add-todo").unwrap();
            let button_confirm_add_todo: gtk::Button =
                builder.get_object("button-confirm-add-todo").unwrap();
            let entry_title: gtk::Entry = builder.get_object("entry-title").unwrap();
            let textview_description: gtk::TextView =
                builder.get_object("textview-description").unwrap();

            {
                let window = window.clone();
                let dirty = dirty.clone();
                button_confirm_add_todo.connect_clicked(move |_button| {
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
                });
            }

            window.show_all();
        });
    }

    // Handle Deleting Selection
    let button_delete_selection: gtk::Button =
        builder.get_object("button-delete-selection").unwrap();
    {
        let store = store.clone();
        let treeview_todo: gtk::TreeView = builder.get_object("treeview-todo").unwrap();
        button_delete_selection.connect_clicked(move |_button| {
            let (paths, model) = treeview_todo.get_selection().get_selected_rows();
            for path in paths.iter().rev() {
                store.remove(&model.get_iter(path).unwrap());
            }
        });
    }

    let save = {
        let current_file = current_file.clone();
        let store = store.clone();

        Arc::new(Box::new(move |quit_after_save: bool| {
            let dirty = dirty.load(Relaxed);
            println!("Dirty: {}", dirty);

            if dirty {
                let todos = get_todos(&store);
                let json = serde_json::to_string(&todos).unwrap();

                if let Some(path) = current_file.read().unwrap().clone() {
                    println!("Old");
                    save_to_old_file(&path, &json, quit_after_save);
                } else {
                    println!("New");
                    save_to_new_file(&json, quit_after_save);
                }
            } else {
                if quit_after_save {
                    gtk::main_quit();
                }
            }
        }))
    };

    // Handle menu
    {
        let menu_button_new: gtk::MenuItem = builder.get_object("menu-button-new").unwrap();
        let menu_button_open: gtk::MenuItem = builder.get_object("menu-button-open").unwrap();
        let menu_button_save: gtk::MenuItem = builder.get_object("menu-button-save").unwrap();

        menu_button_save.connect_select({
            let save = save.clone();
            move |button| {
                save(false);
            }
        });
    }

    // Handle Quitting Application
    {
        let save = save.clone();
        window.connect_delete_event(move |_window, _event| {
            save(true);
            Inhibit(true)
        });
    }

    window.show_all();

    gtk::main();
}
