use log::{info, debug};
use tauri::{
    AppHandle, 
    Manager,
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
};

#[cfg(target_os = "macos")]
use tauri::ActivationPolicy;

pub fn setup_tray(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    info!("Setting up system tray...");
    
    // Create tray menu
    debug!("Creating tray menu");
    let show_i = MenuItem::with_id(app, "show", "Show", true, None::<&str>)?;
    let quit_i = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show_i, &quit_i])?;

    // Create tray icon
    debug!("Building tray icon");
    let _tray = TrayIconBuilder::new()
        .icon(app.default_window_icon().unwrap().clone())
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id().as_ref() {
            "show" => {
                info!("Tray menu: Show clicked");
                #[cfg(target_os = "macos")]
                {
                    debug!("Setting macOS activation policy to Regular");
                    let _ = app.set_activation_policy(ActivationPolicy::Regular);
                }
                
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.show();
                    let _ = window.set_focus();
                    info!("Main window shown and focused");
                }
            }
            "quit" => {
                info!("Tray menu: Quit clicked - exiting application");
                app.exit(0);
            }
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            use tauri::tray::TrayIconEvent;
            if let TrayIconEvent::Click { button, .. } = event {
                use tauri::tray::MouseButton;
                if button == MouseButton::Left {
                    debug!("Tray icon left-clicked - showing window");
                    let app = tray.app_handle();
                    
                    #[cfg(target_os = "macos")]
                    {
                        debug!("Setting macOS activation policy to Regular");
                        let _ = app.set_activation_policy(ActivationPolicy::Regular);
                    }
                    
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        info!("Main window shown via tray click");
                    }
                }
            }
        })
        .build(app)?;
    
    info!("Tray icon created successfully");
    Ok(())
}

pub fn setup_window_handlers(app: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    // Handle window close event to hide instead of quit
    if let Some(window) = app.get_webview_window("main") {
        let window_clone = window.clone();
        let app_handle = app.app_handle().clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                info!("Window close requested - hiding instead of quitting");
                // Prevent the window from closing and hide it instead
                api.prevent_close();
                let _ = window_clone.hide();
                
                // Hide from dock on macOS
                #[cfg(target_os = "macos")]
                {
                    debug!("Setting macOS activation policy to Accessory (hide from dock)");
                    let _ = app_handle.set_activation_policy(ActivationPolicy::Accessory);
                }
            }
        });
    }

    info!("Window handlers configured successfully");
    Ok(())
}
