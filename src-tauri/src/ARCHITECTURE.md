# Project Architecture

## Directory Structure

```
src-tauri/src/
├── lib.rs              # Main entry point and application setup
├── main.rs             # Binary entry point
├── commands/           # Tauri command handlers
│   └── mod.rs         # Reminder and config commands
├── config/             # Application configuration
│   └── mod.rs         # AppConfig struct
├── models/             # Data models
│   ├── mod.rs         # Model exports
│   └── reminder.rs    # Reminder data structure
├── state.rs            # Application state management
└── tray/               # System tray functionality
    └── mod.rs         # Tray icon and window handlers
```

## Module Overview

### `lib.rs`
Main library entry point that:
- Initializes the logger
- Sets up Tauri application
- Registers command handlers
- Configures system tray and window behavior

### `models/`
Data structures and types:
- **`reminder.rs`**: Reminder entity with fields like id, title, description, time, etc.

### `config/`
Application configuration:
- **`AppConfig`**: Settings like debug mode

### `state.rs`
Global application state:
- **`AppState`**: Manages reminders, IDs, and configuration with thread-safe Mutex

### `commands/`
Tauri command handlers (invokable from frontend):
- `add_reminder()` - Create new reminder
- `get_reminders()` - Fetch all reminders
- `toggle_reminder()` - Mark reminder as complete/incomplete
- `delete_reminder()` - Remove a reminder
- `set_debug_mode()` - Enable/disable debug logging
- `get_debug_mode()` - Get current debug mode status

### `tray/`
System tray functionality:
- **`setup_tray()`**: Creates tray icon with menu (Show/Quit)
- **`setup_window_handlers()`**: Handles window close to hide instead of quit

## Data Flow

```
Frontend (Vue)
    ↓ invoke()
Commands Module
    ↓ access
AppState (Mutex)
    ↓ contains
Models (Reminder, Config)
```

## Key Features by Module

| Module | Responsibility |
|--------|---------------|
| `commands/` | Business logic, state mutations |
| `models/` | Data structures, serialization |
| `state.rs` | Thread-safe state management |
| `tray/` | Platform-specific UI (system tray) |
| `config/` | Application settings |

## Adding New Features

### New Command
1. Add function to `commands/mod.rs`
2. Mark with `#[tauri::command]`
3. Register in `lib.rs` `invoke_handler![]`

### New Model
1. Create file in `models/`
2. Add `pub use` in `models/mod.rs`
3. Derive `Serialize`, `Deserialize` for Tauri IPC

### New Tray Action
1. Add MenuItem in `tray/setup_tray()`
2. Handle in `.on_menu_event()` closure

## Thread Safety

All shared state uses `Mutex<T>` for thread-safe access:
```rust
state.reminders.lock().unwrap()  // Acquire lock before access
```

## Logging

Integrated throughout using `log` crate:
- `info!()` - General application flow
- `debug!()` - Detailed diagnostic info
- `warn!()` - Non-critical issues
