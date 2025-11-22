# 避免端口冲突问题

## 问题描述
当启动多个应用实例或未正常关闭之前的实例时，Vite 开发服务器使用的端口（1420 和 1421）可能会被占用，导致启动失败。

## 解决方案

### 方案 1：使用清理脚本（推荐）

#### 使用 bash 脚本
```bash
./dev.sh
```

这个脚本会：
1. 检查端口 1420 和 1421 是否被占用
2. 自动清理占用的端口
3. 启动开发服务器

#### 使用 npm 脚本
```bash
# 清理端口并启动
npm run dev:clean

# 仅清理端口
npm run port:clean
```

### 方案 2：配置修改（已完成）

我们已经修改了 `vite.config.ts`：
```typescript
server: {
  port: 1420,
  strictPort: false, // 允许使用其他端口
}
```

现在如果 1420 端口被占用，Vite 会自动尝试其他端口（如 1421, 1422 等）。

### 方案 3：手动清理端口

#### macOS/Linux
```bash
# 查找占用端口的进程
lsof -ti:1420

# 杀死占用端口的进程
lsof -ti:1420 | xargs kill -9
lsof -ti:1421 | xargs kill -9
```

#### Windows
```powershell
# 查找占用端口的进程
netstat -ano | findstr :1420

# 杀死进程（替换 PID）
taskkill /PID <PID> /F
```

### 方案 4：修改端口号

如果需要使用不同的端口，可以修改 `vite.config.ts`：

```typescript
server: {
  port: 3000, // 改为你想要的端口
  strictPort: false,
}
```

同时需要确保 Tauri 配置也指向正确的端口。

## 最佳实践

### 开发时
1. **使用 `./dev.sh` 或 `npm run dev:clean` 启动**
   - 自动清理端口冲突
   - 更稳定的启动体验

2. **正确关闭应用**
   - 使用 Ctrl+C 优雅退出
   - 避免强制关闭终端

3. **定期检查端口**
   ```bash
   npm run port:clean
   ```

### 多实例开发
如果需要同时运行多个实例：

1. **使用不同端口**
   ```bash
   # 实例 1（默认）
   npm run tauri dev

   # 实例 2（修改 vite.config.ts 使用 3000 端口）
   PORT=3000 npm run dev
   ```

2. **使用 Docker 容器**
   - 每个容器独立的端口映射
   - 更好的隔离性

## 常见问题

### Q: 为什么会出现端口被占用？
A: 
- 之前的应用实例未正常关闭
- 其他应用正在使用相同端口
- 进程崩溃但未释放端口

### Q: 如何查看哪个进程占用了端口？
A:
```bash
# macOS/Linux
lsof -i:1420

# Windows
netstat -ano | findstr :1420
```

### Q: strictPort: false 安全吗？
A: 
- 在开发环境中是安全的
- Vite 会自动选择可用端口
- 如果需要固定端口，保持 `strictPort: true` 并使用清理脚本

### Q: 修改配置后需要重启吗？
A: 
- 修改 `vite.config.ts` 需要重启开发服务器
- 修改 `package.json` 不需要重启

## 自动化脚本详解

### dev.sh
```bash
#!/bin/bash
# 自动清理端口并启动开发服务器
# 使用方法：./dev.sh
```

功能：
1. ✓ 检测端口占用
2. ✓ 自动清理进程
3. ✓ 启动开发服务器
4. ✓ 友好的状态提示

### package.json scripts

```json
{
  "dev:clean": "./dev.sh",           // 清理并启动
  "port:clean": "lsof -ti:1420 ..."  // 仅清理端口
}
```

## 推荐工作流

### 日常开发
```bash
# 1. 进入项目目录
cd tauri-vue-app

# 2. 使用清理脚本启动（推荐）
./dev.sh

# 或使用 npm 命令
npm run dev:clean
```

### 遇到端口冲突时
```bash
# 1. 停止当前开发服务器（Ctrl+C）

# 2. 清理端口
npm run port:clean

# 3. 重新启动
npm run tauri dev
```

### 生产构建
```bash
# 生产构建不受端口影响
npm run build:check
npm run tauri build
```

## 相关文件

- `vite.config.ts` - Vite 配置文件
- `dev.sh` - 开发脚本
- `package.json` - npm 脚本配置
- `src-tauri/tauri.conf.json` - Tauri 配置

## 更新日志

- **2025-11-22**: 
  - 添加 `dev.sh` 脚本
  - 修改 `vite.config.ts` 设置 `strictPort: false`
  - 添加 npm 脚本 `dev:clean` 和 `port:clean`
  - 创建此文档

---

**提示**: 如果经常遇到端口冲突，建议始终使用 `./dev.sh` 或 `npm run dev:clean` 启动开发服务器。
