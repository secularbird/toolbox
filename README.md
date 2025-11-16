# 📝 Tauri Vue Reminder App

A beautiful desktop reminder application inspired by macOS Reminders, built with Tauri, Vue.js 3, and Rust with local SQLite storage.

## ✨ Features Overview

### 🎨 UI/UX (macOS Reminders Style)
- **Three-column layout**: Smart Lists | Reminders | Detail Panel
- **macOS design language**: SF Pro font style, Apple color system, rounded corners
- **Complete dark mode support**: Auto-follows system preferences
- **Smooth animations**: Slide-in panels, checkbox animations, hover effects
- **Empty state designs**: Elegant placeholders with icons

### 📋 Smart Lists
- **📅 Today**: Quick-add reminders for today (auto-sets time to now)
- **📆 Scheduled**: All reminders with due dates
- **🚩 Flagged**: Important/starred reminders
- **📋 All**: Complete overview of all reminders

### 📁 User Lists (Custom Categories)
- 💼 Work
- 👤 Personal
- 🛒 Shopping
- 🏥 Health
- 📌 Other
- ➕ Add custom categories dynamically

### ⚡ Quick Actions
- **Fast add**: Type title + Enter in any list
- **Today list special**: No time picker needed, auto-sets to current time
- **Single-click**: Toggle completion status
- **Double-click**: Open detail editing panel
- **Hover**: Show action buttons (flag, delete)

### 📝 Reminder Properties
- Title and multi-line notes
- Due date and time
- Repeat frequency: Once, Daily, Weekly, Monthly, Yearly
- Category/List assignment
- Flag/Star for importance
- Priority levels (0-3) - ready for future use
- Tags support - ready for future use

### 🔍 Detail Edit Panel
**Double-click any reminder to open:**
- ✓ Toggle completion (large circular checkbox)
- 📝 Edit title inline
- 📄 Add/edit notes (textarea)
- 📅 Set/change date & time
- 🔄 Change repeat frequency
- 📁 Move to different list
- 🚩 Add/remove flag
- 💾 Save changes button
- 🗑️ Delete reminder button
- ✕ Close panel

### 🔔 Notification System
- Independent notification window (notification.html)
- Checks for due reminders every 30 seconds
- Top-right corner popup display
- Shows incomplete overdue tasks
- Quick actions: Complete or Snooze
- Auto-closes when no tasks remain
- Real-time sync with main window

### 🔄 Real-time Sync
- **Event-driven architecture**: Tauri Event System
- All windows sync automatically via `reminders-updated` event
- No external servers or WebSocket needed
- Instant updates across all views

### 🛠️ Debug Features
- Toggle in sidebar (🐛 Debug Logs)
- Press F12 for DevTools
- Console logging with prefixes: `[APP]`, `[NOTIFICATION]`
- Real-time connection status
- Reminder count display

## 🖥️ HMI 设计

### 1. 设计目标
- 提供类 macOS Reminders 的直观三栏体验：列表导航 / 任务列表 / 详情面板。
- 保持零学习成本：所有核心操作（添加、完成、标记、编辑）≤ 2 步。
- 统一视觉与交互逻辑：状态一致、反馈及时、暗黑模式无跳色。

### 2. 界面结构（主窗口）
```
Header(可选预留) ──────────────────────────────
Sidebar(左)
  Smart Lists 分组
  Custom Lists 分组
  Debug / 设置入口

Content(中)
  Toolbar: 快速输入框 +（未来：筛选 / 搜索）
  ReminderList: 滚动区域 + 空状态占位

Detail Panel(右, overlay slide)
  标题区 + 完成按钮 + Flag
  属性区：时间 / 重复 / 分类
  Notes 编辑区
  操作区：保存 / 删除 / 关闭
```

### 3. 主要交互流程
1. 添加提醒（Today）:
   - 输入框获得焦点 → 用户输入标题 → Enter → 自动补当前时间 → 刷新列表。
2. 添加提醒（其他列表）:
   - 输入标题 → 可选日期时间弹出（未来可内联）→ Enter 保存。
3. 编辑提醒：双击列表项 → 右侧面板出现 → 修改字段 → 保存（自动触发 emit）。
4. 完成任务：点击圆形勾选 → 视觉淡出（opacity / strike）→ 事件广播。
5. Flag 标记：悬停显示旗帜 → 点击切换 → 列表即时重排（未来可置顶）。
6. 通知处理：到期窗口显示 → 用户点击完成或 Snooze → 后端更新 → 主窗口同步。
7. Snooze：前端触发 invoke(snooze_reminder) → 时间后移 N 分钟（默认5，可扩展）。

### 4. 状态模型（前端）
```
UIState {
  activeList: string;        // today | scheduled | flagged | all | custom
  editingReminderId?: number;
  showDetailPanel: boolean;
  debugEnabled: boolean;
  loading: boolean;          // 预留未来异步批量操作
  filterText: string;        // 预留搜索
}
ReminderItemState (派生): 完成/未完成 | Flagged | 逾期(overdue) | 即将到期(dueSoon)
```
- 逾期判定：time < now && !completed。
- 即将到期：time - now ≤ 阈值（预留：15min）。

### 5. 组件职责划分
- Sidebar: 列表切换 + Debug 入口 + 统计（未来：数量 Badge）。
- ReminderList: 数据呈现 + 虚拟滚动（未来）+ 交互事件发射。
- ReminderItem: 最小可交互单元（完成 / Flag / 双击编辑）。
- DetailPanel: 表单编辑 + 验证 + 提交。
- NotificationWindow: 到期提醒聚合操作面板。

### 6. 输入与验证
- 标题：必填，长度 > 0，< 256；失败给予红色边框 + 抖动动画（预留）。
- 时间：ISO 字符串；若为空在非 Today 列表可提示“未设置时间”（影响是否进入 Scheduled）。
- 频率：限定枚举 once|daily|weekly|monthly|yearly。
- 分类：未匹配时回退到 "other"。

### 7. 反馈与提示
- 添加成功：行内立即插入并闪烁背景（200ms）。
- 保存成功：右侧面板底部出现 ✓ 轻提示（1.2s 自动消失）。
- 删除：淡出动画后移除，若当前列表为空显示空状态插画。
- Snooze：通知项显示“Snoozed”标记（灰色），重新排序至非逾期区。

### 8. 键盘与可访问性
- Tab 顺序：输入框 → 列表项（上下）→ 详情面板字段 → 操作按钮。
- Enter：提交输入 / 保存编辑。
- Esc：关闭详情面板或通知窗口（若无阻塞操作）。
- Space：在聚焦的 ReminderItem 上切换完成状态。
- ARIA：为按钮添加 aria-label（flag, delete, save）。

### 9. 暗黑模式与主题
- 自动跟随系统 (prefers-color-scheme)，切换时使用 CSS 过渡 150ms。
- 颜色对比：文本与背景对比度 ≥ 4.5:1（重要信息区域）。
- Flag / Overdue 使用强调色（Brand 蓝 / 红）在两种模式下保持区分度。

### 10. 性能目标
- 初次渲染 < 300ms（本地 100 条数据）。
- DetailPanel 打开动画 0.25s 内完成（CSS transform + opacity）。
- 列表操作（完成/Flag）视觉反馈 ≤ 50ms；后端事件同步允许异步延迟（但 UI 先行乐观更新）。

### 11. 扩展预留
- 搜索框：顶部工具栏加入模糊 + 标签过滤。
- 批量操作：复选模式（长按/快捷键 M）→ 批量完成 / 删除 / 设定分类。
- 日历视图：新窗口或右侧切换标签，与列表共享 reminders 数据源。
- 模板支持：输入框下拉快速选择预设标题 + 默认频率。

### 12. HMI 风险与缓解
| 风险 | 描述 | 缓解 |
| ---- | ---- | ---- |
| 详情面板与列表状态不同步 | 乐观更新失败导致显示不一致 | 回滚并提示错误消息条 |
| 大量提醒渲染卡顿 | >1000 条时性能下降 | 引入虚拟列表与分块加载 |
| 通知窗口频繁闪动 | 多条周期性任务同时到期 | 合并批次 + 合理节流 |
| Snooze 逻辑混乱 | 用户反复 Snooze 导致时间漂移 | 限制最大次数或显示累计延后信息 |

---

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

## 📘 使用手顺 (中文)

### 开发环境准备
1. 安装 Node.js (建议 LTS) 与 Rust (使用 rustup 安装)。
2. 在项目根目录执行：`npm install` 安装前端依赖，同时 Tauri 构建时会自动处理 Rust 端依赖。
3. 开发运行：`npm run tauri dev` （会启动主窗口与通知窗口监听）。

### 快速开始
- 添加提醒（Today 列表最快）：点击左侧 "📅 Today"，在顶部输入框输入标题后直接按 Enter，系统会自动填入当前时间。
- 在其他列表添加：选择列表 → 输入标题 →（可选）选择日期时间 → Enter 保存。

### 编辑提醒
1. 双击某条提醒，右侧详情面板滑入。
2. 可修改：标题、备注、日期时间、重复频率、所属分类、是否加旗标。
3. 点击“Save Changes”保存或直接关闭面板（已保存的字段会立即同步）。

### 快速操作
- 完成/未完成：单击圆形复选框。
- 加旗标/取消：悬停出现 🚩 按钮后点击。
- 删除：在详情面板点击 🗑️。

### 键盘快捷键
- Enter：快速添加提醒。
- F12：在开启 Debug 模式时打开开发者工具。

### 调试模式
- 左侧侧边栏可切换 Debug Logs，开启后在 Console 中看到以 [APP] / [NOTIFICATION] 前缀的日志。
- 亦可通过命令：`await invoke('set_debug_mode', { enabled: true });`

### 通知窗口
- 每 30 秒后端检查到期提醒，若存在则弹出右上角通知窗口。
- 可在通知窗口直接“完成”或“Snooze（稍后提醒）”。

### 数据存储
- 使用本地 SQLite，所有数据本地化（隐私友好，无云同步）。
- 各平台数据库位置：
  - macOS: `~/Library/Application Support/com.yaozhuang.tauri-vue-app/reminders.db`
  - Linux: `~/.local/share/com.yaozhuang.tauri-vue-app/reminders.db`
  - Windows: `%APPDATA%\com.yaozhuang.tauri-vue-app\reminders.db`

### 常见问题速查
- 数据异常：删除数据库文件后重新运行自动重建。
- 构建失败：进入 `src-tauri` 目录执行 `cargo clean && cargo build`。
- 无通知：确认应用未被系统通知权限限制，并查看 Debug Logs。

### 工作流核心
```
用户操作 → Tauri命令 → SQLite更新 → 触发事件(reminders-updated) → 所有窗口刷新
```

---

## 🏗️ Architecture

### 总体分层
```
Presentation Layer (Vue3 / notification.html)
  ├─ 组件：Sidebar / ReminderList / DetailPanel / DebugToggle
  ├─ 状态：reminders(ref) / filters(computed) / uiState(ref)
  └─ 基础设施适配：invoke(command) / listen(event)

Application Layer (Rust Commands)
  ├─ 命令入口：add_reminder / update_reminder / toggle_reminder / snooze_reminder 等
  ├─ 事务控制：单次命令内保证原子（SQLx 事务或单语句）
  └─ 事件发布：broadcast_reminders -> app.emit("reminders-updated")

Domain Layer (Models + Operations)
  ├─ 模型：Reminder { id, title, description, time, completed, category, frequency, ... }
  ├─ 规则：
  │    - 保存前：时间标准化为 ISO8601
  │    - Snooze：time += 5 或 10 分钟（策略可扩展）
  │    - Repeat：根据 frequency 计算下一次触发（预留）
  └─ 操作：CRUD 函数 (operations.rs)

Infrastructure Layer
  ├─ SQLite 持久化（SQLx + 文件路径按平台放置）
  ├─ 定时任务：通知检查定时器 (Tokio interval 30s)
  ├─ 系统托盘：tray::mod 管理菜单/退出
  └─ 插件：opener (协议安全配置)
```

### 运行时结构示意
```
┌──────────────────────────────────────────────────────────────────┐
│                          Toolbox (Tauri)                         │
│                                                                  │
│  ┌────────────────────┐        Event Bus        ┌──────────────┐ │
│  │  Main Window (Vue) │◄───────────────────────►│ Notification │ │
│  │  - 输入/过滤       │        reminders-updated │  Window      │ │
│  │  - 详情编辑        │                           │  - 到期提醒  │ │
│  └─────────┬──────────┘                           └─────┬────────┘ │
│            │ invoke(commands)                            │ invoke  │
│            ▼                                             ▼         │
│        ┌───────────────┐  domain + rules  ┌──────────────────────┐ │
│        │ Application    │────────────────►│   Domain/Operations  │ │
│        │ (Commands API) │◄────────────────┤ (SQL 构造 + 转换)    │ │
│        └─────────┬─────┘                 └───────────┬──────────┘ │
│                  │ DB access (SQLx)                   │           │
│                  ▼                                    ▼           │
│             ┌──────────┐                       ┌────────────┐     │
│             │  SQLite   │◄────────────────────►│  Timer Task │     │
│             │ reminders │     due scan (30s)    │  (interval) │     │
│             └──────────┘                       └────────────┘     │
└──────────────────────────────────────────────────────────────────┘
```

### 核心交互序列 (添加提醒)
```
Vue 输入 -> invoke('add_reminder', data)
  Rust command: 校验/规范化 -> operations::insert(db) -> fetch 全量 -> emit(reminders-updated)
Vue & Notification: 监听事件 -> 替换本地 reminders 列表 -> UI 响应式刷新
```

### 核心交互序列 (定时到期提醒弹窗)
```
Tokio interval 30s tick
  -> operations::query_due(db)
    -> 若存在未完成且到期/过期 -> 若通知窗口未打开则创建 -> emit(reminders-updated)
通知窗口接收: render 列表
用户点击完成: invoke('toggle_reminder') -> 更新 + emit -> 若无剩余任务自动关闭
```

### 事件与状态
- 单一事件：`reminders-updated` 承载最新完整列表（简化前端同步逻辑）。
- 扩展建议：将来可增加 `reminder-modified` / `reminder-deleted` 精细化增量更新。
- Debug 模式：通过 set_debug_mode 切换后端是否输出详细日志（影响命令与定时器输出）。

### 可扩展性设计点
1. Repeat 规则抽象为策略：`trait RepeatStrategy { fn next(time) -> DateTime }`。
2. Snooze 参数化：允许前端传入分钟值或预设枚举 (5,10,30)。
3. 增加搜索索引：在 Infrastructure 层添加 FTS5 虚拟表提升搜索性能。
4. 多窗口：可再开一个“统计/日历”窗口，同样订阅事件总线。
5. 数据迁移：引入 schema_version 表 + 迁移脚本数组按版本顺序执行。

### 性能与可靠性
- 写操作后立即 emit 全量：简单但 O(n) 传输，可在规模增大时改为增量 diff。
- 定时器查询使用索引 (time, completed) 优化到期扫描。
- 单窗口 UI 状态保存在内存；重启即重新加载数据库（符合本地优先）。

### 安全
- 禁用自定义不安全协议；opener 插件使用 `requireLiteralLeadingDot` 防止伪造扩展。
- 仅暴露必要 Rust 命令；无网络外放接口，减小攻击面。
- 数据全部本地存储，避免外泄风险。

### 未来演进路径
- 事件层升级为 多类型 + 增量 payload。
- 引入缓存层（内存 + 最近查询映射），减少频繁全量读取。
- 使用后台 Task 调度替代简单 interval，实现动态频率（接近到期频率提升）。

上述架构确保：低复杂度实现快速迭代；通过清晰分层为未来功能（搜索、重复策略、增量同步）预留扩展点。
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

### Android 集成

前置环境:
1. 安装 Android Studio（含 SDK, Platform Tools, Build-tools, NDK, JDK 17）。
2. 设置环境变量: ANDROID_SDK_ROOT(或 ANDROID_HOME)、JAVA_HOME、ANDROID_NDK_HOME。
3. 在 SDK Manager 安装: 至少一个 API 24+ 平台、匹配的 Build-tools、NDK。

初始化工程:
```bash
npm run tauri android init    # 生成 android/ 目录
```

开发调试:
```bash
# 启动模拟器或连接真机 (adb devices 显示设备)
npm run tauri android dev     # 构建并安装 debug 版本
```

如需前端热更新:
```bash
# 确保本地 dev server 端口 1420 可访问
adb reverse tcp:1420 tcp:1420  # 让设备访问宿主机端口
```

生产构建:
```bash
npm run tauri android build -- --apk   # 生成 APK
npm run tauri android build -- --aab   # 生成 AAB
```

签名发布:
1. 生成 keystore:
   keytool -genkey -v -keystore release.keystore -alias toolbox -keyalg RSA -keysize 2048 -validity 10000
2. 将 keystore 放入安全位置 (例如 android/) 并在 Gradle 或 CI 设置：
   STORE_FILE, STORE_PASSWORD, KEY_ALIAS, KEY_PASSWORD
3. 使用 jarsigner 或 Gradle 签名后 zipalign / apksigner 验证。

常见问题排查:
- NDK 未找到: ANDROID_NDK_HOME 指向 <sdk>/ndk/<version>。
- 构建报 minSdkVersion: 确认 tauri.conf.json 中 android.minSdkVersion=24 且模拟器 API >=24。
- 白屏: 确认 devUrl 端口映射或已执行前端打包 (npm run build)。
- 权限: 若未来加入通知/存储，需要在 AndroidManifest.xml 添加对应权限。

可定制项:
- 图标与启动图: 修改 android/app/src/main/res/mipmap-* 与 drawable。
- 包名: 在 AndroidManifest.xml 与 Gradle applicationId 调整，与 tauri identifier 保持语义一致即可。
- 深色模式: 继承前端 CSS，Android 侧自动跟随系统外观。

安全与性能建议:
- 仅保留必要的 Tauri 插件，减少包体积。
- 使用 release 构建时启用 Rust 优化 (默认 --release)。
- 若列表数量巨大可考虑在移动端默认虚拟滚动。

### 与桌面差异说明
- 文件系统路径不同：使用 Android App 沙箱目录存储 SQLite（由 Tauri 自动处理）。
- 通知窗口可选：移动端可改用原生通知（未来通过 plugin 扩展）。

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
