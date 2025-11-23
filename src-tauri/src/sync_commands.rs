use log::{info, debug, warn};
use sqlx::SqlitePool;
use tauri::{AppHandle, Emitter};
use crate::models::SyncSettings;
use crate::database;
use crate::sync;

#[tauri::command]
pub async fn get_sync_settings(pool: tauri::State<'_, SqlitePool>) -> Result<SyncSettings, String> {
    debug!("get_sync_settings command called");
    
    database::get_sync_settings(&pool)
        .await
        .map_err(|e| {
            warn!("Failed to get sync settings: {}", e);
            e.to_string()
        })
}

#[tauri::command]
pub async fn save_sync_settings(
    settings: SyncSettings,
    pool: tauri::State<'_, SqlitePool>,
    app: AppHandle,
) -> Result<(), String> {
    info!("Saving sync settings");
    
    database::update_sync_settings(&pool, &settings)
        .await
        .map_err(|e| {
            warn!("Failed to save sync settings: {}", e);
            e.to_string()
        })?;
    
    // Emit event to notify UI of settings update
    let _ = app.emit("sync-settings-updated", &settings);
    info!("Sync settings saved and broadcasted");
    
    Ok(())
}

#[tauri::command]
pub async fn test_github_connection(token: String) -> Result<bool, String> {
    info!("Testing GitHub connection");
    sync::test_github_connection(&token).await
}

#[tauri::command]
pub async fn sync_to_github(pool: tauri::State<'_, SqlitePool>) -> Result<String, String> {
    info!("sync_to_github command called");
    
    // Get sync settings
    let settings = database::get_sync_settings(&pool)
        .await
        .map_err(|e| e.to_string())?;
    
    if !settings.sync_enabled {
        return Err("Sync is not enabled".to_string());
    }
    
    if settings.data_source != "github" {
        return Err("Data source is not set to GitHub".to_string());
    }
    
    // Get all reminders
    let reminders = database::get_all_reminders(&pool)
        .await
        .map_err(|e| e.to_string())?;
    
    // Sync to GitHub
    let result = sync::sync_reminders_to_github(&reminders, &settings).await?;
    
    // Update last sync time
    let now = chrono::Utc::now().to_rfc3339();
    database::update_last_sync_time(&pool, &now)
        .await
        .map_err(|e| e.to_string())?;
    
    info!("Successfully synced {} reminders to GitHub", reminders.len());
    Ok(result)
}

#[tauri::command]
pub async fn sync_from_github(
    pool: tauri::State<'_, SqlitePool>,
    app: AppHandle,
) -> Result<usize, String> {
    info!("sync_from_github command called");
    
    // Get sync settings
    let settings = database::get_sync_settings(&pool)
        .await
        .map_err(|e| e.to_string())?;
    
    if !settings.sync_enabled {
        return Err("Sync is not enabled".to_string());
    }
    
    if settings.data_source != "github" {
        return Err("Data source is not set to GitHub".to_string());
    }
    
    // Fetch reminders from GitHub
    let github_reminders = sync::fetch_reminders_from_github(&settings).await?;
    
    // NOTE: This currently only fetches and validates the data from GitHub.
    // It does NOT replace local reminders to prevent accidental data loss.
    // A future enhancement will add proper merge/conflict resolution logic.
    // For now, this serves as a validation that the GitHub sync is working.
    info!("Fetched and validated {} reminders from GitHub", github_reminders.len());
    
    // Update last sync time
    let now = chrono::Utc::now().to_rfc3339();
    database::update_last_sync_time(&pool, &now)
        .await
        .map_err(|e| e.to_string())?;
    
    Ok(github_reminders.len())
}
