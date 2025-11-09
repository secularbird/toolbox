use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tauri::{Manager, menu::{Menu, MenuItem}, tray::{TrayIconBuilder, TrayIconEvent}};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Reminder {
    id: u32,
    title: String,
    description: String,
    time: String,
    completed: bool,
    category: String,
    frequency: String,
}

struct AppState {
    reminders: Mutex<Vec<Reminder>>,
    next_id: Mutex<u32>,
}

#[tauri::command]
fn add_reminder(
    title: String,
    description: String,
    time: String,
    category: String,
    frequency: String,
    state: tauri::State<AppState>,
) -> Result<(), String> {
    let mut reminders = state.reminders.lock().unwrap();
    let mut next_id = state.next_id.lock().unwrap();
    
    let reminder = Reminder {
        id: *next_id,
        title,
        description,
        time,
        completed: false,
        category,
        frequency,
    };
    
    reminders.push(reminder);
    *next_id += 1;
    
    Ok(())
}

#[tauri::command]
fn get_reminders(state: tauri::State<AppState>) -> Result<Vec<Reminder>, String> {
    let reminders = state.reminders.lock().unwrap();
    Ok(reminders.clone())
}

#[tauri::command]
fn toggle_reminder(id: u32, state: tauri::State<AppState>) -> Result<(), String> {
    let mut reminders = state.reminders.lock().unwrap();
    
    if let Some(reminder) = reminders.iter_mut().find(|r| r.id == id) {
        reminder.completed = !reminder.completed;
        Ok(())
    } else {
        Err("Reminder not found".to_string())
    }
}

#[tauri::command]
fn delete_reminder(id: u32, state: tauri::State<AppState>) -> Result<(), String> {
    let mut reminders = state.reminders.lock().unwrap();
    
    if let Some(pos) = reminders.iter().position(|r| r.id == id) {
        reminders.remove(pos);
        Ok(())
    } else {
        Err("Reminder not found".to_string())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState {
            reminders: Mutex::new(Vec::new()),
            next_id: Mutex::new(1),
        })
        .invoke_handler(tauri::generate_handler![
            add_reminder,
            get_reminders,
            toggle_reminder,
            delete_reminder
        ])
        .setup(|app| {
            // Create tray menu
            let show = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
            let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show, &quit])?;

            // Create tray icon
            let _tray = TrayIconBuilder::with_id("main")
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            // Handle window close event to hide instead of quit
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent the window from closing and hide it instead
                        api.prevent_close();
                        let _ = window_clone.hide();
                    }
                });
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
