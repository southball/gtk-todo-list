extern crate gtk;

use gtk::prelude::*;

const GLADE_SRC: &'static str = include_str!("./ui.glade");

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let builder = gtk::Builder::from_string(GLADE_SRC);

    let window: gtk::ApplicationWindow = builder.get_object("window-main").unwrap();

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
                button_confirm_add_todo.connect_clicked(move |_button| {
                    let title = entry_title.get_text();
                    let description = {
                        let buffer = textview_description.get_buffer().unwrap();
                        buffer
                            .get_text(&buffer.get_start_iter(), &buffer.get_end_iter(), true)
                            .unwrap()
                    };
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

    // Handle Quitting Application
    window.connect_delete_event(|_window, _event| {
        gtk::main_quit();
        Inhibit(true)
    });

    window.show_all();

    gtk::main();
}
