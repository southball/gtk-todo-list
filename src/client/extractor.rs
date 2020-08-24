use crate::model::ToDo;
use crate::state::AppState;
use crate::ui::todo_treeview::{DESCRIPTION_COLUMN_ID, TITLE_COLUMN_ID};

use gtk::prelude::*;

pub trait ExtractorClient {
    fn get_todos(&self) -> Vec<ToDo>;
    fn set_todos(&self, todos: &[ToDo]);
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

    fn set_todos(&self, todos: &[ToDo]) {
        self.store.clear();
        todos.into_iter().for_each(|todo| {
            self.store.insert_with_values(
                None,
                &[TITLE_COLUMN_ID as u32, DESCRIPTION_COLUMN_ID as u32],
                &[&todo.title, &todo.description],
            );
        });
    }
}
