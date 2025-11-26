# 🧰 Toolbox - Productivity Desktop App

A powerful multi-tool desktop productivity application built with Tauri 2, Vue.js 3, and Rust with local SQLite storage. Combines a full-featured Reminders app with a Wiki/Knowledge Base system.

> **Toolbox 工具箱** - 桌面生产力应用，集成提醒应用和知识库系统

## ✨ Features Overview | 功能概述

### 📚 Wiki / Knowledge Base | 知识库
- **Markdown editor**: Full Markdown support with live preview
- **WYSIWYG mode**: Rich text editing mode
- **Diagram support**: Mermaid and PlantUML diagrams
- **Code highlighting**: Syntax highlighting via highlight.js
- **Sections/Notebooks**: Organize pages into hierarchical sections
- **Version history**: Automatic revision tracking with restore capability
- **Tags**: Flexible tag-based organization
- **Search**: Full-text search across all pages
- **Document import**: Import external documents (Word, etc.)
- **Table insertion**: Easy table creation via modal
- **Reminder integration**: Insert reminder references into wiki pages

### 📝 Reminders App | 提醒应用
- **Three-column layout**: Smart Lists | Reminders | Detail Panel
- **macOS design language**: SF Pro font style, Apple color system, rounded corners
- **Complete dark mode support**: Auto-follows system preferences
- **Smooth animations**: Slide-in panels, checkbox animations, hover effects

#### 📋 Smart Lists
- **📅 Today**: Quick-add reminders for today (auto-sets time to now)
- **📆 Scheduled**: All reminders with due dates
- **🚩 Flagged**: Important/starred reminders
- **📋 All**: Complete overview of all reminders

#### 📁 User Lists (Custom Categories)
- 💼 Work | 👤 Personal | 🛒 Shopping | 🏥 Health | 📌 Other
- ➕ Add custom categories dynamically

#### ⚡ Quick Actions
- **Fast add**: Type title + Enter in any list
- **Today list special**: No time picker needed, auto-sets to current time
- **Single-click**: Toggle completion status
- **Double-click**: Open detail editing panel
- **Hover**: Show action buttons (flag, delete)

#### 📝 Reminder Properties
- Title and multi-line notes
- Due date and time
- Repeat frequency: Once, Daily, Weekly, Monthly, Yearly
- Category/List assignment
- Flag/Star for importance
- **File attachments**: Attach documents, images, and files to reminders

#### 📎 Evidence/Attachments System | 附件系统
- Upload files to reminders (images, documents, etc.)
- Automatic file type detection
- Image preview modal
- File size display
- Open attached files with system default application

### 🔔 Notification System
- Independent notification window
- Checks for due reminders every 30 seconds
- Quick actions: Complete or Snooze
- Real-time sync with main window

### 🔄 Real-time Sync
- **Event-driven architecture**: Tauri Event System
- All windows sync automatically via `reminders-updated` event
- No external servers or WebSocket needed
- Instant updates across all views

### ☁️ GitHub Sync
- **Cloud Backup**: Sync reminders to GitHub for safekeeping
- **Two Methods**: GitHub Gist or Repository JSON file
- **Security**: Token stored locally
- **Manual & Auto Sync**: One-click sync or automatic periodic sync
- See [GitHub Sync Guide](GITHUB_SYNC_GUIDE.md) for setup instructions

### 🛠️ Debug Features
- Toggle in sidebar (🐛 Debug Logs)
- Press F12 for DevTools
- Console logging with prefixes: `[APP]`, `[NOTIFICATION]`
- Real-time connection status

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

### GitHub Sync Setup
1. Click "⚙️ Sync Settings" in sidebar
2. Enable sync and select GitHub as data source
3. Generate GitHub token at [Settings → Tokens](https://github.com/settings/tokens/new)
4. Paste token and choose sync method (Gist or Repository)
5. Click "Test Connection" to verify
6. Use "⬆️ Sync to GitHub" to backup your reminders
7. See [GitHub Sync Guide](GITHUB_SYNC_GUIDE.md) for detailed instructions

### Keyboard Shortcuts

#### Wiki Editor
- **Ctrl/Cmd + B**: Bold text
- **Ctrl/Cmd + I**: Italic text  
- **Ctrl/Cmd + K**: Insert link
- **Ctrl/Cmd + Shift + T**: Insert table
- **Ctrl/Cmd + Shift + R**: Insert reminder
- **Ctrl/Cmd + Z**: Undo
- **Ctrl/Cmd + Shift + Z**: Redo
- **Ctrl/Cmd + Y**: Redo (alternative)
- **Tab**: Indent

#### Reminders
- **Enter**: Quick add reminder

#### General
- **F12**: Open DevTools

## 🏗️ Architecture | 架构

```text
┌───────────────────────────────────────────────────────────────────────┐
│                        Tauri Application                               │
│                                                                        │
│  ┌────────────────────────────────────────────────────────────────┐  │
│  │                    Vue.js 3 Frontend                            │  │
│  │                                                                  │  │
│  │  ┌────────────┐     ┌──────────────┐     ┌─────────────────┐   │  │
│  │  │  WikiApp   │────►│ RemindersApp │     │ Notification    │   │  │
│  │  │  (Main)    │     │  (Drawer)    │     │ Window          │   │  │
│  │  └─────┬──────┘     └──────┬───────┘     └───────┬─────────┘   │  │
│  │        │                   │                     │              │  │
│  │        └───────────────────┼─────────────────────┘              │  │
│  │                            │                                    │  │
│  │                   Tauri IPC (invoke/emit/listen)               │  │
│  └────────────────────────────┼────────────────────────────────────┘  │
│                               │                                        │
│  ┌────────────────────────────┴────────────────────────────────────┐  │
│  │                      Rust Backend                                │  │
│  │                                                                  │  │
│  │  ┌────────────┐  ┌──────────────┐  ┌────────────┐              │  │
│  │  │ Reminder   │  │    Wiki      │  │  Evidence  │              │  │
│  │  │ Commands   │  │  Commands    │  │  Commands  │              │  │
│  │  │  (9 cmds)  │  │  (12 cmds)   │  │ (10 cmds)  │              │  │
│  │  └─────┬──────┘  └──────┬───────┘  └─────┬──────┘              │  │
│  │        │                │                │                      │  │
│  │        └────────────────┼────────────────┘                      │  │
│  │                         │                                        │  │
│  │  ┌──────────────────────┴───────────────────────────────────┐  │  │
│  │  │                   Data Storage                             │  │  │
│  │  │                                                            │  │  │
│  │  │  SQLite Database              File System                  │  │  │
│  │  │  (reminders, evidence)        (wiki pages, revisions)      │  │  │
│  │  └────────────────────────────────────────────────────────────┘  │  │
│  │                                                                  │  │
│  │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐    │  │
│  │  │ Sync (GitHub│  │ Notification│  │ System Tray         │    │  │
│  │  │ Gist/Repo)  │  │ Service     │  │ (Desktop only)      │    │  │
│  │  └─────────────┘  └─────────────┘  └─────────────────────┘    │  │
│  └──────────────────────────────────────────────────────────────────┘  │
└───────────────────────────────────────────────────────────────────────┘

Screen Navigation Flow:
┌─────────────────────────────────────────────────────────────────────┐
│                                                                      │
│   AppSimple.vue (Entry Point)                                       │
│         │                                                            │
│         ▼                                                            │
│   ┌─────────────────────────────────────────────────────────────┐   │
│   │                     WikiApp.vue                              │   │
│   │  ┌────────────┐ ┌──────────────────┐ ┌────────────────────┐ │   │
│   │  │ WikiSidebar│ │   WikiEditor     │ │   WikiMetadata     │ │   │
│   │  │ - Sections │ │   + WikiPreview  │ │   - Tags           │ │   │
│   │  │ - Pages    │ │                  │ │   - Revisions      │ │   │
│   │  └────────────┘ └──────────────────┘ └────────────────────┘ │   │
│   │                                                              │   │
│   │  [📝 Reminders] Button opens:                               │   │
│   │  ┌──────────────────────────────────────────────────────┐   │   │
│   │  │              Reminders Drawer (Overlay)               │   │   │
│   │  │              RemindersApp.vue                         │   │   │
│   │  └──────────────────────────────────────────────────────┘   │   │
│   └─────────────────────────────────────────────────────────────┘   │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### Event Flow | 事件流
1. User action → Tauri Command → Database/File update
2. Database update → Emit event (e.g., `reminders-updated`)
3. All windows listen → Auto-refresh UI

## 🧩 Components | 组件

### Frontend (Vue.js 3 + TypeScript) | 前端

**WikiApp.vue (Main Container)**
- Wiki editor with Markdown/WYSIWYG modes
- Section-based organization
- Page revision history
- Tag management
- Integrated Reminders panel (drawer)

**RemindersApp.vue**
- Three-column layout (Sidebar | Content | Detail Panel)
- Smart lists + Custom categories
- Evidence/attachment management
- Real-time sync across windows

**Key Frontend Components:**
- `WikiEditor.vue` - Markdown/WYSIWYG editor
- `WikiPreview.vue` - Live Markdown preview
- `WikiSidebar.vue` - Navigation and sections
- `WikiMetadata.vue` - Tags and revisions
- `DocumentImportModal.vue` - Import external documents
- `TableInsertModal.vue` - Table creation wizard
- `ReminderInsertModal.vue` - Insert reminders into wiki

### Backend (Rust + Tauri 2.0) | 后端

**Tauri Commands:**

```rust
// Reminders (9 commands)
add_reminder(), get_reminders(), update_reminder()
delete_reminder(), toggle_reminder()
get_due_reminders(), broadcast_reminders()
set_debug_mode(), get_debug_mode()

// Notifications
snooze_reminder(), dismiss_notification()

// Wiki (12 commands)
create_wiki_page(), update_wiki_page(), get_wiki_page()
list_wiki_pages(), delete_wiki_page(), search_wiki_pages()
list_wiki_revisions(), restore_wiki_revision()
list_sections(), create_section()
update_section(), delete_section()

// Evidence (10 commands)
add_evidence_to_reminder(), get_reminder_evidence()
get_all_evidence_items(), update_evidence_desc()
delete_evidence_item(), save_uploaded_file()
get_evidence_file_path(), open_evidence_file()
get_mime_type(), format_file_size()

// Sync (5 commands)
get_sync_settings(), save_sync_settings()
test_github_connection()
sync_to_github(), sync_from_github()
```

**Total: 38 Tauri Commands**

## 🔄 Data Flow | 数据流

### Write Operations | 写操作
```
User Action → Tauri Command → SQLite/File Update → Emit Event → All Windows Refresh
```

### Real-time Sync | 实时同步
```javascript
// Backend broadcasts after any change
app.emit("reminders-updated", &reminders);

// Frontend listens in all windows
await listen<Reminder[]>('reminders-updated', (event) => {
  reminders.value = event.payload;
});
```

### Key Principles | 关键原则
1. **Local-First**: All operations write to local storage immediately | 本地优先
2. **Privacy-Focused**: All data stays on your device | 隐私优先
3. **Event-Driven**: UI updates automatically via Tauri events | 事件驱动
4. **No Polling**: Event system eliminates need for data polling | 无需轮询

## 💾 Data Storage | 数据存储

### SQLite Database (Reminders & Evidence)
```sql
CREATE TABLE reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    time TEXT NOT NULL,              -- ISO 8601 datetime string
    completed INTEGER NOT NULL DEFAULT 0,
    category TEXT NOT NULL,
    frequency TEXT NOT NULL,
    flagged INTEGER NOT NULL DEFAULT 0,
    priority INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE evidence (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    reminder_id INTEGER NOT NULL,
    file_type TEXT NOT NULL,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type TEXT,
    thumbnail_path TEXT,
    description TEXT,
    metadata TEXT,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (reminder_id) REFERENCES reminders(id) ON DELETE CASCADE
);
```

### File System Storage (Wiki)
```
{APP_DATA_DIR}/wiki/
├── pages/
│   ├── {page-id-1}.json       # Page data with content
│   ├── {page-id-2}.json
│   └── ...
├── revisions/
│   ├── {page-id-1}/
│   │   ├── {revision-id-1}.json
│   │   └── ...
│   └── ...
└── sections.json               # Section hierarchy
```

### Data Models (TypeScript)
```typescript
interface Reminder {
  id: number;
  title: string;
  description: string;
  time: string;
  completed: boolean;
  category: string;
  frequency: string;
  priority: number;
  flagged: boolean;
  tags: string[];
}

interface WikiPage {
  id: string;
  title: string;
  content: string;
  tags: string[];
  notebook: string;
  section: string;
  section_id: string | null;
  created_at: number;
  updated_at: number;
}

interface Evidence {
  id: number;
  reminder_id: number;
  file_type: string;
  file_path: string;
  file_name: string;
  file_size: number;
  mime_type: string;
  thumbnail_path: string | null;
  description: string | null;
  created_at: string;
}
```

## Configuration | 配置

### Debug Logging
```bash
RUST_LOG=debug npm run tauri dev
```

Or programmatically:
```javascript
await invoke('set_debug_mode', { enabled: true });
```

### Data Location | 数据存储位置
- **macOS**: `~/Library/Application Support/com.yaozhuang.tauri-vue-app/`
- **Linux**: `~/.local/share/com.yaozhuang.tauri-vue-app/`
- **Windows**: `%APPDATA%\com.yaozhuang.tauri-vue-app\`

## 📂 Project Structure | 项目结构

```text
toolbox/
├── src/                          # Vue.js Frontend
│   ├── AppSimple.vue             # Entry wrapper (renders WikiApp)
│   ├── App.vue                   # Legacy standalone reminders app
│   ├── main.ts                   # Vue bootstrap + global error handlers
│   ├── components/
│   │   ├── WikiApp.vue           # Main wiki container + reminders drawer
│   │   ├── WikiEditor.vue        # Markdown/WYSIWYG editor
│   │   ├── WikiPreview.vue       # Live Markdown preview
│   │   ├── WikiSidebar.vue       # Navigation and sections
│   │   ├── WikiMetadata.vue      # Tags and revision history
│   │   ├── RemindersApp.vue      # Full reminders application
│   │   ├── DocumentImportModal.vue
│   │   ├── TableInsertModal.vue
│   │   ├── ReminderInsertModal.vue
│   │   ├── ContextMenu.vue
│   │   └── SectionNode.vue
│   ├── composables/
│   │   └── useWikiStore.ts       # Wiki state management
│   ├── types/                    # TypeScript type definitions
│   └── utils/                    # Utility functions
│
├── src-tauri/                    # Rust Backend
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── lib.rs                # App setup, command registration
│   │   ├── commands/mod.rs       # Reminder commands
│   │   ├── wiki_commands.rs      # Wiki & section commands
│   │   ├── evidence_commands.rs  # Evidence/attachment commands
│   │   ├── sync_commands.rs      # GitHub sync commands
│   │   ├── database/
│   │   │   ├── mod.rs            # Database module exports
│   │   │   ├── init.rs           # DB initialization & migrations
│   │   │   └── operations.rs     # CRUD operations
│   │   ├── models/               # Data models
│   │   ├── notifications/        # Notification service
│   │   ├── sync/                 # GitHub sync logic
│   │   └── tray/                 # System tray (desktop only)
│   ├── Cargo.toml                # Rust dependencies
│   └── tauri.conf.json           # Tauri configuration
│
├── notification.html             # Notification window (standalone)
├── index.html                    # Main window HTML
├── package.json                  # Node dependencies
├── vite.config.ts                # Vite configuration
├── tsconfig.json                 # TypeScript config
├── README.md                     # This file
└── SOFTWARE_ARCHITECTURE.md      # Detailed architecture docs
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

## 🛠️ Technologies | 技术栈

### Frontend Stack | 前端技术栈
- **Vue.js 3**: Composition API with `<script setup>`
- **TypeScript**: Type safety and IntelliSense
- **Vite**: Fast build tool and dev server
- **Marked**: Markdown parsing
- **Highlight.js**: Code syntax highlighting
- **Mermaid**: Diagram rendering
- **PlantUML**: UML diagram support
- **CSS**: Custom styling with dark mode support

### Backend Stack | 后端技术栈
- **Rust**: High-performance native backend
- **Tauri 2.0**: Modern desktop app framework
- **SQLite**: Embedded database
- **SQLx**: Async SQL query executor
- **Tokio**: Async runtime
- **Chrono**: Date/time handling
- **Serde**: Serialization/deserialization
- **Log/Env_logger**: Structured logging

### Design System | 设计系统
- **macOS Reminders inspired**: Visual language and interactions
- **Apple-style colors**: 
  - Primary: `#007aff` / `#0a84ff` (light/dark)
  - Text: `#1d1d1f` / `#f5f5f7`
- **Dark mode**: Complete theme with auto-detection

## License

MIT

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'Add amazing feature'`)
4. Push to branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## 🎯 Roadmap / Future Features | 未来计划

### Wiki Enhancements
- [ ] Full-text search with FTS5
- [ ] Bi-directional links between pages
- [ ] Graph view of connections
- [ ] Export to PDF/HTML
- [ ] Image embedding and management

### Reminders Enhancements
- [ ] Natural language input ("tomorrow at 3pm")
- [ ] Calendar view (month/week)
- [ ] Subtasks / Nested items
- [ ] Templates for common reminders
- [ ] Export/Import (JSON/CSV)

### General
- [ ] Multi-language support (i18n)
- [ ] Theme customization
- [ ] Desktop widgets
- [ ] Mobile app improvements

## 🐛 Known Issues

- [ ] Timezone handling needs improvement
- [ ] No data migration tool yet
- [ ] Notification window position fixed (not draggable)

## 💡 Tips & Tricks | 使用技巧

1. **Quick Today Entry**: Select Today list, type, press Enter - done!
2. **Flag Important Items**: Double-click → Toggle flag → Filters to Flagged list
3. **Keyboard Navigation**: Tab through fields in detail panel
4. **Debug Mode**: Enable in sidebar to see all events in console
5. **Wiki from Reminders**: Use the drawer to quickly access reminders while editing wiki

## 🙏 Acknowledgments

- Inspired by **Apple's macOS Reminders** and **Notes** applications
- Built with **Tauri** - The future of desktop apps
- UI design follows **Apple Human Interface Guidelines**

## 📝 Changelog | 更新日志

### v0.3.0 (Current)
- ✨ Added Wiki/Knowledge Base with Markdown support
- ✨ WYSIWYG editor mode
- ✨ Mermaid and PlantUML diagram support
- ✨ Section-based organization for wiki pages
- ✨ Revision history and restore functionality
- ✨ Evidence/attachment system for reminders
- ✨ Document import (Word, etc.)
- ✨ Integrated Reminders drawer in Wiki
- 🔄 Restructured app with WikiApp as main entry point

### v0.2.0
- ✨ Redesigned UI to match macOS Reminders
- ✨ Added Today list with auto-time feature
- ✨ Double-click to edit in detail panel
- ✨ Smart lists (Today, Scheduled, Flagged, All)
- ✨ Flag/star reminders
- ✨ Complete dark mode support
- 🔄 Replaced WebSocket with Tauri events
- 🗑️ Removed Go backend dependency

### v0.1.0
- Initial release
- Basic CRUD operations
- Notification system
- System tray integration
- SQLite storage

## 📧 Support | 支持

For issues and questions:
- **Enable debug mode** first and check console logs
- Verify data files exist and are accessible
- Check system tray for application status
- Open DevTools (F12) to inspect errors

**Data location | 数据位置:**
- macOS: `~/Library/Application Support/com.yaozhuang.tauri-vue-app/`
- Linux: `~/.local/share/com.yaozhuang.tauri-vue-app/`
- Windows: `%APPDATA%\com.yaozhuang.tauri-vue-app\`
