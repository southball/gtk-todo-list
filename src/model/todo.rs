use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToDo {
    pub title: Option<String>,
    pub description: Option<String>,
}
