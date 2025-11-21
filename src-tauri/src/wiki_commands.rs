use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WikiPage {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    #[serde(default = "default_notebook")]
    pub notebook: String,
    #[serde(default = "default_section")]
    pub section: String,
    pub section_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WikiPageList {
    pub id: String,
    pub title: String,
    pub tags: Vec<String>,
    pub notebook: String,
    pub section: String,
    pub section_id: Option<String>,
    pub updated_at: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WikiRevisionMeta {
    pub id: String,
    pub page_id: String,
    pub title: String,
    pub notebook: String,
    pub section: String,
    pub section_id: Option<String>,
    pub created_at: i64,
}

fn default_notebook() -> String {
    "Notebook".to_string()
}

fn default_section() -> String {
    "Section".to_string()
}

fn get_wiki_dir(app_handle: tauri::AppHandle) -> Result<PathBuf, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data dir: {}", e))?;
    
    let wiki_dir = app_dir.join("wiki");
    
    if !wiki_dir.exists() {
        fs::create_dir_all(&wiki_dir)
            .map_err(|e| format!("Failed to create wiki directory: {}", e))?;
    }

    Ok(wiki_dir)
}

fn get_revision_dir(app_handle: &tauri::AppHandle, page_id: &str) -> Result<PathBuf, String> {
    let wiki_dir = get_wiki_dir(app_handle.clone())?;
    let revisions_dir = wiki_dir.join("revisions").join(page_id);
    if !revisions_dir.exists() {
        fs::create_dir_all(&revisions_dir)
            .map_err(|e| format!("Failed to create revisions directory: {}", e))?;
    }
    Ok(revisions_dir)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Section {
    pub id: String,
    pub name: String,
    pub parent_id: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

fn sections_file(app_handle: &tauri::AppHandle) -> Result<PathBuf, String> {
    let wiki_dir = get_wiki_dir(app_handle.clone())?;
    Ok(wiki_dir.join("sections.json"))
}

fn load_sections(app_handle: &tauri::AppHandle) -> Result<Vec<Section>, String> {
    let path = sections_file(app_handle)?;
    if !path.exists() {
        let root = Section {
            id: "root".to_string(),
            name: "Notebook".to_string(),
            parent_id: None,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };
        let json = serde_json::to_string_pretty(&vec![root])
            .map_err(|e| format!("Failed to serialize sections: {}", e))?;
        fs::write(&path, json)
            .map_err(|e| format!("Failed to write sections file: {}", e))?;
    }

    let data = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read sections file: {}", e))?;
    let sections: Vec<Section> = serde_json::from_str(&data)
        .map_err(|e| format!("Failed to parse sections file: {}", e))?;
    Ok(sections)
}

fn save_sections(app_handle: &tauri::AppHandle, sections: &[Section]) -> Result<(), String> {
    let path = sections_file(app_handle)?;
    let json = serde_json::to_string_pretty(sections)
        .map_err(|e| format!("Failed to serialize sections: {}", e))?;
    fs::write(path, json)
        .map_err(|e| format!("Failed to write sections file: {}", e))?
    ;
    Ok(())
}

fn ensure_section(app_handle: &tauri::AppHandle, section_id: Option<String>) -> Result<String, String> {
    let mut sections = load_sections(app_handle)?;
    let default_id = "root".to_string();
    if section_id.is_none() {
        return Ok(default_id);
    }
    let target = section_id.unwrap();
    if sections.iter().any(|s| s.id == target) {
        return Ok(target);
    }
    // If section missing, create under root
    let now = chrono::Utc::now().timestamp();
    let sec = Section {
        id: target.clone(),
        name: target.clone(),
        parent_id: Some(default_id.clone()),
        created_at: now,
        updated_at: now,
    };
    sections.push(sec);
    save_sections(app_handle, &sections)?;
    Ok(target)
}

fn section_name(app_handle: &tauri::AppHandle, section_id: &str) -> Result<String, String> {
    let sections = load_sections(app_handle)?;
    if let Some(sec) = sections.iter().find(|s| s.id == section_id) {
        return Ok(sec.name.clone());
    }
    Ok(default_section())
}

#[tauri::command]
pub fn create_wiki_page(
    app_handle: tauri::AppHandle,
    title: String,
    content: String,
    tags: Vec<String>,
    notebook: Option<String>,
    section: Option<String>,
    section_id: Option<String>,
) -> Result<WikiPage, String> {
    let wiki_dir = get_wiki_dir(app_handle.clone())?;
    let section_id = ensure_section(&app_handle, section_id)?;
    let section_name = section_name(&app_handle, &section_id)?;
    
    let timestamp = chrono::Utc::now().timestamp();
    let id = format!("{}", timestamp);
    
    let page = WikiPage {
        id: id.clone(),
        title,
        content,
        tags,
        notebook: notebook.unwrap_or_else(default_notebook),
        section: section.unwrap_or(section_name.clone()),
        section_id: Some(section_id),
        created_at: timestamp,
        updated_at: timestamp,
    };
    
    let file_path = wiki_dir.join(format!("{}.json", id));
    let json = serde_json::to_string_pretty(&page)
        .map_err(|e| format!("Failed to serialize page: {}", e))?;
    
    fs::write(file_path, json)
        .map_err(|e| format!("Failed to write page: {}", e))?;
    
    Ok(page)
}

fn save_revision(
    app_handle: &tauri::AppHandle,
    page: &WikiPage,
) -> Result<WikiRevisionMeta, String> {
    let revisions_dir = get_revision_dir(app_handle, &page.id)?;
    let revision_id = chrono::Utc::now().timestamp();
    let file_path = revisions_dir.join(format!("{}.json", revision_id));

    let json = serde_json::to_string_pretty(page)
        .map_err(|e| format!("Failed to serialize revision: {}", e))?;

    fs::write(&file_path, json)
        .map_err(|e| format!("Failed to write revision: {}", e))?;

    Ok(WikiRevisionMeta {
        id: revision_id.to_string(),
        page_id: page.id.clone(),
        title: page.title.clone(),
        notebook: page.notebook.clone(),
        section: page.section.clone(),
        section_id: page.section_id.clone(),
        created_at: revision_id,
    })
}

#[tauri::command]
pub fn update_wiki_page(
    app_handle: tauri::AppHandle,
    id: String,
    title: String,
    content: String,
    tags: Vec<String>,
    notebook: Option<String>,
    section: Option<String>,
    section_id: Option<String>,
) -> Result<WikiPage, String> {
    let wiki_dir = get_wiki_dir(app_handle.clone())?;
    let file_path = wiki_dir.join(format!("{}.json", id));
    
    let json = fs::read_to_string(&file_path)
        .map_err(|e| format!("Failed to read page: {}", e))?;
    
    let mut page: WikiPage = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse page: {}", e))?;

    // Save current state as a revision before updating
    let _ = save_revision(&app_handle, &page);
    
    page.title = title;
    page.content = content;
    page.tags = tags;
    page.notebook = notebook.unwrap_or_else(|| page.notebook.clone());

    let target_section_id = section_id.unwrap_or_else(|| page.section_id.clone().unwrap_or_else(|| "root".to_string()));
    let ensured_section_id = ensure_section(&app_handle, Some(target_section_id))?;
    let section_name = section.unwrap_or_else(|| section_name(&app_handle, &ensured_section_id).unwrap_or_else(|_| default_section()));

    page.section_id = Some(ensured_section_id);
    page.section = section_name;
    page.updated_at = chrono::Utc::now().timestamp();
    
    let json = serde_json::to_string_pretty(&page)
        .map_err(|e| format!("Failed to serialize page: {}", e))?;
    
    fs::write(file_path, json)
        .map_err(|e| format!("Failed to write page: {}", e))?;
    
    Ok(page)
}

#[tauri::command]
pub fn get_wiki_page(app_handle: tauri::AppHandle, id: String) -> Result<WikiPage, String> {
    let wiki_dir = get_wiki_dir(app_handle)?;
    let file_path = wiki_dir.join(format!("{}.json", id));
    
    let json = fs::read_to_string(file_path)
        .map_err(|e| format!("Failed to read page: {}", e))?;
    
    let page: WikiPage = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse page: {}", e))?;
    
    Ok(page)
}

#[tauri::command]
pub fn list_wiki_pages(app_handle: tauri::AppHandle) -> Result<Vec<WikiPageList>, String> {
    let wiki_dir = get_wiki_dir(app_handle)?;
    
    let mut pages = Vec::new();
    
    let entries = fs::read_dir(wiki_dir)
        .map_err(|e| format!("Failed to read wiki directory: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(page) = serde_json::from_str::<WikiPage>(&json) {
                        pages.push(WikiPageList {
                            id: page.id,
                            title: page.title,
                            tags: page.tags,
                            notebook: page.notebook,
                            section: page.section,
                            section_id: page.section_id,
                            updated_at: page.updated_at,
                        });
                    }
                }
            }
        }
    }
    
    pages.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    
    Ok(pages)
}

#[tauri::command]
pub fn delete_wiki_page(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    let wiki_dir = get_wiki_dir(app_handle)?;
    let file_path = wiki_dir.join(format!("{}.json", id));
    
    fs::remove_file(file_path)
        .map_err(|e| format!("Failed to delete page: {}", e))?;

    Ok(())
}

#[tauri::command]
pub fn list_wiki_revisions(
    app_handle: tauri::AppHandle,
    page_id: String,
) -> Result<Vec<WikiRevisionMeta>, String> {
    let revisions_dir = get_revision_dir(&app_handle, &page_id)?;
    let mut revisions = Vec::new();

    if revisions_dir.exists() {
        let entries = fs::read_dir(revisions_dir)
            .map_err(|e| format!("Failed to read revisions directory: {}", e))?;

        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                    if let Ok(json) = fs::read_to_string(&path) {
                        if let Ok(page) = serde_json::from_str::<WikiPage>(&json) {
                            revisions.push(WikiRevisionMeta {
                                id: entry
                                    .file_name()
                                    .to_string_lossy()
                                    .replace(".json", ""),
                                page_id: page.id.clone(),
                                title: page.title,
                                notebook: page.notebook,
                                section: page.section,
                                section_id: page.section_id,
                                created_at: page.updated_at,
                            });
                        }
                    }
                }
        }
    }

    revisions.sort_by(|a, b| b.created_at.cmp(&a.created_at));
    Ok(revisions)
}

#[tauri::command]
pub fn restore_wiki_revision(
    app_handle: tauri::AppHandle,
    page_id: String,
    revision_id: String,
) -> Result<WikiPage, String> {
    let revisions_dir = get_revision_dir(&app_handle, &page_id)?;
    let revision_path = revisions_dir.join(format!("{}.json", revision_id));

    if !revision_path.exists() {
        return Err("Revision not found".into());
    }

    // Load current page to keep a backup revision before restoring
    let current_page = get_wiki_page(app_handle.clone(), page_id.clone())?;
    let _ = save_revision(&app_handle, &current_page);

    let json = fs::read_to_string(&revision_path)
        .map_err(|e| format!("Failed to read revision: {}", e))?;

    let mut revision_page: WikiPage = serde_json::from_str(&json)
        .map_err(|e| format!("Failed to parse revision: {}", e))?;

    // Apply restore (update timestamp)
    revision_page.updated_at = chrono::Utc::now().timestamp();

    let wiki_dir = get_wiki_dir(app_handle)?;
    let page_path = wiki_dir.join(format!("{}.json", page_id));

    let page_json = serde_json::to_string_pretty(&revision_page)
        .map_err(|e| format!("Failed to serialize restored page: {}", e))?;

    fs::write(page_path, page_json)
        .map_err(|e| format!("Failed to write restored page: {}", e))?;

    Ok(revision_page)
}

#[tauri::command]
pub fn search_wiki_pages(
    app_handle: tauri::AppHandle,
    query: String,
) -> Result<Vec<WikiPageList>, String> {
    let wiki_dir = get_wiki_dir(app_handle)?;
    let query_lower = query.to_lowercase();
    
    let mut pages = Vec::new();
    
    let entries = fs::read_dir(wiki_dir)
        .map_err(|e| format!("Failed to read wiki directory: {}", e))?;
    
    for entry in entries {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Ok(json) = fs::read_to_string(&path) {
                    if let Ok(page) = serde_json::from_str::<WikiPage>(&json) {
                        let title_match = page.title.to_lowercase().contains(&query_lower);
                        let content_match = page.content.to_lowercase().contains(&query_lower);
                        let tag_match = page.tags.iter().any(|t| t.to_lowercase().contains(&query_lower));
                        let notebook_match = page.notebook.to_lowercase().contains(&query_lower);
                        let section_match = page.section.to_lowercase().contains(&query_lower);
                        
                        if title_match || content_match || tag_match || notebook_match || section_match {
                            pages.push(WikiPageList {
                                id: page.id,
                                title: page.title,
                                tags: page.tags,
                                notebook: page.notebook,
                                section: page.section,
                                section_id: page.section_id,
                                updated_at: page.updated_at,
                            });
                        }
                    }
                }
            }
        }
    }
    
    pages.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    
    Ok(pages)
}

fn count_pages_in_section(app_handle: &tauri::AppHandle, section_id: &str) -> Result<usize, String> {
    let wiki_dir = get_wiki_dir(app_handle.clone())?;
    let mut count = 0;
    let entries = fs::read_dir(wiki_dir)
        .map_err(|e| format!("Failed to read wiki directory: {}", e))?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            if let Ok(json) = fs::read_to_string(&path) {
                if let Ok(page) = serde_json::from_str::<WikiPage>(&json) {
                    if let Some(sec_id) = &page.section_id {
                        if sec_id == section_id {
                            count += 1;
                        }
                    }
                }
            }
        }
    }
    Ok(count)
}

#[tauri::command]
pub fn list_sections(app_handle: tauri::AppHandle) -> Result<Vec<Section>, String> {
    load_sections(&app_handle)
}

#[tauri::command]
pub fn create_section(
    app_handle: tauri::AppHandle,
    name: String,
    parent_id: Option<String>,
) -> Result<Section, String> {
    let mut sections = load_sections(&app_handle)?;
    let parent = parent_id.unwrap_or_else(|| "root".to_string());
    if !sections.iter().any(|s| s.id == parent) {
        return Err("Parent section not found".into());
    }
    let id = uuid::Uuid::new_v4().to_string();
    let now = chrono::Utc::now().timestamp();
    let section = Section {
        id: id.clone(),
        name: if name.is_empty() { "Untitled Section".to_string() } else { name },
        parent_id: Some(parent),
        created_at: now,
        updated_at: now,
    };
    sections.push(section.clone());
    save_sections(&app_handle, &sections)?;
    Ok(section)
}

#[tauri::command]
pub fn update_section(
    app_handle: tauri::AppHandle,
    id: String,
    name: String,
) -> Result<Section, String> {
    let mut sections = load_sections(&app_handle)?;
    let now = chrono::Utc::now().timestamp();
    let mut updated: Option<Section> = None;
    if let Some(sec) = sections.iter_mut().find(|s| s.id == id) {
        sec.name = if name.is_empty() { "Untitled Section".to_string() } else { name };
        sec.updated_at = now;
        updated = Some(sec.clone());
    }

    if updated.is_none() {
        return Err("Section not found".into());
    }

    save_sections(&app_handle, &sections)?;
    Ok(updated.unwrap())
}

#[tauri::command]
pub fn delete_section(app_handle: tauri::AppHandle, id: String) -> Result<(), String> {
    if id == "root" {
        return Err("Cannot delete root section".into());
    }
    let mut sections = load_sections(&app_handle)?;
    if sections.iter().any(|s| s.parent_id.as_deref() == Some(&id)) {
        return Err("Section has child sections".into());
    }
    let pages_in = count_pages_in_section(&app_handle, &id)?;
    if pages_in > 0 {
        return Err("Section has pages; move or delete them first".into());
    }
    let before = sections.len();
    sections.retain(|s| s.id != id);
    if sections.len() == before {
        return Err("Section not found".into());
    }
    save_sections(&app_handle, &sections)?;
    Ok(())
}
