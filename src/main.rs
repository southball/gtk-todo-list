extern crate gio;
extern crate glib;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);
    
    window.set_title("Todo List");
    window.set_border_width(10);
    window.set_position(gtk::WindowPosition::Center);
    window.set_default_size(800, 600);

    let grid = gtk::Grid::new();

    let column_type = [String::static_type(), String::static_type()];
    let title_column_id = 0i32;
    let description_column_id = 1i32;

    let list_store = gtk::ListStore::new(&column_type);

    let title_input_buffer = gtk::EntryBuffer::new(None);
    let description_input_buffer = gtk::EntryBuffer::new(None);
    let title_input = gtk::EntryBuilder::new()
        .placeholder_text("Title")
        .buffer(&title_input_buffer)
        .build();
    let description_input = gtk::EntryBuilder::new()
        .placeholder_text("Description")
        .buffer(&description_input_buffer)
        .build();
    let add_button = gtk::ButtonBuilder::new()
        .label("Add Todo")
        .build();

    let title_input_buffer_clone = title_input_buffer.clone();
    let description_input_buffer_clone = description_input_buffer.clone();
    let list_store_clone = list_store.clone();
    add_button.connect_clicked(move |e| {
        list_store_clone.insert_with_values(
            None,
            &[0, 1],
            &[
                &title_input_buffer_clone.get_text(),
                &description_input_buffer_clone.get_text(),
            ],
        );
    });

    grid.attach(&title_input, 0, 1, 1, 1);
    grid.attach(&description_input, 1, 1, 1, 1);
    grid.attach(&add_button, 2, 1, 1, 1);
    
    let todo_view = gtk::TreeView::new();
    let title_renderer = gtk::CellRendererText::new();
    let description_renderer = gtk::CellRendererText::new();
    let title_column = gtk::TreeViewColumn::new();
    let description_column = gtk::TreeViewColumn::new();

    todo_view.set_model(Some(&list_store));

    title_column.set_title("Title");
    title_column.pack_start(&title_renderer, true);
    title_column.add_attribute(&title_renderer, "text", title_column_id);
    title_column.set_sort_column_id(title_column_id);
    title_column.set_sizing(gtk::TreeViewColumnSizing::Autosize);
    title_column.set_resizable(true);
    description_column.set_title("Description");
    description_column.pack_start(&description_renderer, true);
    description_column.add_attribute(&description_renderer, "text", description_column_id);
    description_column.set_sort_column_id(description_column_id);
    description_column.set_sizing(gtk::TreeViewColumnSizing::Autosize);
    description_column.set_resizable(true);
        
    todo_view.append_column(&title_column);
    todo_view.append_column(&description_column);

    todo_view.set_hexpand(true);
    todo_view.set_vexpand(true);

    grid.attach(&todo_view, 0, 0, 3, 1);

    window.add(&grid);
    window.show_all();
}

fn main() {
    let application =
        gtk::Application::new(Some("cc.southball.gtk.todo_list"), Default::default())
            .expect("Failed initialization.");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&[]);
}
