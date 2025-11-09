use log::{info, debug, warn};
use crate::models::Reminder;
use crate::state::AppState;

#[tauri::command]
pub fn add_reminder(
    title: String,
    description: String,
    time: String,
    category: String,
    frequency: String,
    state: tauri::State<AppState>,
) -> Result<(), String> {
    info!("Adding reminder: title='{}', category='{}', time='{}', frequency='{}'", title, category, time, frequency);
    
    let mut reminders = state.reminders.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();
    
    let reminder = Reminder {
        id: *next_id,
        title: title.clone(),
        description: description.clone(),
        time: time.clone(),
        completed: false,
        category: category.clone(),
        frequency: frequency.clone(),
    };
    
    reminders.push(reminder);
    debug!("Reminder added with id={}, total reminders={}", *next_id, reminders.len());
    *next_id += 1;
    
    Ok(())
}

#[tauri::command]
pub fn get_reminders(state: tauri::State<AppState>) -> Result<Vec<Reminder>, String> {
    debug!("Fetching all reminders");
    let reminders = state.reminders.lock().unwrap();
    info!("Retrieved {} reminders", reminders.len());
    Ok(reminders.clone())
}

#[tauri::command]
pub fn toggle_reminder(id: u32, state: tauri::State<AppState>) -> Result<(), String> {
    debug!("Toggling reminder with id={}", id);
    let mut reminders = state.reminders.lock().unwrap();
    
    if let Some(reminder) = reminders.iter_mut().find(|r| r.id == id) {
        reminder.completed = !reminder.completed;
        info!("Reminder id={} toggled to completed={}", id, reminder.completed);
        Ok(())
    } else {
        warn!("Reminder id={} not found for toggle", id);
        Err("Reminder not found".to_string())
    }
}

#[tauri::command]
pub fn delete_reminder(id: u32, state: tauri::State<AppState>) -> Result<(), String> {
    debug!("Deleting reminder with id={}", id);
    let mut reminders = state.reminders.lock().unwrap();
    
    if let Some(pos) = reminders.iter().position(|r| r.id == id) {
        reminders.remove(pos);
        info!("Reminder id={} deleted successfully, remaining reminders={}", id, reminders.len());
        Ok(())
    } else {
        warn!("Reminder id={} not found for deletion", id);
        Err("Reminder not found".to_string())
    }
}

#[tauri::command]
pub fn set_debug_mode(enabled: bool, state: tauri::State<AppState>) -> Result<(), String> {
    let mut config = state.config.lock().unwrap();
    config.debug_mode = enabled;
    
    // Update log level dynamically
    let log_level = if enabled { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", log_level);
    
    info!("Debug mode set to: {}", enabled);
    Ok(())
}

#[tauri::command]
pub fn get_debug_mode(state: tauri::State<AppState>) -> Result<bool, String> {
    let config = state.config.lock().unwrap();
    Ok(config.debug_mode)
}
