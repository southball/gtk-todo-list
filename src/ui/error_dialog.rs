use crate::state::AppState;
use gtk::prelude::*;

pub trait ErrorDialogUI {
    fn show_error_dialog(&self, title: Option<&str>, description: Option<&str>);
}

impl ErrorDialogUI for AppState {
    fn show_error_dialog(&self, title: Option<&str>, description: Option<&str>) {
        let dialog = gtk::MessageDialog::new(
            Some(self.main_window.as_ref()),
            gtk::DialogFlags::MODAL | gtk::DialogFlags::USE_HEADER_BAR,
            gtk::MessageType::Error,
            gtk::ButtonsType::Ok,
            description.unwrap_or(""),
        );

        dialog.get_message_area().and_then(|message_area| {
            message_area.set_valign(gtk::Align::Center);
            None as Option<()>
        });

        dialog.connect_response(|dialog, response| {
            dialog.hide();
        });

        dialog.set_title(title.unwrap_or("Error"));
        dialog.run();
    }
}
