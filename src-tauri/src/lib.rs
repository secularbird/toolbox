mod models;
mod config;
mod commands;
mod tray;
mod state;

use log::info;
use tauri::Manager;
use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();
    
    info!("Application starting...");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState::new())
        .invoke_handler(tauri::generate_handler![
            commands::add_reminder,
            commands::get_reminders,
            commands::toggle_reminder,
            commands::delete_reminder,
            commands::set_debug_mode,
            commands::get_debug_mode
        ])
        .setup(|app| {
            info!("Setting up application...");
            
            // Setup system tray
            tray::setup_tray(app.app_handle())?;
            
            // Setup window handlers
            tray::setup_window_handlers(app.app_handle())?;

            info!("Application setup completed successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    info!("Application exited");
}
