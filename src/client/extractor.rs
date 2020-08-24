use crate::model::ToDo;
use crate::state::AppState;
use crate::ui::todo_treeview::{TITLE_COLUMN_ID, DESCRIPTION_COLUMN_ID};

use gtk::prelude::*;

pub trait ExtractorClient {
    fn get_todos(&self) -> Vec<ToDo>;
}

impl ExtractorClient for AppState {
    fn get_todos(&self) -> Vec<ToDo> {
        let mut todos: Vec<ToDo> = vec![];
        self.store.foreach(|model, _path, iter| {
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
}
