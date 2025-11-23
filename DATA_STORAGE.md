# 📂 数据存储位置说明

## 当前系统数据位置 (macOS)

### 主目录
```
~/Library/Application Support/com.yaozhuang.tauri-vue-app/
```

**完整路径**：
```
/Users/yaozhuang/Library/Application Support/com.yaozhuang.tauri-vue-app/
```

## 📁 目录结构

### 实际存储内容
```
~/Library/Application Support/com.yaozhuang.tauri-vue-app/
├── wiki/                               # Wiki 数据 (文件系统存储)
│   ├── 1763789090.json                # Wiki 页面 (JSON 格式)
│   ├── sections.json                   # 分类/章节配置
│   └── revisions/                      # 版本历史目录
│       └── {page-id}/                  # 每个页面的版本历史
│           ├── {timestamp}.json        # 历史版本快照
│           └── ...
└── reminders.db                        # Reminders 数据 (SQLite 数据库)
```

## 📊 数据存储方式

### 1. Wiki 数据 (JSON 文件)
**位置**: `~/Library/Application Support/com.yaozhuang.tauri-vue-app/wiki/`

**存储格式**:
- **页面文件**: `{page-id}.json` (每个页面一个 JSON 文件)
- **章节配置**: `sections.json` (所有章节的结构)
- **版本历史**: `revisions/{page-id}/{revision-id}.json`

**页面 JSON 结构**:
```json
{
  "id": "1763789090",
  "title": "页面标题",
  "content": "# Markdown 内容...",
  "tags": ["tag1", "tag2"],
  "notebook": "Notebook",
  "section": "Section",
  "section_id": "section-uuid",
  "created_at": 1732290290,
  "updated_at": 1732290290
}
```

### 2. Reminders 数据 (SQLite)
**位置**: `~/Library/Application Support/com.yaozhuang.tauri-vue-app/reminders.db`

**数据库表**:
- `reminders` - 提醒事项
- `evidence` - 附件/证据

## 🔍 查看数据的方法

### 方法 1: Finder (图形界面)
1. 打开 Finder
2. 按 `Cmd + Shift + G` (前往文件夹)
3. 输入: `~/Library/Application Support/com.yaozhuang.tauri-vue-app/`
4. 点击"前往"

### 方法 2: 终端命令
```bash
# 查看数据目录
open ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/

# 列出所有文件
ls -la ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/

# 查看 Wiki 页面
ls -la ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/wiki/

# 查看某个页面内容
cat ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/wiki/1763789090.json | jq
```

### 方法 3: 查看数据库
```bash
# 打开 SQLite 数据库
sqlite3 ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/reminders.db

# 查看表结构
.schema

# 查看提醒数据
SELECT * FROM reminders;

# 退出
.quit
```

## 💾 备份数据

### 备份整个应用数据
```bash
# 创建备份
cp -r ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/ ~/Desktop/wiki-backup-$(date +%Y%m%d)

# 或者打包备份
tar -czf ~/Desktop/wiki-backup-$(date +%Y%m%d).tar.gz -C ~/Library/Application\ Support/ com.yaozhuang.tauri-vue-app/
```

### 仅备份 Wiki 数据
```bash
cp -r ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/wiki/ ~/Desktop/wiki-only-backup/
```

## 🗑️ 清理数据

### 完全清理 (删除所有数据)
```bash
rm -rf ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/
```

### 仅删除 Wiki 数据
```bash
rm -rf ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/wiki/
```

### 仅删除 Reminders 数据
```bash
rm ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/reminders.db
```

## 🖥️ 其他操作系统

### Linux
**数据位置**:
```
~/.local/share/com.yaozhuang.tauri-vue-app/
```

**查看**:
```bash
ls -la ~/.local/share/com.yaozhuang.tauri-vue-app/
```

### Windows
**数据位置**:
```
%APPDATA%\com.yaozhuang.tauri-vue-app\
```

**查看**:
```powershell
# PowerShell
explorer $env:APPDATA\com.yaozhuang.tauri-vue-app

# 命令提示符
dir %APPDATA%\com.yaozhuang.tauri-vue-app
```

## 📝 注意事项

1. **数据安全**: 
   - 所有数据存储在本地，不会上传到云端
   - 建议定期备份重要数据

2. **首次运行**:
   - 目录会在首次启动应用时自动创建
   - 如果目录不存在，应用会自动创建所需的文件夹

3. **数据迁移**:
   - 可以直接复制整个目录到新电脑
   - 路径必须保持一致

4. **权限问题**:
   - 确保应用有读写权限
   - macOS 可能需要在"系统偏好设置 → 安全性与隐私"中授权

## 🔧 故障排除

### 数据丢失
```bash
# 检查目录是否存在
ls ~/Library/Application\ Support/com.yaozhuang.tauri-vue-app/

# 检查权限
ls -la ~/Library/Application\ Support/ | grep tauri-vue-app
```

### 数据损坏
```bash
# 验证 JSON 文件
cat wiki/*.json | jq empty

# 验证 SQLite 数据库
sqlite3 reminders.db "PRAGMA integrity_check;"
```

## 📊 当前数据状态

根据检查，您的系统当前有：
- ✅ Wiki 数据目录已创建
- ✅ 至少有 1 个 Wiki 页面 (`1763789090.json`)
- ✅ 章节配置文件存在 (`sections.json`)
- ✅ 版本历史目录存在
- ✅ Reminders 数据库存在 (`reminders.db`, 20KB)

**最后更新时间**: 2025-11-22 20:09
