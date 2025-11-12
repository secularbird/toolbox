# Tauri Vue Reminder App

A desktop reminder application built with Tauri, Vue.js, and Rust with local SQLite storage.

## Features

- ✅ **System Tray Integration** - Lives in system tray, right-click for menu
- ✅ **Local SQLite Database** - All data stored locally
- ✅ **Smart Notifications** - Popup reminders for incomplete tasks
- ✅ **Privacy-Focused** - All data stays on your device
- ✅ **Auto-Hide Dock** - Minimal dock presence when main window closed
- ✅ **Debug Logging** - Comprehensive logging for troubleshooting

## Quick Start

### Development

```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri dev
```

### Production Build

```bash
# Build the application
npm run tauri build
```

The built application will be in `src-tauri/target/release/`.

## Architecture

```text
┌─────────────────────────────────────────┐
│          Tauri Application              │
│  ┌────────────┐      ┌──────────────┐  │
│  │ Vue.js UI  │◄────►│ Rust Backend │  │
│  └────────────┘      └───────┬──────┘  │
│                              │          │
│                       ┌──────▼──────┐   │
│                       │   SQLite    │   │
│                       └─────────────┘   │
└─────────────────────────────────────────┘
```

## Components

### Frontend (Vue.js)

- Main window showing all reminders
- Add/edit/delete reminders
- Filter by category and completion status

### Backend (Rust)

- Tauri commands for database operations
- Notification service (checks every 30s)
- System tray management

### Notification Window

- Standalone HTML window
- Shows incomplete reminders
- Snooze and complete actions
- Auto-refreshes every 30 seconds
- Positioned in top-right corner

## Data Flow

1. **Local-First**: All operations write to local SQLite database immediately
2. **Privacy-Focused**: All data stays on your device
3. **Event-Driven**: UI updates via Tauri events

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
)
```

## Configuration

### Debug Logging

Set log level via environment variable:
```bash
RUST_LOG=debug npm run tauri dev
```

Or programmatically via Tauri command:
```javascript
await invoke('set_debug_mode', { enabled: true });
```

### Database Location

- **macOS**: `~/Library/Application Support/com.yaozhuang.tauri-vue-app/reminders.db`
- **Linux**: `~/.local/share/com.yaozhuang.tauri-vue-app/reminders.db`
- **Windows**: `%APPDATA%\com.yaozhuang.tauri-vue-app\reminders.db`

## Project Structure

### Project Structure

```text
tauri-vue-app/
├── src/                    # Vue.js frontend
│   ├── components/
│   └── App.vue
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── commands/      # Tauri commands
│   │   ├── database/      # SQLite operations
│   │   ├── notifications/ # Notification service
│   │   ├── tray/          # System tray
│   │   └── models/        # Data models
│   └── tauri.conf.json
├── notification.html      # Notification window
└── README.md
```

## Troubleshooting

### Database Errors

Reset database:
```bash
rm ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/reminders.db
```

### Build Errors

Clean and rebuild:
```bash
cd src-tauri
cargo clean
cargo build
```

## Logs

View logs in terminal during development:
```bash
npm run tauri dev 2>&1 | tee app.log
```

Look for:
- `[INFO]` - General information
- `[DEBUG]` - Detailed debugging info  
- `[WARN]` - Warnings (non-fatal)
- `[ERROR]` - Errors (may be fatal)

## System Requirements

- **macOS**: 10.15+
- **Linux**: Modern distribution with GTK3
- **Windows**: Windows 7+

## Technologies

- **Frontend**: Vue.js 3, TypeScript, Vite
- **Backend**: Rust, Tauri 2.0
- **Database**: SQLite with SQLx
- **WebSocket**: Rust (tokio-tungstenite), Go (gorilla/websocket)
- **UI**: Native system styling

## License

MIT

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## Support

For issues and questions:

- Check logs first (enable debug mode)
- Ensure database file is accessible
- Check system tray for application status
