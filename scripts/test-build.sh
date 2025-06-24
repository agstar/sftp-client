#!/bin/bash

# SFTP Web 构建测试脚本
# 用于本地测试GitHub Actions构建流程

set -e

echo "🚀 开始构建测试..."

# 检查环境
echo "📋 检查环境..."
echo "Node.js版本: $(node --version)"
echo "pnpm版本: $(pnpm --version)"
echo "Rust版本: $(rustc --version)"

# 安装依赖
echo "📦 安装前端依赖..."
pnpm install

# 类型检查
echo "🔍 运行类型检查..."
pnpm run type-check

# 构建前端
echo "🏗️ 构建前端..."
pnpm run build

# 测试Rust编译
echo "🦀 测试Rust编译..."
cd src-tauri
cargo check
cargo test
cargo clippy -- -D warnings

# 构建Tauri应用
echo "📱 构建Tauri应用..."
cargo build --release

echo "✅ 构建测试完成！"
echo "📁 构建产物位置: src-tauri/target/release/"

# 显示构建产物大小
if [ -f "target/release/sftp-web" ] || [ -f "target/release/sftp-web.exe" ]; then
    echo "📊 构建产物信息:"
    ls -lh target/release/sftp-web* || true
fi

echo "🎉 所有测试通过！"
