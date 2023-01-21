use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Entry {
    pub id: i32,
    pub date: i64,
    pub entry: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CreateEntry {
    pub date: i64,
    pub entry: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct UpdateEntry {
    pub entry: String,
}