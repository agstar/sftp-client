# 🚀 SFTP Web 快速开始指南

## 📦 GitHub Actions 自动构建

现在您的项目已经配置了完整的GitHub Actions工作流，可以自动构建Windows、Linux和macOS的可执行文件！

## 🎯 立即开始构建

### 方法一：推送标签发布 (推荐)
```bash
# 1. 更新版本号
npm version patch  # 小版本更新 (0.1.0 → 0.1.1)
# 或
npm version minor  # 中版本更新 (0.1.0 → 0.2.0)
# 或  
npm version major  # 大版本更新 (0.1.0 → 1.0.0)

# 2. 推送标签到GitHub
git push origin --tags

# 3. 🎉 GitHub Actions自动开始构建！
```

### 方法二：手动触发构建
1. 访问您的GitHub仓库
2. 点击 `Actions` 标签页
3. 选择 `Manual Build` 工作流
4. 点击 `Run workflow` 按钮
5. 填写版本号 (如: `v1.0.0`)
6. 选择是否创建Release
7. 点击 `Run workflow` 开始构建

### 方法三：开发测试
```bash
# 推送到main分支会触发CI测试
git push origin main
```

## 📋 构建产物

构建完成后，您将获得以下文件：

### Windows 🪟
- `*.msi` - Windows安装包 (推荐)
- `*.exe` - 便携版可执行文件

### macOS 🍎
- `*.dmg` - macOS磁盘映像
- Universal Binary支持 (Intel + Apple Silicon)

### Linux 🐧
- `*.AppImage` - 便携版应用 (推荐)
- `*.deb` - Debian/Ubuntu安装包

## 🔍 查看构建状态

1. 进入GitHub仓库的 `Actions` 页面
2. 查看工作流运行状态
3. 点击具体的运行记录查看详细日志
4. 构建完成后在 `Releases` 页面下载

## 📱 本地开发

### 开发环境运行
```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm run tauri:dev
```

### 本地构建测试
```bash
# Windows (PowerShell)
.\scripts\test-build.ps1

# Linux/macOS (Bash)
chmod +x scripts/test-build.sh
./scripts/test-build.sh
```

## 🛠️ 工作流说明

### 🔄 CI (持续集成)
- **触发**: 推送到main分支或创建PR
- **功能**: 代码检查、类型检查、构建测试

### 🚀 Build (自动构建)
- **触发**: 推送标签或手动触发
- **功能**: 多平台构建、创建Release

### 📦 Release (正式发布)
- **触发**: 推送版本标签 (v*)
- **功能**: 创建正式发布、上传所有平台版本

### 🎛️ Manual Build (手动构建)
- **触发**: 手动触发
- **功能**: 自定义版本号、选择性发布

## 🎉 发布流程

1. **开发完成** → 提交代码到feature分支
2. **代码审查** → 创建PR到main分支 (触发CI)
3. **合并代码** → PR合并到main分支
4. **版本发布** → 打标签推送 (触发Release)
5. **用户下载** → 从GitHub Releases下载

## 🔧 自定义配置

### 修改应用信息
编辑 `src-tauri/tauri.conf.json`:
```json
{
  "productName": "您的应用名称",
  "version": "1.0.0",
  "identifier": "com.yourcompany.yourapp"
}
```

### 修改构建目标
编辑 `.github/workflows/build.yml` 中的matrix配置。

### 添加代码签名
在GitHub仓库设置中添加相应的Secrets。

## 🐛 常见问题

### Q: 构建失败怎么办？
A: 查看Actions日志，常见原因：
- 依赖安装失败
- 代码编译错误
- 平台特定问题

### Q: 如何修改版本号？
A: 使用 `npm version` 命令或手动编辑 `package.json`

### Q: 构建时间多长？
A: 通常5-15分钟，取决于代码复杂度和GitHub Actions队列

### Q: 支持哪些平台？
A: Windows (x64)、macOS (Universal)、Linux (x64)

## 📚 更多资源

- [GitHub Actions文档](https://docs.github.com/en/actions)
- [Tauri构建指南](https://tauri.app/v1/guides/building/)
- [详细配置说明](./GITHUB_ACTIONS_README.md)

---

🎊 **恭喜！** 您的SFTP Web应用现在已经配置了完整的自动构建流程。

只需要推送一个标签，就能自动构建出所有平台的安装包！

```bash
git tag v1.0.0 && git push origin --tags
```

然后坐等GitHub Actions为您构建出专业的跨平台应用程序！ 🚀
