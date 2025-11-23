use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use log::{info, debug, error};
use std::path::PathBuf;

pub async fn init_database(db_path: PathBuf) -> Result<SqlitePool, sqlx::Error> {
    info!("Initializing database at: {:?}", db_path);
    
    // Ensure parent directory exists
    if let Some(parent) = db_path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| {
            error!("Failed to create database directory: {}", e);
            sqlx::Error::Io(e)
        })?;
        info!("Database directory created/verified");
    }
    
    // Convert path to string and create proper SQLite URL
    let db_path_str = db_path.to_str().ok_or_else(|| {
        error!("Invalid database path");
        sqlx::Error::Configuration("Invalid database path".into())
    })?;
    
    let database_url = format!("sqlite://{}?mode=rwc", db_path_str);
    debug!("Database URL: {}", database_url);
    
    // Create connection pool
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Run migrations
    create_tables(&pool).await?;
    
    info!("Database initialized successfully");
    Ok(pool)
}

async fn create_tables(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    info!("Creating database tables...");
    
    // Create reminders table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS reminders (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            description TEXT NOT NULL,
            time TEXT NOT NULL,
            completed INTEGER NOT NULL DEFAULT 0,
            category TEXT NOT NULL,
            frequency TEXT NOT NULL,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;
    
    // Create evidence table for rich media attachments
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS evidence (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            reminder_id INTEGER NOT NULL,
            file_type TEXT NOT NULL,
            file_path TEXT NOT NULL,
            file_name TEXT NOT NULL,
            file_size INTEGER NOT NULL,
            mime_type TEXT NOT NULL,
            thumbnail_path TEXT,
            description TEXT,
            metadata TEXT,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (reminder_id) REFERENCES reminders(id) ON DELETE CASCADE
        )
        "#
    )
    .execute(pool)
    .await?;
    
    // Create index for faster lookups
    sqlx::query(
        r#"
        CREATE INDEX IF NOT EXISTS idx_evidence_reminder_id 
        ON evidence(reminder_id)
        "#
    )
    .execute(pool)
    .await?;
    
    // Create sync_settings table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sync_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            sync_enabled INTEGER NOT NULL DEFAULT 0,
            data_source TEXT NOT NULL DEFAULT 'local',
            github_token TEXT,
            github_repo TEXT,
            sync_method TEXT DEFAULT 'gist',
            last_sync TEXT,
            auto_sync INTEGER NOT NULL DEFAULT 0,
            sync_interval_minutes INTEGER NOT NULL DEFAULT 30,
            created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
            updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
        )
        "#
    )
    .execute(pool)
    .await?;
    
    // Insert default settings if not exists
    sqlx::query(
        r#"
        INSERT OR IGNORE INTO sync_settings (id, sync_enabled, data_source, auto_sync, sync_interval_minutes)
        VALUES (1, 0, 'local', 0, 30)
        "#
    )
    .execute(pool)
    .await?;
    
    info!("Database tables created successfully");
    Ok(())
}
