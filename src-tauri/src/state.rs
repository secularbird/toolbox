use std::sync::Mutex;
use crate::models::Reminder;
use crate::config::AppConfig;

pub struct AppState {
    pub reminders: Mutex<Vec<Reminder>>,
    pub next_id: Mutex<u32>,
    pub config: Mutex<AppConfig>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            reminders: Mutex::new(Vec::new()),
            next_id: Mutex::new(1),
            config: Mutex::new(AppConfig::default()),
        }
    }
}
