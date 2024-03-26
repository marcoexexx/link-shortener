use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct Link {
    pub original: String,
    pub shorten: String,
}

#[derive(Debug, FromForm)]
pub struct LinkInput {
    pub original: String,
}

pub struct AppState {
    pub links: Arc<Mutex<Vec<Link>>>,
}
impl AppState {
    pub fn new() -> AppState {
        AppState {
            links: Arc::new(Mutex::new(Vec::new())),
        }
    }
}
