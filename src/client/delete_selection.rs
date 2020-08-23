use crate::state::AppState;
use gtk::prelude::*;

pub trait DeleteSelectionClient {
    fn handle_delete_selection_button(&self, button: &gtk::Button, treeview: &gtk::TreeView);
}

impl DeleteSelectionClient for AppState {
    fn handle_delete_selection_button(&self, button: &gtk::Button, treeview: &gtk::TreeView) {
        let store = self.store.clone();
        let treeview = treeview.clone();

        button.connect_clicked(move |_button| {
            let (paths, model) = treeview.get_selection().get_selected_rows();
            for path in paths.iter().rev() {
                store.remove(&model.get_iter(path).unwrap());
            }
        });
    }
}
