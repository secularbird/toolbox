mod models;
mod commands;
#[cfg(not(any(target_os = "android", target_os = "ios")))]
mod tray;
mod database;
mod notifications;

use log::{info, error};
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
            commands::get_due_reminders,
            commands::toggle_reminder,
            commands::delete_reminder,
            commands::update_reminder,
            commands::set_debug_mode,
            commands::get_debug_mode,
            commands::broadcast_reminders,
            notifications::dismiss_notification,
            notifications::snooze_reminder
        ])
        .setup(|app| {
            info!("Setting up application...");
            
            let app_handle = app.app_handle().clone();
            
            // Initialize database asynchronously to avoid blocking on mobile
            tauri::async_runtime::spawn(async move {
                // Get app data directory
                let app_dir = match app_handle.path().app_data_dir() {
                    Ok(dir) => dir,
                    Err(e) => {
                        error!("Failed to get app data dir: {}", e);
                        return;
                    }
                };
                
                let db_path = app_dir.join("reminders.db");
                info!("Database path: {:?}", db_path);
                
                match database::init_database(db_path).await {
                    Ok(pool) => {
                        info!("Database initialized successfully");
                        app_handle.manage(pool.clone());
                        
                        // Start notification service
                        notifications::start_notification_service(pool, app_handle.clone()).await;
                    }
                    Err(e) => {
                        error!("Failed to initialize database: {}", e);
                    }
                }
            });
            
            // Setup system tray (desktop only)
            #[cfg(not(any(target_os = "android", target_os = "ios")))]
            {
                tray::setup_tray(app.app_handle())?;
                tray::setup_window_handlers(app.app_handle())?;
            }

            info!("Application setup completed successfully");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
    
    info!("Application exited");
}
