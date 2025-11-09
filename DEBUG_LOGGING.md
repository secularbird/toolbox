# Debug Logging

This application includes comprehensive debug logging to help with development and troubleshooting.

## Features

### Log Levels
- **INFO**: General information about application flow (e.g., "Adding reminder", "Window shown")
- **DEBUG**: Detailed diagnostic information (e.g., "Tray icon left-clicked", "Setting macOS activation policy")
- **WARN**: Warning messages for non-critical issues (e.g., "Reminder not found")

### Logged Events

#### Application Lifecycle
- Application startup and shutdown
- Setup completion
- Tray icon creation

#### Window Management
- Window show/hide events
- Window close requests
- macOS dock visibility changes

#### Reminder Operations
- Adding reminders (with details)
- Fetching reminders
- Toggling reminder completion
- Deleting reminders

#### Tray Interactions
- Tray icon clicks (left/right)
- Menu item selections

## Controlling Debug Output

### From the UI
Toggle the "🐛 Debug Logs" checkbox in the sidebar to enable/disable debug output.

### From the Console
Set the `RUST_LOG` environment variable before running:

```bash
# Show all debug logs
RUST_LOG=debug npm run tauri dev

# Show only info and above
RUST_LOG=info npm run tauri dev

# Show only warnings and errors
RUST_LOG=warn npm run tauri dev
```

### Default Behavior
- Debug mode is **enabled by default** in the application
- All log levels (debug, info, warn) are shown in the console
- Can be toggled at runtime via the UI

## Example Output

```
[2024-11-09 16:16:01] INFO  tauri_vue_app_lib: Application starting...
[2024-11-09 16:16:01] INFO  tauri_vue_app_lib: Setting up application...
[2024-11-09 16:16:01] DEBUG tauri_vue_app_lib: Creating tray menu
[2024-11-09 16:16:01] DEBUG tauri_vue_app_lib: Building tray icon
[2024-11-09 16:16:01] INFO  tauri_vue_app_lib: Tray icon created successfully
[2024-11-09 16:16:01] INFO  tauri_vue_app_lib: Application setup completed successfully
[2024-11-09 16:16:15] INFO  tauri_vue_app_lib: Adding reminder: title='Meeting', category='work', time='2024-11-10T10:00', frequency='once'
[2024-11-09 16:16:15] DEBUG tauri_vue_app_lib: Reminder added with id=1, total reminders=1
```

## Viewing Logs

### Development Mode
Logs are output to the terminal where you run `npm run tauri dev`.

### Production Build
On macOS, view logs using:
```bash
# View logs in real-time
log stream --predicate 'process == "tauri-vue-app"' --level debug

# Or check Console.app
```

On Linux:
```bash
# Logs may be in stdout/stderr or systemd journal
journalctl -f | grep tauri-vue-app
```

On Windows:
- Logs appear in the console if running from command line
- Or use a tool like DebugView to capture debug output
