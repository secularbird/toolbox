mod models;
mod config;
mod commands;
mod tray;
mod database;

use log::info;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Debug)
        .init();
    
    info!("Application starting...");
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
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
            
            // Initialize database
            let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            let db_path = app_dir.join("reminders.db");
            info!("Database path: {:?}", db_path);
            
            let pool = tauri::async_runtime::block_on(async {
                database::init_database(db_path).await.expect("Failed to initialize database")
            });
            
            // Store pool in app state
            app.manage(pool);
            
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
