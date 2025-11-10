use sqlx::SqlitePool;
use log::{info, debug, error};
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};
use std::time::Duration;
use tokio::time::sleep;

const CHECK_INTERVAL_SECONDS: u64 = 30;

pub async fn start_notification_service(pool: SqlitePool, app: AppHandle) {
    info!("Starting notification service");
    
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(CHECK_INTERVAL_SECONDS)).await;
            
            match check_due_reminders(&pool).await {
                Ok(count) => {
                    if count > 0 {
                        debug!("Found {} incomplete reminders, showing notification", count);
                        if let Err(e) = show_notification_list(&app).await {
                            error!("Failed to show notification list: {}", e);
                        }
                    } else {
                        debug!("No incomplete reminders, closing notification if open");
                        // Close notification window if no incomplete reminders
                        if let Some(window) = app.get_webview_window("notification-list") {
                            let _ = window.close();
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to check due reminders: {}", e);
                }
            }
        }
    });
}

async fn check_due_reminders(pool: &SqlitePool) -> Result<usize, sqlx::Error> {
    debug!("Checking for incomplete reminders");
    
    let count: (i64,) = sqlx::query_as(
        r#"
        SELECT COUNT(*)
        FROM reminders
        WHERE completed = 0
        "#
    )
    .fetch_one(pool)
    .await?;
    
    Ok(count.0 as usize)
}

async fn show_notification_list(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let window_label = "notification-list";
    
    // Check if window already exists and is visible
    if let Some(window) = app.get_webview_window(window_label) {
        if window.is_visible().unwrap_or(false) {
            debug!("Notification window already visible, doing nothing");
            return Ok(());
        }
        // Window exists but hidden, show it
        let _ = window.show();
        let _ = window.set_focus();
        debug!("Showing existing notification window");
        return Ok(());
    }
    
    // Get screen dimensions
    let (screen_width, _screen_height) = get_screen_size(app)?;
    
    // Notification list dimensions
    let width = 320.0;
    let height = 400.0;
    let margin = 20.0;
    
    // Position in top-right corner
    let x = screen_width - width - margin;
    let y = margin;
    
    // Create notification window
    let _window = WebviewWindowBuilder::new(
        app,
        window_label,
        WebviewUrl::App("notification.html".into())
    )
    .title("Reminders")
    .inner_size(width, height)
    .position(x, y)
    .resizable(false)
    .minimizable(false)
    .maximizable(false)
    .skip_taskbar(true)
    .always_on_top(true)
    .decorations(false)
    .visible(true)
    .build()?;
    
    info!("Notification list window created");
    
    Ok(())
}

fn get_screen_size(app: &AppHandle) -> Result<(f64, f64), Box<dyn std::error::Error>> {
    // Get primary monitor
    if let Some(monitor) = app.primary_monitor()? {
        let size = monitor.size();
        let scale = monitor.scale_factor();
        Ok((size.width as f64 / scale, size.height as f64 / scale))
    } else {
        // Default fallback
        Ok((1920.0, 1080.0))
    }
}

#[tauri::command]
pub async fn dismiss_notification(
    app: AppHandle,
) -> Result<(), String> {
    let window_label = "notification-list";
    
    if let Some(window) = app.get_webview_window(window_label) {
        window.close().map_err(|e| e.to_string())?;
        info!("Dismissed notification list");
    }
    
    Ok(())
}

#[tauri::command]
pub async fn snooze_reminder(
    pool: tauri::State<'_, SqlitePool>,
    reminder_id: i64,
    minutes: i64,
) -> Result<(), String> {
    info!("Snoozing reminder {} for {} minutes", reminder_id, minutes);
    
    // Update reminder time
    sqlx::query(
        r#"
        UPDATE reminders
        SET time = datetime(time, ? || ' minutes'),
            updated_at = CURRENT_TIMESTAMP
        WHERE id = ?
        "#
    )
    .bind(format!("+{}", minutes))
    .bind(reminder_id)
    .execute(pool.inner())
    .await
    .map_err(|e| e.to_string())?;
    
    Ok(())
}
