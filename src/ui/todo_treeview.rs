use crate::state::AppState;
use gtk::prelude::*;

pub const TITLE_COLUMN_ID: i32 = 0;
pub const DESCRIPTION_COLUMN_ID: i32 = 1;

pub trait ToDoTreeViewUI {
    fn populate_treeview_todo(&self, treeview: &gtk::TreeView);
}

impl ToDoTreeViewUI for AppState {
    fn populate_treeview_todo(&self, treeview: &gtk::TreeView) {
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

        treeview.append_column(&title_column);
        treeview.append_column(&description_column);

        treeview
            .get_selection()
            .set_mode(gtk::SelectionMode::Multiple);
        treeview.set_model(Some(self.store.as_ref()));
    }
}
