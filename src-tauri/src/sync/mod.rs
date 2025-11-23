use log::{info, debug, warn, error};
use serde::{Deserialize, Serialize};
use crate::models::{Reminder, SyncSettings};

#[derive(Debug, Serialize, Deserialize)]
pub struct GistFile {
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistCreate {
    pub description: String,
    pub public: bool,
    pub files: std::collections::HashMap<String, GistFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistResponse {
    pub id: String,
    pub html_url: String,
    pub files: std::collections::HashMap<String, GistFileResponse>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GistFileResponse {
    pub filename: String,
    pub content: Option<String>,
}

pub async fn test_github_connection(token: &str) -> Result<bool, String> {
    debug!("Testing GitHub connection");
    
    let client = reqwest::Client::new();
    let response = client
        .get("https://api.github.com/user")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Toolbox-App")
        .send()
        .await
        .map_err(|e| format!("Failed to connect to GitHub: {}", e))?;
    
    if response.status().is_success() {
        info!("GitHub connection test successful");
        Ok(true)
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        warn!("GitHub connection test failed: {} - {}", status, body);
        Err(format!("GitHub API returned status {}: {}", status, body))
    }
}

pub async fn sync_reminders_to_github(
    reminders: &[Reminder],
    settings: &SyncSettings,
) -> Result<String, String> {
    info!("Syncing {} reminders to GitHub", reminders.len());
    
    let token = settings.github_token.as_ref()
        .ok_or_else(|| "GitHub token not configured".to_string())?;
    
    match settings.sync_method.as_deref() {
        Some("gist") => sync_to_gist(reminders, token).await,
        Some("repo_json") => {
            let repo = settings.github_repo.as_ref()
                .ok_or_else(|| "GitHub repository not configured".to_string())?;
            sync_to_repo_json(reminders, token, repo).await
        }
        _ => Err("Unsupported sync method".to_string()),
    }
}

async fn sync_to_gist(reminders: &[Reminder], token: &str) -> Result<String, String> {
    debug!("Syncing reminders to GitHub Gist");
    
    let json_content = serde_json::to_string_pretty(reminders)
        .map_err(|e| format!("Failed to serialize reminders: {}", e))?;
    
    let mut files = std::collections::HashMap::new();
    files.insert(
        "reminders.json".to_string(),
        GistFile {
            content: json_content,
        },
    );
    
    let gist_data = GistCreate {
        description: "Toolbox Reminders Backup".to_string(),
        public: false,
        files,
    };
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.github.com/gists")
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Toolbox-App")
        .json(&gist_data)
        .send()
        .await
        .map_err(|e| format!("Failed to create gist: {}", e))?;
    
    if response.status().is_success() {
        let gist: GistResponse = response.json().await
            .map_err(|e| format!("Failed to parse gist response: {}", e))?;
        info!("Successfully synced to gist: {}", gist.html_url);
        Ok(gist.html_url)
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        error!("Failed to sync to gist: {} - {}", status, body);
        Err(format!("GitHub API returned status {}: {}", status, body))
    }
}

async fn sync_to_repo_json(
    reminders: &[Reminder],
    token: &str,
    repo: &str,
) -> Result<String, String> {
    debug!("Syncing reminders to GitHub repository: {}", repo);
    
    let json_content = serde_json::to_string_pretty(reminders)
        .map_err(|e| format!("Failed to serialize reminders: {}", e))?;
    
    let base64_content = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        json_content.as_bytes()
    );
    
    // First, try to get the current file to get its SHA (for updating)
    let client = reqwest::Client::new();
    let get_url = format!("https://api.github.com/repos/{}/contents/reminders.json", repo);
    
    let get_response = client
        .get(&get_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Toolbox-App")
        .send()
        .await
        .map_err(|e| format!("Failed to check existing file: {}", e))?;
    
    let sha = if get_response.status().is_success() {
        #[derive(Deserialize)]
        struct FileInfo {
            sha: String,
        }
        let file_info: FileInfo = get_response.json().await
            .map_err(|e| format!("Failed to parse file info: {}", e))?;
        Some(file_info.sha)
    } else {
        None
    };
    
    // Create or update the file
    #[derive(Serialize)]
    struct UpdateFile {
        message: String,
        content: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        sha: Option<String>,
    }
    
    let update_data = UpdateFile {
        message: format!("Update reminders backup - {}", chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")),
        content: base64_content,
        sha,
    };
    
    let put_url = format!("https://api.github.com/repos/{}/contents/reminders.json", repo);
    let response = client
        .put(&put_url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Toolbox-App")
        .json(&update_data)
        .send()
        .await
        .map_err(|e| format!("Failed to update file: {}", e))?;
    
    if response.status().is_success() {
        let url = format!("https://github.com/{}/blob/main/reminders.json", repo);
        info!("Successfully synced to repository: {}", url);
        Ok(url)
    } else {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        error!("Failed to sync to repository: {} - {}", status, body);
        Err(format!("GitHub API returned status {}: {}", status, body))
    }
}

pub async fn fetch_reminders_from_github(
    settings: &SyncSettings,
) -> Result<Vec<Reminder>, String> {
    info!("Fetching reminders from GitHub");
    
    let token = settings.github_token.as_ref()
        .ok_or_else(|| "GitHub token not configured".to_string())?;
    
    match settings.sync_method.as_deref() {
        Some("gist") => {
            // For gist, we'd need to store the gist ID somewhere
            Err("Fetching from gist requires gist ID to be stored".to_string())
        }
        Some("repo_json") => {
            let repo = settings.github_repo.as_ref()
                .ok_or_else(|| "GitHub repository not configured".to_string())?;
            fetch_from_repo_json(token, repo).await
        }
        _ => Err("Unsupported sync method".to_string()),
    }
}

async fn fetch_from_repo_json(token: &str, repo: &str) -> Result<Vec<Reminder>, String> {
    debug!("Fetching reminders from GitHub repository: {}", repo);
    
    let client = reqwest::Client::new();
    let url = format!("https://api.github.com/repos/{}/contents/reminders.json", repo);
    
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("User-Agent", "Toolbox-App")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch file: {}", e))?;
    
    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        return Err(format!("GitHub API returned status {}: {}", status, body));
    }
    
    #[derive(Deserialize)]
    struct FileContent {
        content: String,
        encoding: String,
    }
    
    let file_info: FileContent = response.json().await
        .map_err(|e| format!("Failed to parse file info: {}", e))?;
    
    if file_info.encoding != "base64" {
        return Err(format!("Unsupported encoding: {}", file_info.encoding));
    }
    
    let decoded = base64::Engine::decode(
        &base64::engine::general_purpose::STANDARD,
        file_info.content.replace('\n', "")
    )
    .map_err(|e| format!("Failed to decode base64 content: {}", e))?;
    
    let json_str = String::from_utf8(decoded)
        .map_err(|e| format!("Failed to parse UTF-8: {}", e))?;
    
    let reminders: Vec<Reminder> = serde_json::from_str(&json_str)
        .map_err(|e| format!("Failed to parse JSON: {}", e))?;
    
    info!("Successfully fetched {} reminders from repository", reminders.len());
    Ok(reminders)
}
