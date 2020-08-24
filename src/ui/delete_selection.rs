use crate::state::AppState;
use crate::ui::MainWindowUI;
use gtk::prelude::*;
use std::sync::atomic::Ordering::Relaxed;

pub trait DeleteSelectionUI {
    fn handle_delete_selection_button(&self, button: &gtk::Button, treeview: &gtk::TreeView);
}

impl DeleteSelectionUI for AppState {
    fn handle_delete_selection_button(&self, button: &gtk::Button, treeview: &gtk::TreeView) {
        let treeview = treeview.clone();
        let app_state = self.clone();

        button.connect_clicked(move |_button| {
            let (paths, model) = treeview.get_selection().get_selected_rows();
            for path in paths.iter().rev() {
                app_state.store.remove(&model.get_iter(path).unwrap());
            }
            app_state.dirty.store(true, Relaxed);
            app_state.update_subtitle();
        });
    }
}
