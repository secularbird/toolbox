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
7. [Screen Navigation Flow | 画面迁移流程](#screen-navigation-flow--画面迁移流程)
8. [Inter-Process Communication | 进程间通信](#inter-process-communication--进程间通信)
9. [Module Interactions | 模块交互](#module-interactions--模块交互)
10. [Security Considerations | 安全考虑](#security-considerations--安全考虑)
11. [Performance Optimizations | 性能优化](#performance-optimizations--性能优化)
12. [Deployment Architecture | 部署架构](#deployment-architecture--部署架构)
13. [Future Enhancements | 未来增强](#future-enhancements--未来增强)

---

## System Overview | 系统概述

### Application Purpose | 应用目的

Toolbox is a unified desktop application that combines two essential productivity tools:
- **Wiki/Knowledge Base**: Personal knowledge management with Markdown, diagrams, and revision history
- **Reminders**: Task and reminder management with notifications and file attachments

Toolbox 是一个统一的桌面应用程序，整合了两个核心生产力工具：
- **知识库**：支持 Markdown、图表和版本历史的个人知识管理系统
- **提醒应用**：任务和提醒管理，带通知和文件附件功能

### Design Principles | 设计原则

1. **Local-First**: All data stored locally, no cloud dependency
2. **Privacy-Focused**: No data collection or external communication (optional GitHub sync)
3. **Native Performance**: Rust backend for optimal speed
4. **Cross-Platform**: Supports Windows, macOS, Linux, and Android
5. **Event-Driven**: Real-time updates across all windows
6. **Modular Design**: Independent features with shared infrastructure

1. **本地优先**：所有数据本地存储，无云依赖
2. **隐私保护**：无数据收集或外部通信（可选 GitHub 同步）
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
│     WikiApp (Main Container)      │      RemindersApp        │
│  + WikiEditor, WikiSidebar        │    (Drawer Overlay)      │
│  + WikiPreview, WikiMetadata      │  + Detail Panel          │
└────────┬─────────────────────────────────────┬────────────────┘
         │                                     │
         │       Tauri IPC (invoke/emit/listen)                
         │                                     │
┌────────┴─────────────────────────────────────┴────────────────┐
│                   Application Layer (Rust)                    │
│                      Tauri Commands API                       │
├───────────────────────────────────────────────────────────────┤
│  Reminder     │    Wiki         │   Evidence    │   Sync     │
│  Commands     │    Commands     │   Commands    │  Commands  │
│  (9 cmds)     │    (12 cmds)    │   (10 cmds)   │  (5 cmds)  │
└────────┬──────────────┬───────────────┬──────────────┬────────┘
         │              │               │              │
┌────────┴──────────────┴───────────────┴──────────────┴────────┐
│                    Domain/Business Logic                      │
│                  (Models + Operations)                        │
├───────────────────────────────────────────────────────────────┤
│  Reminder      │   WikiPage      │   Evidence    │   Sync    │
│  Operations    │   File Ops      │   File Ops    │  GitHub   │
│  + Validation  │   + Revisions   │   + MIME      │  API      │
└────────┬──────────────┬───────────────┬──────────────┬────────┘
         │              │               │              │
┌────────┴──────────────┴───────────────┴──────────────┴────────┐
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
main.ts
└── AppSimple.vue (Entry Point)
    │
    └── WikiApp.vue (Main Container)
        │
        ├── TopBar
        │   ├── Title Chip (📚 Wiki)
        │   ├── Action Buttons (New Page, Import, Save)
        │   ├── Reminders Button (Opens Drawer)
        │   └── Status Text
        │
        ├── WikiSidebar.vue
        │   ├── Search Input
        │   ├── Sections Tree (SectionNode recursive)
        │   │   └── Section Items with context menu
        │   ├── Tag Filter
        │   └── Page List
        │       └── Page Items (with context menu)
        │
        ├── Wiki Editor Panel
        │   ├── Title Row
        │   │   ├── Title Input
        │   │   ├── Metadata (updated time)
        │   │   └── Breadcrumbs
        │   ├── WikiEditor.vue
        │   │   ├── Toolbar (formatting buttons)
        │   │   ├── Mode Toggle (Markdown/WYSIWYG)
        │   │   └── CodeMirror-style Editor
        │   └── WikiPreview.vue (in split mode)
        │       └── Rendered Markdown with Mermaid/PlantUML
        │
        ├── WikiMetadata.vue (Right Panel)
        │   ├── Tags Input
        │   ├── Timestamps (created, updated)
        │   ├── Revision History List
        │   └── Delete Button
        │
        ├── Modals
        │   ├── DocumentImportModal.vue
        │   ├── TableInsertModal.vue
        │   └── ReminderInsertModal.vue
        │
        └── Reminders Drawer (Overlay)
            └── RemindersApp.vue
                ├── Sidebar
                │   ├── Smart Lists (Today, Scheduled, Flagged, All)
                │   ├── Custom Categories
                │   ├── Debug Toggle
                │   └── Sync Settings Button
                ├── Reminder List
                │   ├── Quick Add Form
                │   └── Reminder Items
                └── Detail Panel (Slide-in)
                    ├── Checkbox + Title
                    ├── Notes Editor
                    ├── Date/Time Picker
                    ├── Frequency Selector
                    ├── Category Selector
                    ├── Flag Toggle
                    ├── Attachments Section
                    │   ├── Upload Button
                    │   └── Evidence List
                    └── Action Buttons (Save, Delete)

Notification Window (Separate)
└── notification.html
    └── Standalone HTML/JS (no Vue)
```

### State Management | 状态管理

#### Composition API Pattern | 组合式 API 模式

Each feature uses Vue 3 Composition API with dedicated composables:

```typescript
// Wiki State (useWiki composable)
const {
  pages, currentPage, isLoading, error,
  loadPages, loadPage, createPage, updatePage, deletePage,
  searchPages, listRevisions, restoreRevision,
  sections, loadSections, createSection, updateSection, deleteSection
} = useWiki();

// Reminders State (in component)
const reminders = ref<Reminder[]>([]);
const activeList = ref<string>('today');
const editingReminder = ref<Reminder | null>(null);
const evidenceList = ref<Evidence[]>([]);
const debugMode = ref<boolean>(true);
const syncSettings = ref<SyncSettings>({...});
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

#### Reminders Commands (9) | 提醒命令

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

// Debug Controls
set_debug_mode(enabled: bool)
get_debug_mode() -> bool
```

#### Notification Commands (2) | 通知命令

```rust
snooze_reminder(id, minutes) -> Result<()>
dismiss_notification(window) -> Result<()>
```

#### Wiki Commands (12) | 知识库命令

```rust
// Page Operations
create_wiki_page(title, content, tags, section_id) -> WikiPage
update_wiki_page(id, title, content, tags, section_id) -> WikiPage
get_wiki_page(id) -> WikiPage
list_wiki_pages() -> Vec<WikiPageList>
delete_wiki_page(id) -> Result<()>
search_wiki_pages(query) -> Vec<WikiPageList>

// Revision Operations
list_wiki_revisions(page_id) -> Vec<WikiRevisionMeta>
restore_wiki_revision(page_id, revision_id) -> WikiPage

// Section Operations
list_sections() -> Vec<Section>
create_section(name, parent_id) -> Section
update_section(id, name) -> Section
delete_section(id) -> Result<()>
```

#### Evidence Commands (10) | 附件命令

```rust
// File Upload/Management
save_uploaded_file(file_name, file_data) -> String
get_evidence_file_path(id) -> String
open_evidence_file(file_path) -> Result<()>
get_mime_type(file_path) -> String
format_file_size(bytes) -> String

// Database Operations
add_evidence_to_reminder(reminder_id, file_type, file_path, file_name, ...) -> Evidence
get_reminder_evidence(reminder_id) -> Vec<Evidence>
get_all_evidence_items() -> Vec<Evidence>
update_evidence_desc(evidence_id, description) -> Result<()>
delete_evidence_item(evidence_id) -> Result<()>
```

#### Sync Commands (5) | 同步命令

```rust
get_sync_settings() -> SyncSettings
save_sync_settings(settings) -> Result<()>
test_github_connection(token) -> bool
sync_to_github() -> String  // Returns URL
sync_from_github() -> i32   // Returns count
```

**Total: 38 Tauri Commands**

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

## Screen Navigation Flow | 画面迁移流程

### Application Entry Flow | 应用入口流程

```
┌─────────────────────────────────────────────────────────────────┐
│                    Application Startup                          │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  main.ts                                                        │
│  ├── createApp(AppSimple.vue)                                  │
│  ├── Setup error handlers                                       │
│  └── mount("#app")                                              │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  AppSimple.vue (Entry Wrapper)                                  │
│  └── Renders WikiApp.vue as main content                       │
└─────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────────┐
│  WikiApp.vue (Main Application Shell)                           │
│                                                                  │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                      Top Bar                              │   │
│  │  [📚 Wiki] [+ New Page] [📄 Import] [📝 Reminders] [Save]│   │
│  └─────────────────────────────────────────────────────────┘   │
│                                                                  │
│  ┌─────────┐ ┌─────────────────────────────┐ ┌───────────────┐ │
│  │ Sidebar │ │        Editor Area          │ │   Metadata    │ │
│  │         │ │                              │ │               │ │
│  │Sections │ │  ┌──────────────────────┐   │ │ Tags          │ │
│  │ ∟ Pages │ │  │   Title Input        │   │ │ Created       │ │
│  │         │ │  ├──────────────────────┤   │ │ Updated       │ │
│  │ Search  │ │  │   WikiEditor         │   │ │               │ │
│  │         │ │  │   (Markdown/WYSIWYG) │   │ │ Revisions     │ │
│  │ Tags    │ │  │                      │   │ │ ∟ Restore     │ │
│  │         │ │  ├──────────────────────┤   │ │               │ │
│  │         │ │  │   WikiPreview        │   │ │ [Delete]      │ │
│  │         │ │  │   (Live Render)      │   │ │               │ │
│  │         │ │  └──────────────────────┘   │ │               │ │
│  └─────────┘ └─────────────────────────────┘ └───────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Reminders Drawer Flow | 提醒抽屉流程

```
User clicks [📝 Reminders] button
                │
                ▼
┌─────────────────────────────────────────────────────────────────┐
│  Reminders Overlay (Full-screen backdrop)                       │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │                   Reminders Drawer                        │  │
│  │  ┌─────────────────────────────────────────────────────┐ │  │
│  │  │ Header: 📝 Reminders                           [✕] │ │  │
│  │  └─────────────────────────────────────────────────────┘ │  │
│  │                                                           │  │
│  │  ┌─────────┐ ┌─────────────────────┐ ┌───────────────┐  │  │
│  │  │Sidebar  │ │   Reminder List     │ │ Detail Panel  │  │  │
│  │  │         │ │                     │ │ (slide-in)    │  │  │
│  │  │Smart    │ │ [Quick Add Form]    │ │               │  │  │
│  │  │ ∟Today  │ │                     │ │ Title         │  │  │
│  │  │ ∟Sched. │ │ ○ Reminder 1        │ │ Notes         │  │  │
│  │  │ ∟Flagged│ │ ○ Reminder 2        │ │ Date/Time     │  │  │
│  │  │ ∟All    │ │ ✓ Reminder 3 ✓     │ │ Repeat        │  │  │
│  │  │         │ │                     │ │ Category      │  │  │
│  │  │My Lists │ │                     │ │ Flag          │  │  │
│  │  │ ∟Work   │ │                     │ │               │  │  │
│  │  │ ∟Pers.  │ │                     │ │ Attachments   │  │  │
│  │  │ ∟...    │ │                     │ │ [+ Add File]  │  │  │
│  │  │         │ │                     │ │               │  │  │
│  │  │[+ Cat]  │ │                     │ │ [Save] [Del]  │  │  │
│  │  │[Debug]  │ │                     │ │               │  │  │
│  │  │[Sync]   │ │                     │ │               │  │  │
│  │  └─────────┘ └─────────────────────┘ └───────────────┘  │  │
│  └──────────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
                │
                ▼
Click outside or [✕] → Close drawer → Return to WikiApp
```

### Modal Flow | 模态框流程

```
WikiApp Actions → Modal Windows:

[📄 Import] Button
    │
    └──► DocumentImportModal
         ├── Drag & Drop Zone
         ├── File Preview
         └── [Import] → Creates new wiki page

[Insert Table] (from toolbar)
    │
    └──► TableInsertModal
         ├── Row/Column selectors
         ├── Table Preview
         └── [Insert] → Inserts markdown table

[Insert Reminder] (from toolbar)
    │
    └──► ReminderInsertModal
         ├── Reminder form
         └── [Create] → Creates reminder + inserts reference
```

### User Interaction States | 用户交互状态

```
┌─────────────────────────────────────────────────────────────────┐
│                     State Machine                                │
└─────────────────────────────────────────────────────────────────┘

Wiki Editor States:
┌──────────┐    select page    ┌──────────┐    edit content    ┌──────────┐
│  Empty   │ ─────────────────►│ Viewing  │ ──────────────────►│ Editing  │
│  State   │                   │  Page    │                    │  (dirty) │
└──────────┘                   └──────────┘                    └──────────┘
     │                              │                               │
     │ create page                  │ delete page                   │ autosave/save
     └──────────────────────────────┴───────────────────────────────┘

Reminder Detail Panel States:
┌──────────┐    double-click    ┌──────────┐    save/close    ┌──────────┐
│  Hidden  │ ──────────────────►│  Visible │ ────────────────►│  Hidden  │
│          │◄──────────────────│ (editing)│                   │          │
└──────────┘    click away      └──────────┘                   └──────────┘
```

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
Auto-save timer triggers (1.5s debounce)
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

#### Section Management Flow | 分类管理流程

```
User right-clicks section → Context menu
    ↓
[Add Section] / [Rename] / [Delete]
    ↓
invoke('create_section' / 'update_section' / 'delete_section')
    ↓
[Backend] Update sections.json
    ↓
Return updated section list
    ↓
[Frontend] Refresh sidebar
```

---

## Security Considerations | 安全考虑

### Tauri Security Model | Tauri 安全模型

1. **Command Whitelist**: Only registered commands are callable
2. **No Eval**: No dynamic code execution
3. **CSP Headers**: Content Security Policy enforced
4. **Protocol Restrictions**: Limited protocol access

### Data Security | 数据安全

1. **Local Storage Only**: No network transmission (except optional GitHub sync)
2. **File Permissions**: Respect OS-level permissions
3. **SQL Injection Prevention**: SQLx prepared statements
4. **Path Traversal Protection**: Validate all file paths

### Privacy Protection | 隐私保护

1. **No Telemetry**: Zero data collection
2. **Optional Cloud Sync**: GitHub sync is user-controlled
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
3. **Debounced Input**: Auto-save with debouncing (1.5s)
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

#### Reminders (9 commands)
- add_reminder, get_reminders, update_reminder, delete_reminder
- toggle_reminder, get_due_reminders, broadcast_reminders
- set_debug_mode, get_debug_mode

#### Notifications (2 commands)
- snooze_reminder, dismiss_notification

#### Wiki (12 commands)
- create_wiki_page, update_wiki_page, get_wiki_page, list_wiki_pages
- delete_wiki_page, search_wiki_pages
- list_wiki_revisions, restore_wiki_revision
- list_sections, create_section, update_section, delete_section

#### Evidence (10 commands)
- add_evidence_to_reminder, get_reminder_evidence, get_all_evidence_items
- update_evidence_desc, delete_evidence_item
- save_uploaded_file, get_evidence_file_path, open_evidence_file
- get_mime_type, format_file_size

#### Sync (5 commands)
- get_sync_settings, save_sync_settings
- test_github_connection
- sync_to_github, sync_from_github

**Total: 38 Tauri Commands**

### File Structure Reference | 文件结构参考

```
toolbox/
├── src/                          # Frontend source
│   ├── App.vue                   # Legacy standalone reminders app
│   ├── AppSimple.vue             # Entry wrapper (renders WikiApp)
│   ├── main.ts                   # Entry point
│   ├── components/
│   │   ├── WikiApp.vue           # Main wiki container + reminders drawer
│   │   ├── WikiEditor.vue        # Markdown/WYSIWYG editor
│   │   ├── WikiPreview.vue       # Live preview renderer
│   │   ├── WikiSidebar.vue       # Navigation and sections
│   │   ├── WikiMetadata.vue      # Tags and revision history
│   │   ├── RemindersApp.vue      # Full reminders application
│   │   ├── DocumentImportModal.vue # Document import
│   │   ├── TableInsertModal.vue  # Table creation wizard
│   │   ├── ReminderInsertModal.vue # Reminder insertion
│   │   ├── ContextMenu.vue       # Right-click context menu
│   │   └── SectionNode.vue       # Recursive section tree node
│   ├── composables/
│   │   └── useWikiStore.ts       # Wiki state management
│   ├── types/                    # TypeScript type definitions
│   └── utils/                    # Utility functions
│
├── src-tauri/                    # Backend source
│   ├── src/
│   │   ├── main.rs               # Entry point
│   │   ├── lib.rs                # App setup, command registration
│   │   ├── commands/
│   │   │   └── mod.rs            # Reminder commands
│   │   ├── database/
│   │   │   ├── mod.rs            # Module exports
│   │   │   ├── init.rs           # DB initialization
│   │   │   └── operations.rs    # CRUD operations
│   │   ├── models/
│   │   │   ├── mod.rs
│   │   │   └── reminder.rs       # Data models
│   │   ├── notifications/
│   │   │   └── mod.rs            # Notification service
│   │   ├── tray/
│   │   │   └── mod.rs            # System tray (desktop only)
│   │   ├── sync/
│   │   │   └── mod.rs            # GitHub sync logic
│   │   ├── wiki_commands.rs      # Wiki commands
│   │   ├── evidence_commands.rs  # Evidence commands
│   │   └── sync_commands.rs      # Sync commands
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

**Last Updated**: 2024-11-25  
**Version**: 1.1.0  
**Maintainer**: Toolbox Development Team

This document should be updated whenever:
- New features are added
- Architecture significantly changes
- New commands are introduced
- Technology stack is updated

---

**End of Software Architecture Documentation**
