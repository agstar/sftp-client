# Windows SFTP 支持修复

## 问题分析

**原始错误**: `警告: 无法获取文件名，跳过: "C:"`

**根本原因**:
- 应用原本只支持 Unix/Linux 路径格式
- Windows 盘符（如 "C:"）无法通过标准的 `file_name()` 方法解析
- 导致所有 Windows 盘符被跳过，显示空目录

## 修复内容

### 1. 后端修复 (src-tauri/src/lib.rs)

**Windows 路径识别和处理**:
```rust
// 获取文件名，支持 Windows 路径格式
let name = if let Some(file_name) = entry_path.file_name().and_then(|n| n.to_str()) {
    // 标准文件名处理
    file_name.to_string()
} else {
    let path_str = entry_path.to_string_lossy();
    
    // 检查是否是 Windows 盘符 (如 "C:", "D:" 等)
    if path_str.len() == 2 && path_str.ends_with(':') {
        let drive_name = format!("{}盘", &path_str[0..1]);
        println!("识别为 Windows 盘符: '{}' -> '{}'", path_str, drive_name);
        drive_name
    } else if path_str.contains('\\') {
        // Windows 路径，提取最后一部分
        let parts: Vec<&str> = path_str.split('\\').collect();
        let last_part = parts.last().map(|s| s.to_string()).unwrap_or_else(|| path_str.to_string());
        last_part
    } else {
        // 使用完整路径作为名称
        path_str.to_string()
    }
};
```

### 2. 前端修复 (src/components/FileExplorer.vue)

**路径导航支持**:
```javascript
const navigateToPath = (path: string) => {
  let normalizedPath = path;
  
  // 检查是否是 Windows 盘符路径
  if (path.match(/^[A-Z]:$/i)) {
    // Windows 盘符，如 "C:"
    normalizedPath = path + '/';
  } else if (path.includes('\\')) {
    // Windows 路径，转换为正斜杠
    normalizedPath = path.replace(/\\/g, '/');
  }
  
  currentPath.value = normalizedPath;
  loadDirectory();
};
```

**双击文件夹处理**:
```javascript
const handleFileDoubleClick = (file: any) => {
  if (file.is_dir) {
    let newPath;
    
    if (file.name.match(/^[A-Z]盘$/i)) {
      // Windows 盘符，如 "C盘"
      const driveLetter = file.name.charAt(0);
      newPath = driveLetter + ':';
    } else {
      // 标准路径构建
      // ...
    }
    
    navigateToPath(newPath);
  }
};
```

**返回上级目录**:
```javascript
const navigateUp = () => {
  // 检查是否是 Windows 盘符根目录
  if (currentPath.value.match(/^[A-Z]:\/$/i) || currentPath.value.match(/^[A-Z]:$/i)) {
    navigateToPath('/'); // 返回盘符列表
    return;
  }
  
  // 其他标准处理...
};
```

## 支持的路径格式

### Windows 格式
- **盘符**: `C:`, `D:`, `E:` 等
- **盘符根目录**: `C:/`, `D:/` 等
- **Windows 路径**: `C:\Users\Username\Documents`
- **显示名称**: `C盘`, `D盘` 等

### Unix/Linux 格式
- **根目录**: `/`
- **绝对路径**: `/home/user/documents`
- **相对路径**: `documents/files`

## 预期行为

### 连接到 Windows SFTP 服务器
1. **根目录 (`/`)**: 显示所有可用盘符（C盘、D盘等）
2. **双击盘符**: 进入对应盘符的根目录（如 `C:/`）
3. **文件夹导航**: 正常进入子文件夹
4. **返回上级**: 从盘符根目录返回到盘符列表

### 连接到 Unix/Linux SFTP 服务器
- 保持原有的 Unix 路径处理逻辑
- 正常的 `/` 根目录和子目录导航

## 调试输出示例

### Windows 服务器连接
```
=== 开始列出目录 ===
请求路径: '/' (连接ID: conn_xxx)
处理条目 1: "C:"
  识别为 Windows 盘符: 'C:' -> 'C盘'
  -> 添加文件: C盘 (路径: C:, 目录: true, 大小: 0)
处理条目 2: "D:"
  识别为 Windows 盘符: 'D:' -> 'D盘'
  -> 添加文件: D盘 (路径: D:, 目录: true, 大小: 0)
最终文件数量: 2 (原始: 2)
```

### 双击 C 盘
```
双击文件夹详情: {name: "C盘", path: "C:", currentPath: "/"}
Windows 盘符路径: C:
导航到路径 - 原始: C:
识别为 Windows 盘符: C:/
标准化路径: C:/
```

## 测试步骤

1. **重新启动应用**
2. **连接到 Windows SFTP 服务器**
3. **验证盘符显示**: 应该看到 C盘、D盘等
4. **测试双击盘符**: 应该能进入盘符根目录
5. **测试返回上级**: 从盘符根目录返回到盘符列表
6. **查看调试日志**: 确认路径处理正确

## 兼容性

- ✅ **Windows SFTP 服务器**: 完全支持盘符和 Windows 路径
- ✅ **Unix/Linux SFTP 服务器**: 保持原有功能
- ✅ **混合环境**: 自动识别路径格式

这个修复应该完全解决 Windows SFTP 服务器的兼容性问题，让你能够正常浏览 C 盘和其他盘符的内容。
