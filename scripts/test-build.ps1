# SFTP Web 构建测试脚本 (Windows PowerShell)
# 用于本地测试GitHub Actions构建流程

$ErrorActionPreference = "Stop"

Write-Host "🚀 开始构建测试..." -ForegroundColor Green

# 检查环境
Write-Host "📋 检查环境..." -ForegroundColor Yellow
Write-Host "Node.js版本: $(node --version)"
Write-Host "pnpm版本: $(pnpm --version)"
Write-Host "Rust版本: $(rustc --version)"

# 安装依赖
Write-Host "📦 安装前端依赖..." -ForegroundColor Yellow
pnpm install

# 类型检查
Write-Host "🔍 运行类型检查..." -ForegroundColor Yellow
pnpm run type-check

# 构建前端
Write-Host "🏗️ 构建前端..." -ForegroundColor Yellow
pnpm run build

# 测试Rust编译
Write-Host "🦀 测试Rust编译..." -ForegroundColor Yellow
Set-Location src-tauri
cargo check
cargo test
cargo clippy -- -D warnings

# 构建Tauri应用
Write-Host "📱 构建Tauri应用..." -ForegroundColor Yellow
cargo build --release

Write-Host "✅ 构建测试完成！" -ForegroundColor Green
Write-Host "📁 构建产物位置: src-tauri/target/release/"

# 显示构建产物大小
if (Test-Path "target/release/sftp-web.exe") {
    Write-Host "📊 构建产物信息:" -ForegroundColor Cyan
    Get-ChildItem target/release/sftp-web.exe | Format-Table Name, Length, LastWriteTime
}

Write-Host "🎉 所有测试通过！" -ForegroundColor Green

# 返回根目录
Set-Location ..
