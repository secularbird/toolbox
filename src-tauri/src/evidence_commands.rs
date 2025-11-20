use tauri::{command, AppHandle, Manager};
use sqlx::SqlitePool;
use std::path::PathBuf;
use log::info;
use crate::models::{Evidence, EvidenceInput};
use crate::database;

#[command]
pub async fn add_evidence_to_reminder(
    app: AppHandle,
    reminder_id: i64,
    file_type: String,
    file_path: String,
    file_name: String,
    file_size: i64,
    mime_type: String,
    thumbnail_path: Option<String>,
    description: Option<String>,
    metadata: Option<String>,
) -> Result<Evidence, String> {
    info!("Adding evidence to reminder {}", reminder_id);
    
    let pool = app.state::<SqlitePool>();
    
    let input = EvidenceInput {
        reminder_id,
        file_type,
        file_path,
        file_name,
        file_size,
        mime_type,
        thumbnail_path,
        description,
        metadata,
    };
    
    database::add_evidence(&pool, input).await
}

#[command]
pub async fn get_reminder_evidence(
    app: AppHandle,
    reminder_id: i64,
) -> Result<Vec<Evidence>, String> {
    let pool = app.state::<SqlitePool>();
    database::get_evidence_by_reminder(&pool, reminder_id).await
}

#[command]
pub async fn get_all_evidence_items(app: AppHandle) -> Result<Vec<Evidence>, String> {
    let pool = app.state::<SqlitePool>();
    database::get_all_evidence(&pool).await
}

#[command]
pub async fn update_evidence_desc(
    app: AppHandle,
    evidence_id: i64,
    description: Option<String>,
) -> Result<(), String> {
    let pool = app.state::<SqlitePool>();
    database::update_evidence_description(&pool, evidence_id, description).await
}

#[command]
pub async fn delete_evidence_item(
    app: AppHandle,
    evidence_id: i64,
) -> Result<(), String> {
    let pool = app.state::<SqlitePool>();
    database::delete_evidence(&pool, evidence_id).await
}

#[command]
pub async fn save_uploaded_file(
    app: AppHandle,
    file_name: String,
    file_data: Vec<u8>,
) -> Result<String, String> {
    info!("Saving uploaded file: {}", file_name);
    
    // Get app data directory
    let app_dir = app.path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    
    // Create evidence directory
    let evidence_dir = app_dir.join("evidence");
    std::fs::create_dir_all(&evidence_dir)
        .map_err(|e| format!("Failed to create evidence dir: {}", e))?;
    
    // Generate unique filename with timestamp
    let timestamp = chrono::Utc::now().timestamp();
    let extension = std::path::Path::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("bin");
    let unique_name = format!("{}_{}.{}", timestamp, uuid::Uuid::new_v4(), extension);
    
    let file_path = evidence_dir.join(&unique_name);
    
    // Write file
    std::fs::write(&file_path, file_data)
        .map_err(|e| format!("Failed to write file: {}", e))?;
    
    info!("File saved successfully: {:?}", file_path);
    
    Ok(file_path.to_string_lossy().to_string())
}

#[command]
pub async fn get_evidence_file_path(
    app: AppHandle,
    evidence_id: i64,
) -> Result<String, String> {
    let pool = app.state::<SqlitePool>();
    let evidence = database::get_evidence_by_id(&pool, evidence_id).await?;
    Ok(evidence.file_path)
}

#[command]
pub async fn open_evidence_file(
    app: AppHandle,
    file_path: String,
) -> Result<(), String> {
    use tauri_plugin_opener::OpenerExt;
    
    app.opener()
        .open_path(&file_path, None::<&str>)
        .map_err(|e| format!("Failed to open file: {}", e))?;
    
    Ok(())
}

#[command]
pub fn get_mime_type(file_path: String) -> String {
    let path = PathBuf::from(&file_path);
    
    match path.extension().and_then(|e| e.to_str()) {
        Some("jpg") | Some("jpeg") => "image/jpeg".to_string(),
        Some("png") => "image/png".to_string(),
        Some("gif") => "image/gif".to_string(),
        Some("webp") => "image/webp".to_string(),
        Some("svg") => "image/svg+xml".to_string(),
        Some("mp4") => "video/mp4".to_string(),
        Some("webm") => "video/webm".to_string(),
        Some("mov") => "video/quicktime".to_string(),
        Some("mp3") => "audio/mpeg".to_string(),
        Some("wav") => "audio/wav".to_string(),
        Some("ogg") => "audio/ogg".to_string(),
        Some("pdf") => "application/pdf".to_string(),
        Some("doc") | Some("docx") => "application/msword".to_string(),
        Some("xls") | Some("xlsx") => "application/vnd.ms-excel".to_string(),
        Some("txt") => "text/plain".to_string(),
        Some("json") => "application/json".to_string(),
        Some("zip") => "application/zip".to_string(),
        _ => "application/octet-stream".to_string(),
    }
}

#[command]
pub fn format_file_size(bytes: i64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let bytes_f = bytes as f64;
    let base = 1024f64;
    let exp = (bytes_f.log(base).floor() as usize).min(UNITS.len() - 1);
    
    let size = bytes_f / base.powi(exp as i32);
    
    if exp == 0 {
        format!("{} {}", bytes, UNITS[exp])
    } else {
        format!("{:.2} {}", size, UNITS[exp])
    }
}
