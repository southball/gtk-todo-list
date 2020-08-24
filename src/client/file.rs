use super::ExtractorClient;
use crate::state::AppState;

use gio::prelude::*;
use gtk::prelude::*;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::{Arc, Mutex};

type Callback = Arc<Mutex<Box<dyn Fn(bool) -> ()>>>;

pub trait FileClient {
    fn new(&self, callback: Option<Callback>);
    fn open(&self, callback: Option<Callback>);
    fn save(&self, callback: Option<Callback>);
    fn save_as(&self, callback: Option<Callback>);
    fn save_changed(&self, callback: Option<Callback>);
    fn save_and_quit(&self);
    fn save_new_file(&self, callback: Option<Callback>);
    fn save_previous_file(&self, callback: Option<Callback>);
}

impl FileClient for AppState {
    fn new(&self, callback: Option<Callback>) {
        let app_state = self.clone();
        self.save_changed(Some(Arc::new(Mutex::new(Box::new(move |_saved| {
            *app_state.file.write().unwrap() = None;
            app_state.dirty.store(false, Relaxed);
            app_state.store.clear();
            callback.as_ref().and_then(|callback| {
                let callback = callback.lock().unwrap();
                callback(true);
                None as Option<()>
            });
        })))));
    }

    fn open(&self, callback: Option<Callback>) {
        unimplemented!("Open Function");
    }

    fn save(&self, callback: Option<Callback>) {
        if self.file.read().unwrap().is_some() {
            self.save_previous_file(callback);
        } else {
            self.save_new_file(callback);
        }
    }

    fn save_as(&self, callback: Option<Callback>) {
        self.save_new_file(callback);
    }

    fn save_changed(&self, callback: Option<Callback>) {
        let dirty = self.dirty.load(Relaxed);
        let window = self.main_window.as_ref().clone();

        if !dirty {
            callback.as_ref().and_then(|callback| {
                let callback = callback.lock().unwrap();
                callback(false);
                None as Option<()>
            });
        } else {
            let dialog = gtk::MessageDialog::new::<gtk::Window>(
                Some(&window.upcast()),
                gtk::DialogFlags::MODAL | gtk::DialogFlags::USE_HEADER_BAR,
                gtk::MessageType::Question,
                gtk::ButtonsType::YesNo,
                "You have unsaved changes. Do you want to save?",
            );

            dialog.set_title("Unsaved Changes");

            dialog.get_message_area().and_then(|message_area| {
                message_area.set_valign(gtk::Align::Center);
                None as Option<()>
            });

            dialog.connect_response({
                let app = self.clone();

                move |dialog, response| {
                    match response {
                        gtk::ResponseType::Yes => {
                            dialog.hide();
                            app.save(callback.clone());
                        },
                        gtk::ResponseType::No => {
                            dialog.hide();
                            callback.as_ref().and_then(|callback| {
                                let callback = callback.lock().unwrap();
                                callback(false);
                                None as Option<()>
                            });
                        },
                        _ => {
                            dialog.hide();
                        }
                    }
                }
            });

            dialog.run();
        }
    }

    fn save_and_quit(&self) {
        let quit_callback = Arc::new(Mutex::new(Box::new(|_success| {
            gtk::main_quit();
        }) as Box<dyn Fn(bool) -> ()>));

        self.save_changed(Some(quit_callback));
    }

    fn save_new_file(&self, callback: Option<Callback>) {
        let dialog = gtk::FileChooserDialog::new(
            Some("Choose Save Location"),
            Some(self.main_window.as_ref()),
            gtk::FileChooserAction::Save,
        );
        let content = serde_json::to_string(&self.get_todos()).unwrap();

        dialog.set_filename("untitled.json");

        dialog.add_button("Cancel", gtk::ResponseType::Cancel);
        dialog.add_button("Save", gtk::ResponseType::Ok);

        dialog.connect_response({
            let callback = callback.map(|callback: Arc<_>| callback.clone());
            let content = content.to_owned();
            let parent = self.main_window.clone();
            let app = self.clone();

            move |dialog, event| match event {
                gtk::ResponseType::Ok => {
                    let file = dialog.get_file().unwrap();

                    if let Some(path) = file.get_path() {
                        if let Ok(_) = std::fs::write(&path, &content) {
                            dialog.hide();
                            callback.as_ref().and_then(|callback| {
                                let callback = callback.lock().unwrap();
                                callback(true);
                                app.dirty.store(false, Relaxed);
                                None as Option<()>
                            });
                            *app.file.write().unwrap() = Some(path.clone());
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
                                None as Option<()>
                            });
                            dialog.connect_response(|dialog, response| {
                                if response == gtk::ResponseType::Ok {
                                    dialog.hide();
                                }
                            });
                            dialog.run();
                        }
                    }
                }
                _ => {
                    dialog.hide();
                    callback.as_ref().and_then(|callback| {
                        let callback = callback.lock().unwrap();
                        callback(false);
                        None as Option<()>
                    });
                }
            }
        });

        dialog.run();
    }

    fn save_previous_file(&self, callback: Option<Callback>) {
        let file = self.file.read().unwrap().clone().unwrap();
        let content = serde_json::to_string(&self.get_todos()).unwrap();

        if let Ok(_) = std::fs::write(&file, content) {
            let app = self.clone();

            callback.and_then(|callback| {
                let callback = callback.lock().unwrap();
                callback(true);
                app.dirty.store(false, Relaxed);
                None as Option<()>
            });
        } else {
            let dialog = gtk::MessageDialog::new(
                Some(self.main_window.as_ref()),
                gtk::DialogFlags::MODAL | gtk::DialogFlags::USE_HEADER_BAR,
                gtk::MessageType::Error,
                gtk::ButtonsType::Ok,
                "Error saving file.",
            );
            dialog.set_title("Error");
            dialog.get_message_area().and_then(|message_area| {
                message_area.set_valign(gtk::Align::Center);
                None as Option<()>
            });
            dialog.connect_response(|dialog, response| {
                if response == gtk::ResponseType::Ok {
                    dialog.hide();
                }
            });
            dialog.run();
        }
    }
}
