use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: u32,
    pub title: String,
    pub description: String,
    pub time: String,
    pub completed: bool,
    pub category: String,
    pub frequency: String,
}
