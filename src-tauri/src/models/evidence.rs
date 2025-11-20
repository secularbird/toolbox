use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: i64,
    pub reminder_id: i64,
    pub file_type: String,  // image, video, audio, document, link
    pub file_path: String,  // Local path or URL
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub thumbnail_path: Option<String>,
    pub description: Option<String>,
    pub created_at: String,
    pub metadata: Option<String>, // JSON string for additional metadata
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceInput {
    pub reminder_id: i64,
    pub file_type: String,
    pub file_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub mime_type: String,
    pub thumbnail_path: Option<String>,
    pub description: Option<String>,
    pub metadata: Option<String>,
}
