use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncSettings {
    pub id: i64,
    pub sync_enabled: bool,
    pub data_source: String, // "github", "local", etc.
    pub github_token: Option<String>,
    pub github_repo: Option<String>, // Format: "owner/repo"
    pub sync_method: Option<String>, // "gist", "issues", "repo_json"
    pub last_sync: Option<String>, // ISO 8601 timestamp
    pub auto_sync: bool,
    pub sync_interval_minutes: i32, // Auto-sync interval
}

impl Default for SyncSettings {
    fn default() -> Self {
        SyncSettings {
            id: 1,
            sync_enabled: false,
            data_source: "local".to_string(),
            github_token: None,
            github_repo: None,
            sync_method: Some("gist".to_string()),
            last_sync: None,
            auto_sync: false,
            sync_interval_minutes: 30,
        }
    }
}
