# GitHub Actions 自动构建说明

本项目配置了完整的GitHub Actions工作流，支持自动构建Windows、Linux和macOS的可执行文件。

## 📁 工作流文件

### 1. `.github/workflows/ci.yml` - 持续集成
**触发条件**：
- 推送到 `main` 或 `master` 分支
- 创建Pull Request到 `main` 或 `master` 分支

**功能**：
- ✅ 多平台编译测试 (Windows/Linux/macOS)
- ✅ 前端代码检查和类型检查
- ✅ Rust代码测试和Clippy检查
- ✅ 构建验证

### 2. `.github/workflows/build.yml` - 自动构建和发布
**触发条件**：
- 推送标签 (格式: `v*`)
- 手动触发 (`workflow_dispatch`)
- Pull Request

**功能**：
- 🚀 自动构建三平台可执行文件
- 📦 创建GitHub Release
- 🎯 上传构建产物

### 3. `.github/workflows/manual-build.yml` - 手动构建
**触发条件**：
- 手动触发，支持自定义参数

**功能**：
- 🎛️ 自定义版本号
- 📋 选择是否创建Release
- 💾 上传构建产物到Artifacts

### 4. `.github/workflows/release.yml` - 正式发布
**触发条件**：
- 推送版本标签 (格式: `v*`)

**功能**：
- 🏷️ 创建正式Release
- 📦 构建所有平台版本
- 📝 自动生成发布说明

## 🚀 使用方法

### 方法一：标签发布 (推荐)
```bash
# 1. 更新版本号
npm version patch  # 或 minor/major

# 2. 推送标签
git push origin --tags

# 3. GitHub Actions自动构建并发布
```

### 方法二：手动触发构建
1. 进入GitHub仓库页面
2. 点击 `Actions` 标签
3. 选择 `Manual Build` 工作流
4. 点击 `Run workflow`
5. 填写参数并运行

### 方法三：开发测试
```bash
# 推送到主分支会触发CI测试
git push origin main
```

## 📦 构建产物

### Windows
- `*.msi` - Windows安装包
- `*.exe` - 便携版可执行文件

### macOS
- `*.dmg` - macOS磁盘映像
- `*.app.tar.gz` - 应用程序包

### Linux
- `*.AppImage` - 便携版应用
- `*.deb` - Debian/Ubuntu安装包

## ⚙️ 配置说明

### 环境要求
- **Node.js**: 18.x
- **pnpm**: 8.x
- **Rust**: 最新稳定版

### 平台特定依赖

#### Ubuntu/Linux
```bash
sudo apt-get install -y \
  libgtk-3-dev \
  libwebkit2gtk-4.0-dev \
  libappindicator3-dev \
  librsvg2-dev \
  patchelf
```

#### macOS
- 支持Universal Binary (Intel + Apple Silicon)
- 自动配置交叉编译目标

#### Windows
- 使用MSVC工具链
- 自动处理Windows特定依赖

## 🔧 自定义配置

### 修改构建目标
编辑 `.github/workflows/build.yml`:
```yaml
matrix:
  include:
    - platform: 'macos-latest'
      args: '--target universal-apple-darwin'  # macOS通用二进制
    - platform: 'ubuntu-20.04'
      args: '--target x86_64-unknown-linux-gnu'  # Linux x64
    - platform: 'windows-latest'
      args: '--target x86_64-pc-windows-msvc'  # Windows x64
```

### 修改发布信息
编辑 `release.yml` 中的发布说明模板。

### 添加代码签名
```yaml
- name: Code sign (macOS)
  if: matrix.platform == 'macos-latest'
  env:
    APPLE_CERTIFICATE: ${{ secrets.APPLE_CERTIFICATE }}
    APPLE_CERTIFICATE_PASSWORD: ${{ secrets.APPLE_CERTIFICATE_PASSWORD }}
  run: |
    # 添加代码签名逻辑
```

## 🔐 Secrets配置

在GitHub仓库设置中添加以下Secrets（如需要）：

### 代码签名 (可选)
- `APPLE_CERTIFICATE` - macOS代码签名证书
- `APPLE_CERTIFICATE_PASSWORD` - 证书密码
- `WINDOWS_CERTIFICATE` - Windows代码签名证书

### 自动发布 (可选)
- `GITHUB_TOKEN` - 自动提供，无需手动设置

## 📊 工作流状态

### 查看构建状态
- 进入GitHub仓库的 `Actions` 页面
- 查看各个工作流的运行状态
- 下载构建产物

### 调试构建问题
1. 查看失败的工作流日志
2. 检查平台特定的错误信息
3. 验证依赖和配置

## 🎯 最佳实践

### 版本管理
```bash
# 语义化版本控制
git tag v1.0.0    # 主要版本
git tag v1.1.0    # 次要版本  
git tag v1.1.1    # 补丁版本
```

### 发布流程
1. **开发** → 推送到feature分支
2. **测试** → 创建PR到main分支 (触发CI)
3. **发布** → 合并PR并打标签 (触发Release)

### 质量控制
- ✅ 所有平台构建成功
- ✅ 通过代码检查和测试
- ✅ 手动验证关键功能

## 🐛 常见问题

### Q: 构建失败怎么办？
A: 检查Actions日志，常见原因：
- 依赖安装失败
- 代码编译错误
- 平台特定问题

### Q: 如何添加新的构建目标？
A: 在matrix中添加新的平台配置，并安装相应依赖。

### Q: 构建时间太长？
A: 启用缓存机制，使用更快的runner类型。

### Q: 如何跳过某个平台的构建？
A: 在工作流中添加条件判断或临时注释相关配置。

## 📚 参考资料

- [Tauri Action文档](https://github.com/tauri-apps/tauri-action)
- [GitHub Actions文档](https://docs.github.com/en/actions)
- [Tauri构建指南](https://tauri.app/v1/guides/building/)

---

通过这套完整的GitHub Actions配置，您可以实现：
- 🔄 自动化构建和测试
- 📦 多平台发布
- 🎯 版本管理
- 📊 质量控制

让开发和发布流程更加高效和可靠！
