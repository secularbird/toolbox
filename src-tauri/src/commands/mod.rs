use log::{info, debug, warn};
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};
use crate::models::Reminder;

#[tauri::command]
pub async fn add_reminder(
    title: String,
    description: String,
    time: String,
    category: String,
    frequency: String,
    pool: tauri::State<'_, SqlitePool>,
    app: AppHandle,
) -> Result<(), String> {
    info!("Adding reminder: title='{}', category='{}', time='{}', frequency='{}'", title, category, time, frequency);
    
    crate::database::add_reminder(&pool, &title, &description, &time, &category, &frequency)
        .await
        .map_err(|e| {
            warn!("Failed to add reminder: {}", e);
            e.to_string()
        })?;
    
    // Broadcast update event to all windows
    let reminders = crate::database::get_all_reminders(&pool)
        .await
        .map_err(|e| e.to_string())?;
    let _ = app.emit("reminders-updated", &reminders);
    info!("Broadcasted reminders-updated event to all windows");
    
    Ok(())
}

#[tauri::command]
pub async fn get_reminders(pool: tauri::State<'_, SqlitePool>) -> Result<Vec<Reminder>, String> {
    debug!("get_reminders command called");
    
    let reminders = crate::database::get_all_reminders(&pool)
        .await
        .map_err(|e| {
            warn!("Failed to get reminders: {}", e);
            e.to_string()
        })?;
    
    info!("Returning {} reminders to frontend", reminders.len());
    Ok(reminders)
}

#[tauri::command]
pub async fn get_due_reminders(pool: tauri::State<'_, SqlitePool>) -> Result<Vec<Reminder>, String> {
    debug!("Fetching due reminders");
    
    let rows = sqlx::query_as::<_, (i64, String, String, String, i64, String, String)>(
        r#"
        SELECT id, title, description, time, completed, category, frequency
        FROM reminders
        WHERE completed = 0 AND datetime(time) <= datetime('now')
        ORDER BY time ASC
        "#
    )
    .fetch_all(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    let reminders: Vec<Reminder> = rows
        .into_iter()
        .map(|(id, title, description, time, completed, category, frequency)| Reminder {
            id: id as u32,
            title,
            description,
            time,
            completed: completed != 0,
            category,
            frequency,
        })
        .collect();
    
    info!("Retrieved {} due reminders", reminders.len());
    Ok(reminders)
}

#[tauri::command]
pub async fn toggle_reminder(
    id: u32,
    pool: tauri::State<'_, SqlitePool>,
    app: AppHandle,
) -> Result<(), String> {
    debug!("Toggling reminder with id={}", id);
    
    crate::database::toggle_reminder(&pool, id)
        .await
        .map_err(|e| {
            warn!("Failed to toggle reminder: {}", e);
            e.to_string()
        })?;
    
    // Broadcast update event to all windows
    let reminders = crate::database::get_all_reminders(&pool)
        .await
        .map_err(|e| e.to_string())?;
    let _ = app.emit("reminders-updated", &reminders);
    info!("Broadcasted reminders-updated event after toggle");
    
    Ok(())
}

#[tauri::command]
pub async fn delete_reminder(
    id: u32,
    pool: tauri::State<'_, SqlitePool>,
    app: AppHandle,
) -> Result<(), String> {
    debug!("Deleting reminder with id={}", id);
    
    crate::database::delete_reminder(&pool, id)
        .await
        .map_err(|e| {
            warn!("Failed to delete reminder: {}", e);
            e.to_string()
        })?;
    
    // Broadcast update event to all windows
    let reminders = crate::database::get_all_reminders(&pool)
        .await
        .map_err(|e| e.to_string())?;
    let _ = app.emit("reminders-updated", &reminders);
    info!("Broadcasted reminders-updated event after delete");
    
    Ok(())
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

// Broadcast current reminders to all windows
#[tauri::command]
pub async fn broadcast_reminders(
    pool: tauri::State<'_, SqlitePool>,
    app: AppHandle,
) -> Result<(), String> {
    debug!("Broadcasting current reminders to all windows");
    
    let reminders = crate::database::get_all_reminders(&pool)
        .await
        .map_err(|e| e.to_string())?;
    
    let _ = app.emit("reminders-updated", &reminders);
    info!("Broadcasted {} reminders to all windows", reminders.len());
    
    Ok(())
}

#[tauri::command]
pub async fn update_reminder(
    id: u32,
    title: String,
    description: String,
    time: String,
    category: String,
    frequency: String,
    pool: tauri::State<'_, SqlitePool>,
    app: AppHandle,
) -> Result<(), String> {
    info!("Updating reminder id={}: title='{}', category='{}', time='{}', frequency='{}'", 
          id, title, category, time, frequency);
    
    sqlx::query(
        "UPDATE reminders SET title = ?, description = ?, time = ?, category = ?, frequency = ? WHERE id = ?"
    )
    .bind(&title)
    .bind(&description)
    .bind(&time)
    .bind(&category)
    .bind(&frequency)
    .bind(id as i64)
    .execute(pool.inner())
    .await
    .map_err(|e| {
        warn!("Failed to update reminder: {}", e);
        e.to_string()
    })?;
    
    // Broadcast update event to all windows
    let reminders = crate::database::get_all_reminders(&pool)
        .await
        .map_err(|e| e.to_string())?;
    let _ = app.emit("reminders-updated", &reminders);
    info!("Broadcasted reminders-updated event after update");
    
    Ok(())
}
