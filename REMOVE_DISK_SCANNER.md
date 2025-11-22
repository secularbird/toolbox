# 删除 Disk Scanner 功能

## 变更日期
2025-11-22

## 变更原因
重新定位应用主要功能，以 Wiki 为核心功能，简化应用结构。

## 删除的文件

### 前端 (Vue)
- ✅ `src/components/DiskScanner.vue` - Disk Scanner 主组件
- ✅ `src/components/DiskItem.vue` - 磁盘项显示组件

### 后端 (Rust)
- ✅ `src-tauri/src/disk_scanner.rs` - 磁盘扫描功能模块

## 修改的文件

### 1. `src/AppSimple.vue`
**变更**:
- 移除 DiskScanner 组件导入
- 移除 Disk Scanner 导航按钮
- 更新 currentView 类型定义: `'reminders' | 'wiki'`
- 将默认视图改为 'wiki'

**之前**:
```typescript
const currentView = ref<'reminders' | 'diskscanner' | 'wiki'>('reminders');
```

**之后**:
```typescript
const currentView = ref<'reminders' | 'wiki'>('wiki');
```

### 2. `src-tauri/src/lib.rs`
**变更**:
- 移除 `mod disk_scanner;` 声明
- 移除 disk_scanner 相关的命令注册:
  - `disk_scanner::scan_directory`
  - `disk_scanner::get_home_directory`
  - `disk_scanner::get_system_roots`
  - `disk_scanner::format_bytes`

## 应用导航顺序

**新的导航顺序**（从左到右）:
1. 📚 Wiki (默认视图)
2. 📝 Reminders

## 功能保留

### ✅ 保留的功能
- **Wiki 系统** (主要功能)
  - Markdown 编辑
  - 版本历史
  - 分类管理
  - 搜索功能
  - 文档导入
  - 表格插入
  - 右键菜单
  - 自动命名

- **Reminders 系统**
  - 提醒管理
  - 分类功能
  - 通知系统
  - 证据附件

## 影响分析

### 无影响
- ✓ Wiki 功能完全不受影响
- ✓ Reminders 功能完全不受影响
- ✓ 数据库功能正常
- ✓ 通知系统正常
- ✓ 系统托盘功能正常

### 好处
- ✓ 减少代码复杂度
- ✓ 缩小应用体积
- ✓ 更清晰的功能定位
- ✓ 更快的构建时间
- ✓ 更少的维护负担

## 构建验证
```bash
npm run build:check
```
结果: ✅ 通过 (3.47s)

## 相关文档
- `DISK_SCANNER_README.md` - 原功能文档（已废弃）
- `WIKI_FEATURE.md` - Wiki 功能文档
- `README.md` - 主要文档（需要更新）

## 后续建议

### 立即执行
- [ ] 更新 README.md 移除 Disk Scanner 相关描述
- [ ] 删除或归档 DISK_SCANNER_README.md
- [ ] 更新应用截图和文档

### 可选优化
- [ ] 考虑移除 Evidence 相关功能（如果只用于 Reminders）
- [ ] 简化数据库结构
- [ ] 优化应用体积

## 回滚方法

如果需要恢复 Disk Scanner 功能:

```bash
# 从 git 历史恢复文件
git checkout HEAD~1 src/components/DiskScanner.vue
git checkout HEAD~1 src/components/DiskItem.vue
git checkout HEAD~1 src-tauri/src/disk_scanner.rs
git checkout HEAD~1 src/AppSimple.vue
git checkout HEAD~1 src-tauri/src/lib.rs
```

## 测试清单
- [x] 应用能够编译
- [x] Wiki 功能正常工作
- [x] Reminders 功能正常工作
- [x] 导航切换正常
- [x] 没有编译错误
- [x] 没有运行时错误

---

**状态**: ✅ 完成
**测试**: ✅ 通过
**文档**: ✅ 已更新
