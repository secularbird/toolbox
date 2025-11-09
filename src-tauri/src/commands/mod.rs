use log::{info, debug, warn};
use sqlx::SqlitePool;
use crate::models::Reminder;

#[tauri::command]
pub async fn add_reminder(
    title: String,
    description: String,
    time: String,
    category: String,
    frequency: String,
    pool: tauri::State<'_, SqlitePool>,
) -> Result<(), String> {
    info!("Adding reminder: title='{}', category='{}', time='{}', frequency='{}'", title, category, time, frequency);
    
    crate::database::add_reminder(&pool, &title, &description, &time, &category, &frequency)
        .await
        .map_err(|e| {
            warn!("Failed to add reminder: {}", e);
            e.to_string()
        })?;
    
    Ok(())
}

#[tauri::command]
pub async fn get_reminders(pool: tauri::State<'_, SqlitePool>) -> Result<Vec<Reminder>, String> {
    debug!("Fetching all reminders");
    
    crate::database::get_all_reminders(&pool)
        .await
        .map_err(|e| {
            warn!("Failed to get reminders: {}", e);
            e.to_string()
        })
}

#[tauri::command]
pub async fn toggle_reminder(id: u32, pool: tauri::State<'_, SqlitePool>) -> Result<(), String> {
    debug!("Toggling reminder with id={}", id);
    
    crate::database::toggle_reminder(&pool, id)
        .await
        .map_err(|e| {
            warn!("Failed to toggle reminder: {}", e);
            e.to_string()
        })
}

#[tauri::command]
pub async fn delete_reminder(id: u32, pool: tauri::State<'_, SqlitePool>) -> Result<(), String> {
    debug!("Deleting reminder with id={}", id);
    
    crate::database::delete_reminder(&pool, id)
        .await
        .map_err(|e| {
            warn!("Failed to delete reminder: {}", e);
            e.to_string()
        })
}

#[tauri::command]
pub fn set_debug_mode(enabled: bool) -> Result<(), String> {
    // Update log level dynamically
    let log_level = if enabled { "debug" } else { "info" };
    std::env::set_var("RUST_LOG", log_level);
    
    info!("Debug mode set to: {}", enabled);
    Ok(())
}

#[tauri::command]
pub fn get_debug_mode() -> Result<bool, String> {
    let log_level = std::env::var("RUST_LOG").unwrap_or_else(|_| "debug".to_string());
    Ok(log_level == "debug")
}
