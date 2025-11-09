# Tauri Vue Reminder App - Project Summary

## Overview

A modern, privacy-focused reminder application built with Tauri 2 and Vue 3, featuring local SQLite storage and system tray integration.

## Tech Stack

### Frontend
- **Vue 3** with TypeScript
- **Vite** for build tooling
- Scoped CSS for styling

### Backend
- **Rust** with Tauri 2
- **SQLx** for SQLite database
- **Tokio** async runtime
- **Log** + **env_logger** for debugging

## Architecture

```
tauri-vue-app/
├── src/                      # Vue frontend
│   ├── App.vue              # Main component
│   └── main.ts              # Entry point
├── src-tauri/               # Rust backend
│   └── src/
│       ├── commands/        # Tauri command handlers
│       ├── config/          # App configuration
│       ├── database/        # SQLite operations
│       ├── models/          # Data models
│       ├── tray/            # System tray functionality
│       └── lib.rs           # Main entry point
└── sync-server/             # Optional Go sync server
    └── main.go
```

## Features

### ✅ Implemented

1. **Local SQLite Database**
   - Persistent storage
   - Fast queries
   - No internet required
   - Complete privacy

2. **System Tray Integration**
   - Minimize to tray
   - Quick access menu
   - Hides from dock (macOS)
   - Background operation

3. **Category Management**
   - Built-in categories
   - Custom categories
   - Color-coded
   - Counter badges

4. **Recurring Reminders**
   - Once, Daily, Weekly, Monthly, Yearly
   - Frequency tracking

5. **Debug Logging**
   - Toggleable from UI
   - Multiple log levels
   - Operation tracking

6. **CI/CD**
   - GitHub Actions workflow
   - Multi-platform builds
   - Automatic releases

## Database Schema

```sql
CREATE TABLE reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    time TEXT NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0,
    category TEXT NOT NULL,
    frequency TEXT NOT NULL,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);
```

## API Commands

All commands are async and work with the local SQLite database:

- `add_reminder` - Create new reminder
- `get_reminders` - Fetch all reminders
- `toggle_reminder` - Mark complete/incomplete
- `delete_reminder` - Remove reminder
- `set_debug_mode` - Enable/disable debug logs
- `get_debug_mode` - Get debug status

## Build & Run

### Development
```bash
npm install
npm run tauri dev
```

### Production Build
```bash
npm run tauri build
```

Outputs:
- **macOS**: `src-tauri/target/release/bundle/dmg/*.dmg`
- **Linux**: `src-tauri/target/release/bundle/deb/*.deb`
- **Windows**: `src-tauri/target/release/bundle/msi/*.msi`

## Database Location

| Platform | Path |
|----------|------|
| macOS | `~/Library/Application Support/com.yaozhuang.tauri-vue-app/reminders.db` |
| Linux | `~/.local/share/tauri-vue-app/reminders.db` |
| Windows | `%APPDATA%\com.yaozhuang.tauri-vue-app\reminders.db` |

## Optional: Sync Server

A Go-based sync server is included in `sync-server/` for future cloud sync functionality.

**Status**: Not integrated yet (local-only storage currently)

To run the server:
```bash
cd sync-server
go run main.go
```

## Documentation

- `README.md` - Main project documentation
- `STORAGE.md` - Local storage guide
- `DEBUG_LOGGING.md` - Debug logging guide
- `BUILD_OUTPUT.md` - Build configuration
- `ARCHITECTURE.md` - Code structure (in src-tauri/src/)
- `sync-server/README.md` - Sync server docs (future feature)

## Key Design Decisions

1. **Local-First**: All data stored locally for privacy and speed
2. **Modular Rust Code**: Separated into logical modules (commands, database, tray, etc.)
3. **Async Operations**: All database operations are async for better performance
4. **System Tray**: Runs in background, doesn't quit on window close
5. **Debug Logging**: Comprehensive logging for development and troubleshooting

## Future Enhancements

- [ ] Desktop notifications
- [ ] Server sync (optional)
- [ ] Cloud backup
- [ ] Multi-device sync
- [ ] Search functionality
- [ ] Priority levels
- [ ] Export/import

## License

MIT
