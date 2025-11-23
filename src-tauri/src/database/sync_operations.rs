use sqlx::SqlitePool;
use log::{info, debug, warn};
use crate::models::SyncSettings;

pub async fn get_sync_settings(pool: &SqlitePool) -> Result<SyncSettings, sqlx::Error> {
    debug!("Fetching sync settings from database");
    
    let row = sqlx::query_as::<_, (i64, i64, String, Option<String>, Option<String>, Option<String>, Option<String>, i64, i64)>(
        r#"
        SELECT id, sync_enabled, data_source, github_token, github_repo, sync_method, last_sync, auto_sync, sync_interval_minutes
        FROM sync_settings
        WHERE id = 1
        "#
    )
    .fetch_one(pool)
    .await?;
    
    let settings = SyncSettings {
        id: row.0,
        sync_enabled: row.1 != 0,
        data_source: row.2,
        github_token: row.3,
        github_repo: row.4,
        sync_method: row.5,
        last_sync: row.6,
        auto_sync: row.7 != 0,
        sync_interval_minutes: row.8 as i32,
    };
    
    info!("Retrieved sync settings");
    Ok(settings)
}

pub async fn update_sync_settings(pool: &SqlitePool, settings: &SyncSettings) -> Result<(), sqlx::Error> {
    debug!("Updating sync settings in database");
    
    sqlx::query(
        r#"
        UPDATE sync_settings
        SET sync_enabled = ?,
            data_source = ?,
            github_token = ?,
            github_repo = ?,
            sync_method = ?,
            last_sync = ?,
            auto_sync = ?,
            sync_interval_minutes = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = 1
        "#
    )
    .bind(if settings.sync_enabled { 1 } else { 0 })
    .bind(&settings.data_source)
    .bind(&settings.github_token)
    .bind(&settings.github_repo)
    .bind(&settings.sync_method)
    .bind(&settings.last_sync)
    .bind(if settings.auto_sync { 1 } else { 0 })
    .bind(settings.sync_interval_minutes)
    .execute(pool)
    .await?;
    
    info!("Sync settings updated successfully");
    Ok(())
}

pub async fn update_last_sync_time(pool: &SqlitePool, timestamp: &str) -> Result<(), sqlx::Error> {
    debug!("Updating last sync time to: {}", timestamp);
    
    sqlx::query(
        r#"
        UPDATE sync_settings
        SET last_sync = ?,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = 1
        "#
    )
    .bind(timestamp)
    .execute(pool)
    .await?;
    
    info!("Last sync time updated");
    Ok(())
}
