# SFTP 客户端

一个基于 Tauri 2.x 框架开发的现代化 SFTP 客户端，具有优雅的用户界面和强大的文件传输功能。

## ✨ 特性

- 🔐 **安全连接**: 支持 SSH2 协议的 SFTP 连接
- 🎨 **优雅界面**: 基于 TailwindCSS 和 DaisyUI 的现代化设计
- 📁 **文件管理**: 完整的文件和文件夹操作功能
- 🚀 **高速传输**: 优化的文件上传下载体验
- 🔔 **智能通知**: 实时状态反馈和操作提示
- ⌨️ **快捷键支持**: 提高操作效率的键盘快捷键
- 📱 **响应式设计**: 适配不同屏幕尺寸

## 🛠️ 技术栈

- **前端**: Vue 3 + TypeScript + TailwindCSS + DaisyUI
- **后端**: Rust + Tauri 2.x + SSH2
- **构建工具**: Vite + pnpm

## 🚀 快速开始

### 环境要求

- Node.js 18+
- Rust 1.70+
- pnpm

### 安装依赖

```bash
pnpm install
```

### 开发模式

```bash
pnpm tauri dev
```

### 构建应用

```bash
pnpm tauri build
```

## 📖 使用说明

### 连接服务器

1. 启动应用后，在连接管理界面填写服务器信息
2. 点击"测试连接"验证配置是否正确
3. 点击"立即连接"建立 SFTP 连接

### 文件操作

- **浏览文件**: 双击文件夹进入，使用面包屑导航
- **下载文件**: 点击文件行的下载按钮
- **上传文件**: 拖拽文件到上传区域
- **删除文件**: 点击文件行的删除按钮
- **新建文件夹**: 点击"新建文件夹"按钮

### 键盘快捷键

- `F5`: 刷新文件列表
- `Esc`: 返回连接管理
- `Ctrl+N`: 新建连接
- `F1`: 显示帮助

## 🎨 设计理念

本应用采用极简主义美学设计，注重：

- **优雅的渐变配色**: 柔和的色彩过渡
- **恰到好处的留白**: 清晰的信息层级
- **精心打磨的圆角**: 现代化的视觉体验
- **细腻的微交互**: 流畅的操作反馈
- **沉浸式体验**: 专注于核心功能

## 🔧 开发

### 项目结构

```
src/
├── components/          # Vue 组件
│   ├── ConnectionManager.vue
│   ├── FileExplorer.vue
│   ├── TransferProgress.vue
│   ├── NotificationContainer.vue
│   └── HelpModal.vue
├── composables/         # Vue 组合式函数
│   ├── useNotification.ts
│   └── useKeyboardShortcuts.ts
├── App.vue             # 主应用组件
└── main.ts             # 应用入口

src-tauri/
├── src/
│   └── lib.rs          # Rust 后端逻辑
├── Cargo.toml          # Rust 依赖配置
└── tauri.conf.json     # Tauri 配置
```

## 📝 开发计划

- [ ] 支持密钥认证
- [ ] 书签和收藏夹功能
- [ ] 传输队列管理
- [ ] 多语言支持
- [ ] 主题切换
- [ ] 文件预览功能

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License

## Type Support For `.vue` Imports in TS

Since TypeScript cannot handle type information for `.vue` imports, they are shimmed to be a generic Vue component type by default. In most cases this is fine if you don't really care about component prop types outside of templates. However, if you wish to get actual prop types in `.vue` imports (for example to get props validation when using manual `h(...)` calls), you can enable Volar's Take Over mode by following these steps:

1. Run `Extensions: Show Built-in Extensions` from VS Code's command palette, look for `TypeScript and JavaScript Language Features`, then right click and select `Disable (Workspace)`. By default, Take Over mode will enable itself if the default TypeScript extension is disabled.
2. Reload the VS Code window by running `Developer: Reload Window` from the command palette.

You can learn more about Take Over mode [here](https://github.com/johnsoncodehk/volar/discussions/471).
