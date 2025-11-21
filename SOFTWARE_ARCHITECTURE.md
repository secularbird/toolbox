# 🏗️ Software Architecture Documentation
# 软件架构文档

> **Toolbox Multi-Tool Desktop Application**  
> Built with Tauri 2, Vue.js 3, Rust, and SQLite

---

## 📋 Table of Contents | 目录

1. [System Overview | 系统概述](#system-overview--系统概述)
2. [Architecture Layers | 架构层次](#architecture-layers--架构层次)
3. [Technology Stack | 技术栈](#technology-stack--技术栈)
4. [Frontend Architecture | 前端架构](#frontend-architecture--前端架构)
5. [Backend Architecture | 后端架构](#backend-architecture--后端架构)
6. [Data Storage | 数据存储](#data-storage--数据存储)
7. [Inter-Process Communication | 进程间通信](#inter-process-communication--进程间通信)
8. [Module Interactions | 模块交互](#module-interactions--模块交互)
9. [Security Considerations | 安全考虑](#security-considerations--安全考虑)
10. [Performance Optimizations | 性能优化](#performance-optimizations--性能优化)
11. [Deployment Architecture | 部署架构](#deployment-architecture--部署架构)
12. [Future Enhancements | 未来增强](#future-enhancements--未来增强)

---

## System Overview | 系统概述

### Application Purpose | 应用目的

Toolbox is a unified desktop application that combines three essential productivity tools:
- **Reminders**: Task and reminder management with notifications
- **Wiki**: Personal knowledge base with markdown support
- **Disk Scanner**: Storage analysis and visualization tool

Toolbox 是一个统一的桌面应用程序，整合了三个核心生产力工具：
- **提醒应用**：任务和提醒管理，带通知功能
- **知识库**：支持 Markdown 的个人知识管理系统
- **磁盘扫描器**：存储分析和可视化工具

### Design Principles | 设计原则

1. **Local-First**: All data stored locally, no cloud dependency
2. **Privacy-Focused**: No data collection or external communication
3. **Native Performance**: Rust backend for optimal speed
4. **Cross-Platform**: Supports Windows, macOS, Linux, and Android
5. **Event-Driven**: Real-time updates across all windows
6. **Modular Design**: Independent features with shared infrastructure

1. **本地优先**：所有数据本地存储，无云依赖
2. **隐私保护**：无数据收集或外部通信
3. **原生性能**：Rust 后端提供最佳速度
4. **跨平台**：支持 Windows、macOS、Linux 和 Android
5. **事件驱动**：所有窗口实时更新
6. **模块化设计**：独立功能与共享基础设施

---

## Architecture Layers | 架构层次

### Layer Diagram | 层次图

```
┌───────────────────────────────────────────────────────────────┐
│                     Presentation Layer                        │
│          (Vue 3 Components + HTML/CSS/TypeScript)             │
├───────────────────────────────────────────────────────────────┤
│  RemindersApp  │    WikiApp     │    DiskScanner             │
│  + Components  │  + Components  │    + Components            │
└────────┬──────────────┬───────────────────┬───────────────────┘
         │              │                   │
         │    Tauri IPC (invoke/emit/listen)                    
         │              │                   │
┌────────┴──────────────┴───────────────────┴───────────────────┐
│                   Application Layer (Rust)                    │
│                      Tauri Commands API                       │
├───────────────────────────────────────────────────────────────┤
│  Reminder     │    Wiki         │   Disk Scanner             │
│  Commands     │    Commands     │   Commands                 │
│  (11 cmds)    │    (8 cmds)     │   (4 cmds)                 │
└────────┬──────────────┬───────────────────┬───────────────────┘
         │              │                   │
┌────────┴──────────────┴───────────────────┴───────────────────┐
│                    Domain/Business Logic                      │
│                  (Models + Operations)                        │
├───────────────────────────────────────────────────────────────┤
│  Reminder      │   WikiPage      │   DiskItem                │
│  Operations    │   File Ops      │   Filesystem Ops          │
│  + Validation  │   + Revisions   │   + Size Calc             │
└────────┬──────────────┬───────────────────┬───────────────────┘
         │              │                   │
┌────────┴──────────────┴───────────────────┴───────────────────┐
│                 Infrastructure Layer                          │
├───────────────────────────────────────────────────────────────┤
│  SQLite DB     │  File System    │  System APIs              │
│  (SQLx)        │  (std::fs)      │  (dirs, OS)               │
│                │                 │                             │
│  Event Bus     │  Timers         │  Tray Icon                │
│  (Tauri)       │  (Tokio)        │  (Desktop only)           │
└───────────────────────────────────────────────────────────────┘
```

### Layer Responsibilities | 层次职责

#### Presentation Layer | 表示层
- User interface rendering
- User input handling
- State management (reactive refs/computed)
- Component composition
- Visual feedback and animations

#### Application Layer | 应用层
- Command handlers (Tauri commands)
- Request validation
- Business rules enforcement
- Event emission
- Transaction coordination

#### Domain Layer | 领域层
- Core business entities (Reminder, WikiPage, DiskItem)
- Business logic operations (CRUD, search, calculations)
- Data validation rules
- Domain events

#### Infrastructure Layer | 基础设施层
- Data persistence (SQLite, filesystem)
- External system integration
- Logging and monitoring
- Platform-specific features

---

## Technology Stack | 技术栈

### Frontend Stack | 前端技术栈

| Technology | Version | Purpose | 用途 |
|------------|---------|---------|------|
| **Vue.js** | 3.5.13 | UI framework | 用户界面框架 |
| **TypeScript** | 5.6.2 | Type safety | 类型安全 |
| **Vite** | 6.4.1 | Build tool | 构建工具 |
| **Marked** | 17.0.0 | Markdown parsing | Markdown 解析 |
| **Highlight.js** | 11.11.1 | Code highlighting | 代码高亮 |
| **Tauri API** | 2.x | IPC client | IPC 客户端 |

### Backend Stack | 后端技术栈

| Technology | Version | Purpose | 用途 |
|------------|---------|---------|------|
| **Rust** | 2021 Edition | Native backend | 原生后端 |
| **Tauri** | 2.x | Desktop framework | 桌面框架 |
| **SQLx** | 0.8 | Database driver | 数据库驱动 |
| **Tokio** | 1.x | Async runtime | 异步运行时 |
| **Serde** | 1.x | Serialization | 序列化 |
| **Chrono** | 0.4 | Date/time handling | 日期时间处理 |
| **UUID** | 1.10 | ID generation | ID 生成 |
| **dirs** | 5.0 | Path resolution | 路径解析 |

### Database | 数据库

- **SQLite 3**: Embedded relational database
- **SQLx**: Type-safe SQL queries with compile-time verification
- **Migrations**: Automatic schema initialization

---

## Frontend Architecture | 前端架构

### Component Tree | 组件树

```
App.vue (or AppSimple.vue)
│
├── Navigation Tabs
│   ├─ 📝 Reminders
│   ├─ 📚 Wiki
│   └─ 💾 Disk Scanner
│
├── RemindersApp (when active)
│   ├── Sidebar
│   │   ├── SmartLists
│   │   ├── CustomCategories
│   │   └── DebugToggle
│   ├── ReminderList
│   │   └── ReminderItem (multiple)
│   └── DetailPanel (conditional)
│       ├── TitleEditor
│       ├── NotesEditor
│       ├── DateTimePicker
│       ├── CategorySelector
│       ├── EvidenceList
│       └── ActionButtons
│
├── WikiApp (when active)
│   ├── WikiSidebar
│   │   ├── NotebookFilter
│   │   ├── TagFilter
│   │   └── PageList
│   ├── WikiEditor
│   │   ├── ToolBar
│   │   └── MarkdownTextArea
│   ├── WikiPreview
│   │   └── RenderedMarkdown
│   └── WikiMetadata
│       ├── TitleInput
│       ├── TagsInput
│       ├── NotebookSelector
│       └── RevisionHistory
│
└── DiskScanner (when active)
    ├── InputArea
    │   ├── PathInput
    │   ├── QuickAccessButtons
    │   └── DepthControl
    └── ResultsArea
        └── DiskItem (recursive)
            ├── FileIcon
            ├── SizeBar
            └── Children (DiskItem[])
```

### State Management | 状态管理

#### Composition API Pattern | 组合式 API 模式

Each feature uses Vue 3 Composition API with dedicated composables:

```typescript
// Reminders State
const reminders = ref<Reminder[]>([]);
const activeList = ref<string>('today');
const editingReminder = ref<Reminder | null>(null);
const debugMode = ref<boolean>(false);

// Wiki State  
const {
  pages, currentPage, isLoading, error,
  loadPages, createPage, updatePage, deletePage,
  searchPages, listRevisions, restoreRevision
} = useWiki();

// Disk Scanner State
const path = ref<string>('');
const result = ref<DiskItem | null>(null);
const scanning = ref<boolean>(false);
const limitDepth = ref<boolean>(true);
const maxDepth = ref<number>(3);
```

#### Reactive Data Flow | 响应式数据流

```
User Action
    ↓
Event Handler
    ↓
Tauri Invoke (IPC call)
    ↓
Backend Processing
    ↓
Event Emission (broadcast)
    ↓
Event Listener
    ↓
State Update (ref.value = ...)
    ↓
Template Re-render (automatic)
```

### Styling Architecture | 样式架构

- **Scoped CSS**: Component-level styles with `<style scoped>`
- **CSS Custom Properties**: Theme colors and shared values
- **Responsive Design**: Flexbox and Grid layouts
- **Dark Mode**: Media query `prefers-color-scheme`
- **Animations**: CSS transitions and keyframes

---

## Backend Architecture | 后端架构

### Command Structure | 命令结构

#### Reminders Commands (11) | 提醒命令

```rust
// CRUD Operations
add_reminder(title, description, time, category, frequency)
get_reminders() -> Vec<Reminder>
update_reminder(id, updates)
delete_reminder(id)
toggle_reminder(id) -> Result<()>

// Special Queries
get_due_reminders() -> Vec<Reminder>
broadcast_reminders(app_handle) -> Result<()>

// Notification Actions
snooze_reminder(id, minutes) -> Result<()>
dismiss_notification(window) -> Result<()>

// Debug Controls
set_debug_mode(enabled: bool)
get_debug_mode() -> bool
```

#### Wiki Commands (8) | 知识库命令

```rust
create_wiki_page(title, content, tags, notebook, section)
update_wiki_page(id, updates)
get_wiki_page(id) -> WikiPage
list_wiki_pages() -> Vec<WikiPageList>
delete_wiki_page(id)
search_wiki_pages(query) -> Vec<WikiPageList>
list_wiki_revisions(page_id) -> Vec<WikiRevisionMeta>
restore_wiki_revision(page_id, revision_id)
```

#### Disk Scanner Commands (4) | 磁盘扫描命令

```rust
scan_directory(path, max_depth) -> DiskItem
get_home_directory() -> String
get_system_roots() -> Vec<String>
format_bytes(bytes) -> String
```

#### Evidence Commands (10) | 证据附件命令

```rust
add_evidence_to_reminder(reminder_id, file_info)
get_reminder_evidence(reminder_id) -> Vec<EvidenceItem>
get_all_evidence_items() -> Vec<EvidenceItem>
update_evidence_desc(id, description)
delete_evidence_item(id)
save_uploaded_file(file_data, filename, reminder_id) -> String
get_evidence_file_path(id) -> String
open_evidence_file(id)
get_mime_type(filename) -> String
format_file_size(bytes) -> String
```

### Service Layer | 服务层

#### Database Service | 数据库服务

```rust
// src-tauri/src/database/mod.rs
pub mod init;
pub mod operations;

pub async fn init_database(path: PathBuf) -> Result<SqlitePool> {
    // Create database connection pool
    // Run migrations
    // Return managed pool
}
```

#### Notification Service | 通知服务

```rust
// src-tauri/src/notifications/mod.rs
pub async fn start_notification_service(
    pool: SqlitePool,
    app_handle: AppHandle
) {
    // Spawn background task
    // Check for due reminders every 30 seconds
    // Emit events to trigger notifications
}
```

#### Tray Service | 系统托盘服务 (Desktop Only)

```rust
// src-tauri/src/tray/mod.rs
pub fn setup_tray(app: &AppHandle) -> Result<()> {
    // Create system tray icon
    // Setup menu items
    // Register event handlers
}
```

### Error Handling | 错误处理

```rust
// Consistent error type
type Result<T> = std::result::Result<T, String>;

// Error conversion
impl From<sqlx::Error> for String {
    fn from(err: sqlx::Error) -> String {
        format!("Database error: {}", err)
    }
}

// Command error handling
#[tauri::command]
pub async fn example_command() -> Result<Data> {
    let data = operation()
        .map_err(|e| format!("Operation failed: {}", e))?;
    Ok(data)
}
```

---

## Data Storage | 数据存储

### SQLite Database Schema | SQLite 数据库架构

#### Reminders Table | 提醒表

```sql
CREATE TABLE reminders (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    description TEXT NOT NULL DEFAULT '',
    time TEXT NOT NULL,                    -- ISO 8601
    completed INTEGER NOT NULL DEFAULT 0,  -- Boolean (0/1)
    category TEXT NOT NULL DEFAULT 'other',
    frequency TEXT NOT NULL DEFAULT 'once',
    flagged INTEGER NOT NULL DEFAULT 0,
    priority INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for performance
CREATE INDEX idx_reminders_time ON reminders(time);
CREATE INDEX idx_reminders_completed ON reminders(completed);
CREATE INDEX idx_reminders_category ON reminders(category);
```

#### Evidence Table | 附件表

```sql
CREATE TABLE evidence (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    reminder_id INTEGER NOT NULL,
    file_path TEXT NOT NULL,
    file_name TEXT NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type TEXT,
    description TEXT DEFAULT '',
    created_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (reminder_id) REFERENCES reminders(id) ON DELETE CASCADE
);

CREATE INDEX idx_evidence_reminder ON evidence(reminder_id);
```

### File System Storage | 文件系统存储

#### Wiki Pages Structure | Wiki 页面结构

```
{APP_DATA_DIR}/wiki/
├── pages/
│   ├── {page-id-1}.json       # Page data
│   ├── {page-id-2}.json
│   └── ...
└── revisions/
    ├── {page-id-1}/
    │   ├── {revision-id-1}.json
    │   ├── {revision-id-2}.json
    │   └── ...
    └── {page-id-2}/
        └── ...
```

#### Evidence Files Structure | 附件文件结构

```
{APP_DATA_DIR}/evidence/
├── reminder_{id}/
│   ├── file1.pdf
│   ├── image.png
│   └── document.docx
└── ...
```

#### Platform-Specific Paths | 平台特定路径

- **macOS**: `~/Library/Application Support/com.yaozhuang.tauri-vue-app/`
- **Linux**: `~/.local/share/com.yaozhuang.tauri-vue-app/`
- **Windows**: `%APPDATA%\com.yaozhuang.tauri-vue-app\`
- **Android**: App-specific sandbox directory

---

## Inter-Process Communication | 进程间通信

### IPC Patterns | IPC 模式

#### Command Pattern (Request-Response) | 命令模式（请求-响应）

```typescript
// Frontend: Send command
const result = await invoke<Reminder[]>('get_reminders');

// Backend: Handle command
#[tauri::command]
pub async fn get_reminders(
    pool: State<'_, SqlitePool>
) -> Result<Vec<Reminder>> {
    operations::get_all_reminders(&pool).await
}
```

#### Event Pattern (Pub-Sub) | 事件模式（发布-订阅）

```typescript
// Frontend: Subscribe to events
await listen<Reminder[]>('reminders-updated', (event) => {
    reminders.value = event.payload;
});

// Backend: Publish events
app_handle.emit("reminders-updated", &reminders)?;
```

### Event Bus Architecture | 事件总线架构

```
┌─────────────────────────────────────────────────────────┐
│                    Event Bus (Tauri)                    │
├─────────────────────────────────────────────────────────┤
│                                                         │
│  Events:                                                │
│  • reminders-updated   → Vec<Reminder>                  │
│  • wiki-page-updated   → WikiPage                       │
│  • debug-mode-changed  → bool                           │
│                                                         │
└─────────────────────────────────────────────────────────┘
         ↓                    ↓                    ↓
    Main Window      Notification Window    (Future Windows)
```

### Data Serialization | 数据序列化

- **Format**: JSON
- **Library**: Serde + Serde_json
- **Type Safety**: TypeScript interfaces match Rust structs

```rust
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reminder {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub time: String,
    pub completed: bool,
    pub category: String,
    pub frequency: String,
    pub flagged: bool,
    pub priority: i32,
}
```

```typescript
interface Reminder {
    id: number;
    title: string;
    description: string;
    time: string;
    completed: boolean;
    category: string;
    frequency: string;
    flagged: boolean;
    priority: number;
}
```

---

## Module Interactions | 模块交互

### Reminders Flow | 提醒应用流程

#### Adding a Reminder | 添加提醒

```
User Input (Title)
    ↓
[RemindersApp] Quick Add Input
    ↓
invoke('add_reminder', data)
    ↓
[Backend] add_reminder command
    ↓
[Database] INSERT INTO reminders
    ↓
[Backend] Fetch updated list
    ↓
emit('reminders-updated', reminders)
    ↓
[Frontend] listen() receives event
    ↓
Update reactive state
    ↓
UI re-renders automatically
```

#### Notification Flow | 通知流程

```
[Timer Service] Every 30 seconds
    ↓
Query due/overdue reminders
    ↓
If reminders found:
    ↓
Check if notification window exists
    ↓
If not: Create notification window
    ↓
emit('reminders-updated')
    ↓
[Notification Window] Receives event
    ↓
Display reminders in UI
    ↓
User clicks "Complete" or "Snooze"
    ↓
invoke('toggle_reminder' or 'snooze_reminder')
    ↓
Update database
    ↓
emit('reminders-updated')
    ↓
All windows sync
```

### Wiki Flow | 知识库流程

#### Creating a Page | 创建页面

```
User clicks "New Page"
    ↓
[WikiApp] Creates empty editor state
    ↓
User types content
    ↓
Auto-save timer triggers
    ↓
invoke('create_wiki_page', page_data)
    ↓
[Backend] Generate UUID
    ↓
Write page to filesystem
    ↓
Create initial revision
    ↓
Return page ID
    ↓
[Frontend] Update pages list
    ↓
Navigate to new page
```

#### Version Control Flow | 版本控制流程

```
User updates page
    ↓
invoke('update_wiki_page')
    ↓
[Backend] Read current page
    ↓
Save current version to revisions/
    ↓
Write updated page
    ↓
Update timestamps
    ↓
Return success
    ↓
[Frontend] Refresh revision list
```

### Disk Scanner Flow | 磁盘扫描流程

```
User enters path
    ↓
Clicks "Scan" button
    ↓
invoke('scan_directory', path, max_depth)
    ↓
[Backend] Recursive filesystem traversal
    ↓
Calculate sizes (bottom-up)
    ↓
Sort by size
    ↓
Return tree structure
    ↓
[Frontend] Receive DiskItem tree
    ↓
Render with DiskItem component (recursive)
    ↓
User clicks folder to expand
    ↓
Vue toggles expanded state
    ↓
Children render (lazy)
```

---

## Security Considerations | 安全考虑

### Tauri Security Model | Tauri 安全模型

1. **Command Whitelist**: Only registered commands are callable
2. **No Eval**: No dynamic code execution
3. **CSP Headers**: Content Security Policy enforced
4. **Protocol Restrictions**: Limited protocol access

### Data Security | 数据安全

1. **Local Storage Only**: No network transmission
2. **File Permissions**: Respect OS-level permissions
3. **SQL Injection Prevention**: SQLx prepared statements
4. **Path Traversal Protection**: Validate all file paths

### Privacy Protection | 隐私保护

1. **No Telemetry**: Zero data collection
2. **No External Connections**: Fully offline capable
3. **Local Database**: SQLite with no remote access
4. **User Data Control**: Export/delete functionality

### Code Security | 代码安全

```rust
// Example: Safe path handling
fn validate_path(path: &str) -> Result<PathBuf> {
    let path = PathBuf::from(path);
    
    // Prevent path traversal
    if path.to_str().unwrap_or("").contains("..") {
        return Err("Invalid path".to_string());
    }
    
    // Ensure path is absolute
    if !path.is_absolute() {
        return Err("Path must be absolute".to_string());
    }
    
    Ok(path)
}
```

---

## Performance Optimizations | 性能优化

### Frontend Optimizations | 前端优化

1. **Virtual Scrolling**: For large lists (planned)
2. **Lazy Loading**: Components loaded on demand
3. **Debounced Input**: Auto-save with debouncing
4. **Computed Properties**: Efficient reactive calculations
5. **Keep-Alive**: Cache component state when switching views

### Backend Optimizations | 后端优化

1. **Connection Pooling**: SQLx pool for database connections
2. **Async Operations**: Tokio runtime for non-blocking I/O
3. **Indexed Queries**: Database indexes on frequently queried columns
4. **Batch Operations**: Group updates when possible
5. **Lazy Evaluation**: Only load data when needed

### Database Optimizations | 数据库优化

```sql
-- Indexes for fast queries
CREATE INDEX idx_reminders_time ON reminders(time);
CREATE INDEX idx_reminders_completed ON reminders(completed);
CREATE INDEX idx_evidence_reminder ON evidence(reminder_id);

-- Optimize queries
SELECT * FROM reminders 
WHERE completed = 0 AND time <= datetime('now')
ORDER BY time ASC;  -- Uses index
```

### Memory Management | 内存管理

- **Rust RAII**: Automatic resource cleanup
- **Vue Refs**: Reactive references with automatic cleanup
- **Event Cleanup**: Unlisten on component unmount
- **File Handles**: Proper closing after operations

---

## Deployment Architecture | 部署架构

### Desktop Platforms | 桌面平台

```
Application Bundle
├── Binary Executable (Rust + Tauri)
├── Frontend Assets (HTML/CSS/JS)
├── SQLite Database (created on first run)
└── Configuration Files
```

#### macOS (.app)
- Code-signed application bundle
- Notarization for Gatekeeper
- DMG installer distribution

#### Windows (.exe / .msi)
- Signed executable
- MSI installer for enterprise
- Auto-updater support

#### Linux (AppImage / .deb / .rpm)
- Portable AppImage
- System packages for package managers

### Mobile Platform | 移动平台

#### Android (.apk / .aab)
- Conditional compilation for mobile features
- System tray disabled on mobile
- Touch-optimized UI
- App sandbox storage

### Build Process | 构建流程

```bash
# Frontend build
npm run build
    ↓
Vite bundles Vue app
    ↓
Output to dist/

# Backend build
cargo build --release
    ↓
Compile Rust to native code
    ↓
Output to target/release/

# Tauri bundle
npm run tauri build
    ↓
Package frontend + backend
    ↓
Create platform-specific installers
```

---

## Future Enhancements | 未来增强

### Planned Features | 计划功能

#### Reminders Enhancements
- [ ] Natural language input ("tomorrow at 3pm")
- [ ] Recurring reminders with advanced patterns
- [ ] Calendar view integration
- [ ] Subtasks and checklists
- [ ] Templates for common reminders
- [ ] Batch operations
- [ ] Export/Import (JSON/CSV)

#### Wiki Enhancements
- [ ] Full-text search with FTS5
- [ ] Bi-directional links between pages
- [ ] Graph view of connections
- [ ] Markdown table of contents
- [ ] Image embedding and management
- [ ] Export to PDF/HTML
- [ ] Collaborative editing (future)

#### Disk Scanner Enhancements
- [ ] File type breakdown (pie charts)
- [ ] Duplicate file detection
- [ ] File deletion from app
- [ ] Export reports
- [ ] Scheduled scans
- [ ] Historical comparisons
- [ ] Custom filters

### Architectural Improvements | 架构改进

1. **Incremental Updates**: Delta events instead of full state
2. **Virtual Scrolling**: For large datasets
3. **Offline Sync**: When cloud features are added
4. **Plugin System**: Allow third-party extensions
5. **Multi-language**: i18n support
6. **Theme Customization**: User-defined color schemes

### Scalability Considerations | 可扩展性考虑

- **Data Volume**: Optimize for 10,000+ reminders
- **Wiki Pages**: Handle 1,000+ pages efficiently
- **Concurrent Users**: Support for shared databases (future)
- **Cross-Device Sync**: Cloud sync architecture (future)

---

## Appendix | 附录

### Command Reference Quick Index | 命令快速索引

#### Reminders (11 commands)
- add_reminder, get_reminders, update_reminder, delete_reminder
- toggle_reminder, get_due_reminders, broadcast_reminders
- snooze_reminder, dismiss_notification
- set_debug_mode, get_debug_mode

#### Wiki (8 commands)
- create_wiki_page, update_wiki_page, get_wiki_page, list_wiki_pages
- delete_wiki_page, search_wiki_pages
- list_wiki_revisions, restore_wiki_revision

#### Disk Scanner (4 commands)
- scan_directory, get_home_directory
- get_system_roots, format_bytes

#### Evidence (10 commands)
- add_evidence_to_reminder, get_reminder_evidence, get_all_evidence_items
- update_evidence_desc, delete_evidence_item
- save_uploaded_file, get_evidence_file_path, open_evidence_file
- get_mime_type, format_file_size

**Total: 33 Tauri Commands**

### File Structure Reference | 文件结构参考

```
toolbox/
├── src/                          # Frontend source
│   ├── App.vue                   # Full-featured app
│   ├── AppSimple.vue             # Navigation wrapper
│   ├── main.ts                   # Entry point
│   ├── components/
│   │   ├── RemindersApp.vue      # Reminders container
│   │   ├── WikiApp.vue           # Wiki container
│   │   ├── WikiEditor.vue        # Markdown editor
│   │   ├── WikiPreview.vue       # Preview renderer
│   │   ├── WikiSidebar.vue       # Wiki navigation
│   │   ├── WikiMetadata.vue      # Page metadata
│   │   ├── DiskScanner.vue       # Scanner UI
│   │   └── DiskItem.vue          # File/folder item
│   ├── composables/
│   │   └── useWikiStore.ts       # Wiki state management
│   └── assets/                   # Static assets
│
├── src-tauri/                    # Backend source
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── lib.rs                # App setup
│   │   ├── commands/
│   │   │   └── mod.rs            # Reminder commands
│   │   ├── database/
│   │   │   ├── init.rs           # DB initialization
│   │   │   └── operations.rs    # CRUD operations
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   └── reminder.rs       # Data models
│   │   ├── notifications/
│   │   │   └── mod.rs            # Notification service
│   │   ├── tray/
│   │   │   └── mod.rs            # System tray
│   │   ├── disk_scanner.rs       # Disk scanner logic
│   │   ├── wiki_commands.rs      # Wiki commands
│   │   └── evidence_commands.rs  # Evidence commands
│   ├── Cargo.toml                # Rust dependencies
│   ├── tauri.conf.json           # Tauri configuration
│   └── build.rs                  # Build script
│
├── public/                       # Public assets
├── notification.html             # Notification window
├── package.json                  # Node dependencies
├── vite.config.ts                # Vite configuration
├── tsconfig.json                 # TypeScript configuration
├── README.md                     # User documentation
└── SOFTWARE_ARCHITECTURE.md      # This file
```

### Glossary | 术语表

- **IPC**: Inter-Process Communication (进程间通信)
- **CRUD**: Create, Read, Update, Delete (创建、读取、更新、删除)
- **SQLx**: SQL database library for Rust
- **Tauri**: Desktop application framework
- **Vue 3**: Progressive JavaScript framework
- **Rust**: Systems programming language
- **Tokio**: Async runtime for Rust
- **Serde**: Serialization framework for Rust

---

## Document Maintenance | 文档维护

**Last Updated**: 2024-11-21  
**Version**: 1.0.0  
**Maintainer**: Toolbox Development Team

This document should be updated whenever:
- New features are added
- Architecture significantly changes
- New commands are introduced
- Technology stack is updated

---

**End of Software Architecture Documentation**
