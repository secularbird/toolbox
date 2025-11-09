use sqlx::SqlitePool;
use log::{info, debug, warn};
use crate::models::Reminder;

pub async fn add_reminder(
    pool: &SqlitePool,
    title: &str,
    description: &str,
    time: &str,
    category: &str,
    frequency: &str,
) -> Result<i64, sqlx::Error> {
    debug!("Adding reminder to database: {}", title);
    
    let result = sqlx::query(
        r#"
        INSERT INTO reminders (title, description, time, completed, category, frequency)
        VALUES (?, ?, ?, 0, ?, ?)
        "#
    )
    .bind(title)
    .bind(description)
    .bind(time)
    .bind(category)
    .bind(frequency)
    .execute(pool)
    .await?;
    
    let id = result.last_insert_rowid();
    info!("Reminder added with id={}", id);
    
    Ok(id)
}

pub async fn get_all_reminders(pool: &SqlitePool) -> Result<Vec<Reminder>, sqlx::Error> {
    debug!("Fetching all reminders from database");
    
    let rows = sqlx::query_as::<_, (i64, String, String, String, i64, String, String)>(
        r#"
        SELECT id, title, description, time, completed, category, frequency
        FROM reminders
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await?;
    
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
    
    info!("Retrieved {} reminders", reminders.len());
    Ok(reminders)
}

pub async fn toggle_reminder(pool: &SqlitePool, id: u32) -> Result<(), sqlx::Error> {
    debug!("Toggling reminder id={}", id);
    
    sqlx::query(
        r#"
        UPDATE reminders
        SET completed = NOT completed,
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        "#
    )
    .bind(id as i64)
    .execute(pool)
    .await?;
    
    info!("Reminder id={} toggled", id);
    Ok(())
}

pub async fn delete_reminder(pool: &SqlitePool, id: u32) -> Result<(), sqlx::Error> {
    debug!("Deleting reminder id={}", id);
    
    let result = sqlx::query("DELETE FROM reminders WHERE id = ?")
        .bind(id as i64)
        .execute(pool)
        .await?;
    
    if result.rows_affected() > 0 {
        info!("Reminder id={} deleted successfully", id);
        Ok(())
    } else {
        warn!("Reminder id={} not found", id);
        Err(sqlx::Error::RowNotFound)
    }
}
