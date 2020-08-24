extern crate gtk;
extern crate serde;
extern crate serde_json;

mod client;
mod model;
mod state;
mod ui;

use state::*;
use ui::*;

use gio::prelude::*;
use gtk::prelude::*;
use std::sync::atomic::AtomicBool;
use std::sync::{Arc, RwLock};

const SRC_MAIN_WINDOW: &'static str = include_str!("./ui/main_window.glade");

fn main() {
    gtk::init().expect("Failed to initialize GTK.");

    let builder = gtk::Builder::from_string(SRC_MAIN_WINDOW);
    let main_window: gtk::ApplicationWindow = builder.get_object("window-main").unwrap();
    let header_bar: gtk::HeaderBar = builder.get_object("header-bar-main").unwrap();

    // Application Store
    let store_types = [String::static_type(), String::static_type()];
    let store = gtk::ListStore::new(&store_types);

    let app_state = AppState {
        main_window: Arc::new(main_window.clone().upcast()),
        header_bar: Arc::new(header_bar.clone()),
        dirty: Arc::new(AtomicBool::new(false)),
        file: Arc::new(RwLock::new(None)),
        store: Arc::new(store.clone()),
    };

    app_state.handle_main_window(&builder);

    gtk::main();
}
