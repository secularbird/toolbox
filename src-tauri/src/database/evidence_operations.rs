use sqlx::SqlitePool;
use log::{info, error};
use crate::models::{Evidence, EvidenceInput};

pub async fn add_evidence(pool: &SqlitePool, input: EvidenceInput) -> Result<Evidence, String> {
    info!("Adding evidence for reminder_id: {}", input.reminder_id);
    
    let result = sqlx::query(
        r#"
        INSERT INTO evidence (
            reminder_id, file_type, file_path, file_name, 
            file_size, mime_type, thumbnail_path, description, metadata
        )
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#
    )
    .bind(input.reminder_id)
    .bind(&input.file_type)
    .bind(&input.file_path)
    .bind(&input.file_name)
    .bind(input.file_size)
    .bind(&input.mime_type)
    .bind(&input.thumbnail_path)
    .bind(&input.description)
    .bind(&input.metadata)
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Failed to add evidence: {}", e);
        format!("Database error: {}", e)
    })?;
    
    let id = result.last_insert_rowid();
    
    // Fetch the created evidence
    get_evidence_by_id(pool, id).await
}

pub async fn get_evidence_by_id(pool: &SqlitePool, id: i64) -> Result<Evidence, String> {
    sqlx::query_as::<_, Evidence>(
        r#"
        SELECT id, reminder_id, file_type, file_path, file_name,
               file_size, mime_type, thumbnail_path, description,
               metadata, created_at
        FROM evidence
        WHERE id = ?
        "#
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|e| format!("Evidence not found: {}", e))
}

pub async fn get_evidence_by_reminder(pool: &SqlitePool, reminder_id: i64) -> Result<Vec<Evidence>, String> {
    sqlx::query_as::<_, Evidence>(
        r#"
        SELECT id, reminder_id, file_type, file_path, file_name,
               file_size, mime_type, thumbnail_path, description,
               metadata, created_at
        FROM evidence
        WHERE reminder_id = ?
        ORDER BY created_at DESC
        "#
    )
    .bind(reminder_id)
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("Failed to get evidence: {}", e);
        format!("Database error: {}", e)
    })
}

pub async fn get_all_evidence(pool: &SqlitePool) -> Result<Vec<Evidence>, String> {
    sqlx::query_as::<_, Evidence>(
        r#"
        SELECT id, reminder_id, file_type, file_path, file_name,
               file_size, mime_type, thumbnail_path, description,
               metadata, created_at
        FROM evidence
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| {
        error!("Failed to get all evidence: {}", e);
        format!("Database error: {}", e)
    })
}

pub async fn update_evidence_description(
    pool: &SqlitePool,
    id: i64,
    description: Option<String>
) -> Result<(), String> {
    sqlx::query(
        r#"
        UPDATE evidence
        SET description = ?
        WHERE id = ?
        "#
    )
    .bind(&description)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| {
        error!("Failed to update evidence description: {}", e);
        format!("Database error: {}", e)
    })?;
    
    Ok(())
}

pub async fn delete_evidence(pool: &SqlitePool, id: i64) -> Result<(), String> {
    info!("Deleting evidence: {}", id);
    
    // First get the evidence to delete the file
    let evidence = get_evidence_by_id(pool, id).await?;
    
    // Delete from database
    sqlx::query("DELETE FROM evidence WHERE id = ?")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| {
            error!("Failed to delete evidence: {}", e);
            format!("Database error: {}", e)
        })?;
    
    // Try to delete the physical file (if it's a local file)
    if !evidence.file_path.starts_with("http") {
        let _ = std::fs::remove_file(&evidence.file_path);
        if let Some(thumb) = &evidence.thumbnail_path {
            let _ = std::fs::remove_file(thumb);
        }
    }
    
    info!("Evidence deleted successfully");
    Ok(())
}

pub async fn delete_evidence_by_reminder(pool: &SqlitePool, reminder_id: i64) -> Result<(), String> {
    info!("Deleting all evidence for reminder: {}", reminder_id);
    
    // Get all evidence first to delete files
    let evidence_list = get_evidence_by_reminder(pool, reminder_id).await?;
    
    // Delete from database
    sqlx::query("DELETE FROM evidence WHERE reminder_id = ?")
        .bind(reminder_id)
        .execute(pool)
        .await
        .map_err(|e| {
            error!("Failed to delete evidence: {}", e);
            format!("Database error: {}", e)
        })?;
    
    // Try to delete physical files
    for evidence in evidence_list {
        if !evidence.file_path.starts_with("http") {
            let _ = std::fs::remove_file(&evidence.file_path);
            if let Some(thumb) = &evidence.thumbnail_path {
                let _ = std::fs::remove_file(thumb);
            }
        }
    }
    
    Ok(())
}

// Implement FromRow for Evidence manually
impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for Evidence {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        
        Ok(Evidence {
            id: row.try_get("id")?,
            reminder_id: row.try_get("reminder_id")?,
            file_type: row.try_get("file_type")?,
            file_path: row.try_get("file_path")?,
            file_name: row.try_get("file_name")?,
            file_size: row.try_get("file_size")?,
            mime_type: row.try_get("mime_type")?,
            thumbnail_path: row.try_get("thumbnail_path")?,
            description: row.try_get("description")?,
            metadata: row.try_get("metadata")?,
            created_at: row.try_get("created_at")?,
        })
    }
}
