use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use tauri::command;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DiskItem {
    pub name: String,
    pub path: String,
    pub size: u64,
    pub is_dir: bool,
    pub children: Option<Vec<DiskItem>>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScanProgress {
    pub current_path: String,
    pub items_scanned: usize,
}

fn get_dir_size(path: &Path, max_depth: Option<usize>, current_depth: usize) -> Result<(u64, Vec<DiskItem>), String> {
    if let Some(max) = max_depth {
        if current_depth >= max {
            return Ok((0, vec![]));
        }
    }

    let mut total_size = 0u64;
    let mut children = Vec::new();

    let entries = fs::read_dir(path).map_err(|e| e.to_string())?;

    for entry in entries {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();
                let metadata = match entry.metadata() {
                    Ok(meta) => meta,
                    Err(e) => {
                        children.push(DiskItem {
                            name: entry.file_name().to_string_lossy().to_string(),
                            path: entry_path.to_string_lossy().to_string(),
                            size: 0,
                            is_dir: false,
                            children: None,
                            error: Some(format!("Permission denied: {}", e)),
                        });
                        continue;
                    }
                };

                let name = entry.file_name().to_string_lossy().to_string();
                
                if metadata.is_dir() {
                    match get_dir_size(&entry_path, max_depth, current_depth + 1) {
                        Ok((size, sub_children)) => {
                            total_size += size;
                            children.push(DiskItem {
                                name,
                                path: entry_path.to_string_lossy().to_string(),
                                size,
                                is_dir: true,
                                children: if sub_children.is_empty() { None } else { Some(sub_children) },
                                error: None,
                            });
                        }
                        Err(e) => {
                            children.push(DiskItem {
                                name,
                                path: entry_path.to_string_lossy().to_string(),
                                size: 0,
                                is_dir: true,
                                children: None,
                                error: Some(e),
                            });
                        }
                    }
                } else {
                    let file_size = metadata.len();
                    total_size += file_size;
                    children.push(DiskItem {
                        name,
                        path: entry_path.to_string_lossy().to_string(),
                        size: file_size,
                        is_dir: false,
                        children: None,
                        error: None,
                    });
                }
            }
            Err(e) => {
                log::warn!("Error reading directory entry: {}", e);
            }
        }
    }

    // Sort children by size (largest first)
    children.sort_by(|a, b| b.size.cmp(&a.size));

    Ok((total_size, children))
}

#[command]
pub async fn scan_directory(path: String, max_depth: Option<usize>) -> Result<DiskItem, String> {
    let path_buf = PathBuf::from(&path);
    
    if !path_buf.exists() {
        return Err("Path does not exist".to_string());
    }

    let metadata = fs::metadata(&path_buf).map_err(|e| e.to_string())?;

    if !metadata.is_dir() {
        return Ok(DiskItem {
            name: path_buf.file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string(),
            path: path.clone(),
            size: metadata.len(),
            is_dir: false,
            children: None,
            error: None,
        });
    }

    let (size, children) = get_dir_size(&path_buf, max_depth, 0)?;

    Ok(DiskItem {
        name: path_buf.file_name()
            .unwrap_or_else(|| path_buf.as_os_str())
            .to_string_lossy()
            .to_string(),
        path: path.clone(),
        size,
        is_dir: true,
        children: if children.is_empty() { None } else { Some(children) },
        error: None,
    })
}

#[command]
pub async fn get_home_directory() -> Result<String, String> {
    dirs::home_dir()
        .ok_or_else(|| "Could not determine home directory".to_string())
        .map(|p| p.to_string_lossy().to_string())
}

#[command]
pub async fn get_system_roots() -> Result<Vec<String>, String> {
    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsString;
        use std::os::windows::ffi::OsStringExt;
        
        let mut roots = Vec::new();
        let drives = unsafe {
            let logical_drives = winapi::um::fileapi::GetLogicalDrives();
            logical_drives
        };
        
        for i in 0..26 {
            if drives & (1 << i) != 0 {
                let drive_letter = (b'A' + i) as char;
                roots.push(format!("{}:\\", drive_letter));
            }
        }
        Ok(roots)
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        Ok(vec!["/".to_string()])
    }
}

#[command]
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB", "PB"];
    
    if bytes == 0 {
        return "0 B".to_string();
    }
    
    let base = 1024f64;
    let exp = (bytes as f64).log(base).floor() as usize;
    let exp = exp.min(UNITS.len() - 1);
    
    let size = bytes as f64 / base.powi(exp as i32);
    
    if exp == 0 {
        format!("{} {}", bytes, UNITS[exp])
    } else {
        format!("{:.2} {}", size, UNITS[exp])
    }
}
