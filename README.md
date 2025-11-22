# 📚 Tauri Wiki & Reminder App

A powerful desktop application with **Wiki as the primary feature**, complemented by a reminder system. Built with Tauri, Vue.js 3, and Rust with local storage.

## ✨ Main Features

### 📚 Wiki System (Primary Feature)

**Markdown Editor with Live Preview**
- Real-time markdown rendering with syntax highlighting
- Side-by-side editor and preview
- Full markdown support with extended syntax

**Advanced Features**
- 📄 **Document Import**: Import Word documents (.docx, .doc) with preview
- ⊞ **Table Insertion**: Visual table builder with keyboard shortcut (Ctrl+Shift+T)
- 🖱️ **Context Menu**: Right-click pages for quick actions (Open, Rename, Delete)
- 📁 **Section Management**: Hierarchical organization like OneNote
- 🔍 **Search**: Full-text search across pages and tags
- 🏷️ **Tags**: Flexible categorization with tag filtering
- 📋 **Version History**: Track changes and restore previous versions
- 💾 **Auto-save**: Automatic draft saving with unsaved changes indicator
- 📊 **Mermaid Diagrams**: All diagram types with auto dark mode
- ↶ **Undo/Redo**: Full history management with keyboard shortcuts

**Auto-naming Convention**
- New sections inherit parent name + date (YYYY-MM-DD)
- New pages inherit section name + date
- Imported documents automatically get date stamps

**User Interface**
- Three-panel layout: Sections | Pages List | Editor + Preview
- Breadcrumb navigation
- Metadata panel with timestamps
- Dark mode support

### 📝 Reminders System (Secondary Feature)

**macOS-Style Interface**
- Three-column layout: Smart Lists | Reminders | Detail Panel
- Complete dark mode support
- Smooth animations and hover effects

**Core Features**
- Smart lists: Today, Scheduled, Flagged, All
- Custom categories: Work, Personal, Shopping, Health, etc.
- Quick-add with auto-time for Today list
- Repeat frequency options
- Priority levels and flags
- Real-time notifications
- Evidence attachments

## 🚀 Quick Start

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

## 🎯 Usage Guide

### Adding Reminders

#### In Today List (Fastest)
1. Click "📅 Today" in sidebar
2. Type task title in top input
3. Press **Enter** or click away
4. ✨ Time automatically set to now!

#### In Other Lists
1. Select any list (Scheduled, Work, Personal, etc.)
2. Type task title
3. Optionally set date/time in the picker that appears
4. Press Enter or blur to save

### Editing Reminders
1. **Double-click** any reminder item
2. Right panel slides in with full details
3. Edit any field:
   - Title, notes, date/time
   - Repeat frequency
   - Category/list
   - Flag status
4. Click "Save Changes" or close panel

### Quick Actions
- **Toggle done**: Click the circle checkbox
- **Flag/unflag**: Click 🚩 button (shows on hover)
- **Delete**: Click 🗑️ in detail panel

### Keyboard Shortcuts
- **Enter**: Quick add reminder
- **F12**: Open DevTools (when debug enabled)

## 🏗️ Architecture

```text
┌─────────────────────────────────────────────────────────┐
│                  Tauri Application                      │
│                                                         │
│  ┌──────────────┐   Tauri Events   ┌──────────────┐  │
│  │   Main UI    │◄─────────────────►│ Notification │  │
│  │  (App.vue)   │                   │   Window     │  │
│  └──────┬───────┘                   └──────┬───────┘  │
│         │                                   │          │
│         │         ┌──────────────┐         │          │
│         └────────►│ Rust Backend │◄────────┘          │
│                   │  (Commands)  │                    │
│                   └──────┬───────┘                    │
│                          │                            │
│                   ┌──────▼───────┐                    │
│                   │    SQLite    │                    │
│                   │   Database   │                    │
│                   └──────────────┘                    │
└─────────────────────────────────────────────────────────┘

Event Flow:
1. User action → Tauri Command → Database update
2. Database update → Emit "reminders-updated" event
3. All windows listen → Auto-refresh UI
```

## 🧩 Components

### Frontend (Vue.js 3 + TypeScript)

**Main Window (App.vue)**
- Three-column layout (Sidebar | Content | Detail Panel)
- Smart lists + Custom categories
- Real-time filtering and statistics
- Inline quick-add form
- Detail edit panel (slides in on double-click)
- Event listener for real-time updates

**Key Features:**
- Composition API with `<script setup>`
- Reactive refs and computed properties
- Tauri invoke for backend commands
- Event system for cross-window sync

### Backend (Rust + Tauri 2.0)

**Tauri Commands (11 total):**
```rust
add_reminder()        // Create new reminder
get_reminders()       // Fetch all reminders
get_due_reminders()   // Fetch overdue/due reminders
toggle_reminder()     // Toggle completion status
delete_reminder()     // Remove reminder
update_reminder()     // Edit reminder details ⭐
broadcast_reminders() // Sync to all windows
set_debug_mode()      // Enable/disable debug logs
get_debug_mode()      // Check debug status
dismiss_notification()// Close notification
snooze_reminder()     // Postpone reminder
```

**Services:**
- Database initialization and migrations
- Notification checker (30s interval)
- System tray management
- Window lifecycle handlers
- Event broadcasting system

### Notification Window (notification.html)

**Standalone Window:**
- Pure HTML/CSS/JavaScript (no framework)
- Uses Tauri global API (`window.__TAURI__`)
- Positioned top-right corner
- Modal-style overlay
- Auto-loads on due reminders

**Features:**
- Real-time event listening
- Active data fetching on load
- Auto-close when empty (1.5s delay)
- Snooze/Complete actions
- Debug shortcuts (D, R, C, F12)

## 🔄 Data Flow

### Write Operations
```
User Action → Tauri Command → SQLite Update → Emit Event → All Windows Refresh
```

### Real-time Sync
```javascript
// Backend broadcasts after any change
app.emit("reminders-updated", &reminders);

// Frontend listens in all windows
await listen<Reminder[]>('reminders-updated', (event) => {
  reminders.value = event.payload;
});
```

### Key Principles
1. **Local-First**: All operations write to local SQLite immediately
2. **Privacy-Focused**: All data stays on your device, no cloud sync
3. **Event-Driven**: UI updates automatically via Tauri events
4. **No Polling**: Event system eliminates need for data polling

## 💾 Database Schema

```sql
CREATE TABLE reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    time TEXT NOT NULL,              -- ISO 8601 datetime string
    completed INTEGER NOT NULL DEFAULT 0,  -- 0=false, 1=true
    category TEXT NOT NULL,          -- work, personal, shopping, etc.
    frequency TEXT NOT NULL,         -- once, daily, weekly, monthly, yearly
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
)
```

### Data Model (TypeScript)
```typescript
interface Reminder {
  id: number;
  title: string;
  description: string;
  time: string;           // ISO datetime
  completed: boolean;
  category: string;       // List assignment
  frequency: string;      // Repeat pattern
  priority: number;       // 0-3 (future use)
  flagged: boolean;       // Star/important flag (future use)
  tags: string[];         // Hashtags (future use)
}
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

## 📂 Project Structure

```text
tauri-vue-app/
├── src/                          # Vue.js Frontend
│   ├── App.vue                   # Main application (3-column layout)
│   ├── main.ts                   # Vue bootstrap + global error handlers
│   ├── assets/                   # Images, styles
│   └── vite-env.d.ts            # TypeScript declarations
│
├── src-tauri/                    # Rust Backend
│   ├── src/
│   │   ├── main.rs              # Entry point
│   │   ├── lib.rs               # App setup, command registration
│   │   ├── commands/
│   │   │   └── mod.rs           # All Tauri commands (11 functions)
│   │   ├── database/
│   │   │   ├── mod.rs           # Database module exports
│   │   │   ├── init.rs          # DB initialization & migrations
│   │   │   └── operations.rs   # CRUD operations
│   │   ├── models/
│   │   │   ├── mod.rs           # Model exports
│   │   │   └── reminder.rs     # Reminder struct
│   │   ├── notifications/
│   │   │   └── mod.rs           # Notification service & window
│   │   └── tray/
│   │       └── mod.rs           # System tray setup
│   ├── Cargo.toml               # Rust dependencies
│   ├── tauri.conf.json          # Tauri configuration
│   ├── build.rs                 # Build script
│   └── icons/                   # App icons
│
├── notification.html             # Notification window (standalone)
├── index.html                    # Main window HTML
├── package.json                  # Node dependencies
├── vite.config.ts               # Vite configuration
├── tsconfig.json                # TypeScript config
└── README.md                     # This file
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

## 🛠️ Technologies

### Frontend Stack
- **Vue.js 3**: Composition API with `<script setup>`
- **TypeScript**: Type safety and IntelliSense
- **Vite**: Fast build tool and dev server
- **CSS**: Custom styling (no UI framework)

### Backend Stack
- **Rust**: High-performance native backend
- **Tauri 2.0**: Modern desktop app framework
- **SQLite**: Embedded database
- **SQLx**: Async SQL query executor
- **Tokio**: Async runtime
- **Chrono**: Date/time handling
- **Log/Env_logger**: Structured logging

### Design System
- **macOS Reminders inspired**: Visual language and interactions
- **SF Pro font style**: Apple's system font aesthetic
- **Color palette**: 
  - Primary: `#007aff` / `#0a84ff` (light/dark)
  - Text: `#1d1d1f` / `#f5f5f7`
  - Border: `#e5e5ea` / `#38383a`
- **Dark mode**: Complete theme with auto-detection

### Removed Technologies
- ~~Go backend server~~
- ~~WebSocket (tokio-tungstenite, gorilla/websocket)~~
- ~~HTTP server~~

*Replaced with Tauri's built-in event system for simpler, more efficient communication.*

## License

MIT

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## 🎯 Roadmap / Future Features

- [ ] **Search**: Full-text search across all reminders
- [ ] **Subtasks**: Nested todo items within reminders
- [ ] **Tags**: Hashtag support for flexible organization
- [ ] **Priority**: Visual priority indicators (!, !!, !!!)
- [ ] **Attachments**: Link files or images to reminders
- [ ] **Natural language input**: "Remind me tomorrow at 3pm"
- [ ] **Calendar view**: Month/week view of scheduled items
- [ ] **Recurring reminders**: Smart scheduling (weekdays, etc.)
- [ ] **Export/Import**: JSON/CSV data portability
- [ ] **Templates**: Quick-add from predefined templates
- [ ] **Widgets**: Desktop widgets for quick overview

## 🐛 Known Issues

- [ ] Timezone handling needs improvement
- [ ] No data migration tool yet
- [ ] Notification window position fixed (not draggable)

## 💡 Tips & Tricks

1. **Quick Today Entry**: Select Today list, type, press Enter - done!
2. **Flag Important Items**: Double-click → Toggle flag → Filters to Flagged list
3. **Keyboard Navigation**: Tab through fields in detail panel
4. **Debug Mode**: Enable in sidebar to see all events in console
5. **Empty Lists**: Use custom categories to organize work/life balance

## 🙏 Acknowledgments

- Inspired by **Apple's macOS Reminders** application
- Built with **Tauri** - The future of desktop apps
- UI design follows **Apple Human Interface Guidelines**

## 📝 Changelog

### v0.2.0 (Current)
- ✨ Redesigned UI to match macOS Reminders
- ✨ Added Today list with auto-time feature
- ✨ Double-click to edit in detail panel
- ✨ Smart lists (Today, Scheduled, Flagged, All)
- ✨ Flag/star reminders
- ✨ Complete dark mode support
- 🔄 Replaced WebSocket with Tauri events
- 🗑️ Removed Go backend dependency
- 🗑️ Removed category column from main view

### v0.1.0
- Initial release
- Basic CRUD operations
- Notification system
- System tray integration
- SQLite storage

## 📧 Support

For issues and questions:

- **Enable debug mode** first and check console logs
- Verify database file exists and is accessible
- Check system tray for application status
- Open DevTools (F12) to inspect errors
- Review this README for common solutions

**Database location:**
- macOS: `~/Library/Application Support/com.yaozhuang.tauri-vue-app/reminders.db`
- Linux: `~/.local/share/com.yaozhuang.tauri-vue-app/reminders.db`
- Windows: `%APPDATA%\com.yaozhuang.tauri-vue-app\reminders.db`
