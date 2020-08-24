use std::clone::Clone;
use std::path::PathBuf;
use std::sync::{atomic::AtomicBool, Arc, RwLock};

#[derive(Clone)]
pub struct AppState {
    pub main_window: Arc<gtk::Window>,
    pub header_bar: Arc<gtk::HeaderBar>,
    pub dirty: Arc<AtomicBool>,
    pub file: Arc<RwLock<Option<PathBuf>>>,
    pub store: Arc<gtk::ListStore>,
}
