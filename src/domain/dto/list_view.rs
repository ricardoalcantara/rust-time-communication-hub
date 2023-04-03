use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListView<T> {
    pub list: Vec<T>,
    pub page: u64,
}

impl<T> ListView<T> {
    pub fn new(list: Vec<T>, page: u64) -> Self {
        Self { list, page }
    }
}
