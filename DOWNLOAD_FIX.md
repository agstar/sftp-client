# 文件下载功能修复

## 问题分析

**错误信息**: `创建本地文件失败: 系统找不到指定的路径。 (os error 3)`

**根本原因**:
1. 使用了硬编码的 `./downloads/` 路径，但该目录不存在
2. 没有自动创建下载目录
3. 文件名可能包含 Windows 不支持的特殊字符
4. 路径分隔符在不同操作系统上不一致

## 修复内容

### 1. 自动获取系统下载目录

**新增后端函数**:
```rust
#[tauri::command]
async fn get_downloads_directory() -> Result<String, String> {
    let downloads_dir = if cfg!(target_os = "windows") {
        // Windows: 使用 %USERPROFILE%\Downloads
        match std::env::var("USERPROFILE") {
            Ok(user_profile) => format!("{}\\Downloads", user_profile),
            Err(_) => {
                // 备选：当前目录下的 downloads 文件夹
                let current_dir = std::env::current_dir()?;
                format!("{}\\downloads", current_dir.display())
            }
        }
    } else {
        // Unix/Linux/macOS: 使用 $HOME/Downloads
        match std::env::var("HOME") {
            Ok(home) => format!("{}/Downloads", home),
            Err(_) => {
                let current_dir = std::env::current_dir()?;
                format!("{}/downloads", current_dir.display())
            }
        }
    };
    
    // 确保目录存在
    fs::create_dir_all(&downloads_dir)?;
    Ok(downloads_dir)
}
```

### 2. 改进下载文件函数

**增强的错误处理和目录创建**:
```rust
async fn download_file(
    connection_id: String,
    remote_path: String,
    local_path: String,
) -> Result<String, String> {
    // 确保本地目录存在
    let local_path_obj = Path::new(&local_path);
    if let Some(parent_dir) = local_path_obj.parent() {
        fs::create_dir_all(parent_dir)
            .map_err(|e| format!("创建本地目录失败: {}", e))?;
    }
    
    // 详细的日志记录
    println!("开始下载文件: {} -> {}", remote_path, local_path);
    
    // 文件传输逻辑...
    let bytes_copied = std::io::copy(&mut remote_file, &mut local_file)?;
    
    Ok(format!("下载完成，文件大小: {} 字节", bytes_copied))
}
```

### 3. 前端文件名处理

**文件名清理和路径构建**:
```javascript
const downloadFile = async (file: any) => {
    // 获取系统默认下载目录
    const downloadsPath = await invoke('get_downloads_directory');
    
    // 清理文件名中的特殊字符
    const sanitizedFileName = file.name.replace(/[<>:"/\\|?*]/g, '_');
    
    // 构建本地文件路径，处理不同操作系统的路径分隔符
    const localPath = `${downloadsPath}${downloadsPath.includes('\\') ? '\\' : '/'}${sanitizedFileName}`;
    
    // 下载文件...
    const result = await invoke('download_file', {
        connectionId: props.connection.id,
        remotePath: file.path,
        localPath
    });
};
```

## 支持的下载目录

### Windows
- **主要**: `C:\Users\用户名\Downloads`
- **备选**: `当前目录\downloads`

### macOS/Linux
- **主要**: `/Users/用户名/Downloads` 或 `/home/用户名/Downloads`
- **备选**: `当前目录/downloads`

## 文件名处理

### 特殊字符替换
Windows 不支持的字符会被替换为下划线：
- `<` `>` `:` `"` `/` `\` `|` `?` `*` → `_`

### 示例
- `8C4DB6DD7EF32332C1720095ABD7DDE3C84EA6CF.torrent` → 保持不变
- `file:name?.txt` → `file_name_.txt`
- `path/to/file.txt` → `path_to_file.txt`

## 调试信息

### 后端日志
```
开始下载文件: /path/to/file.torrent -> C:\Users\用户名\Downloads\file.torrent
创建目录: "C:\\Users\\用户名\\Downloads"
打开远程文件: /path/to/file.torrent
创建本地文件: C:\Users\用户名\Downloads\file.torrent
开始传输文件...
文件传输完成，传输字节数: 12345
```

### 前端日志
```
下载目录: C:\Users\用户名\Downloads
下载文件: {
  remotePath: "/path/to/file.torrent",
  localPath: "C:\\Users\\用户名\\Downloads\\file.torrent",
  fileName: "file.torrent",
  sanitizedFileName: "file.torrent"
}
```

## 错误处理改进

### 常见错误和解决方案

1. **目录不存在**:
   - 自动创建所需的目录结构
   - 提供备选下载位置

2. **权限不足**:
   - 使用用户有权限的目录
   - 提供清晰的错误信息

3. **文件名无效**:
   - 自动清理特殊字符
   - 保持文件扩展名

4. **磁盘空间不足**:
   - 提供详细的错误信息
   - 显示文件大小信息

## 测试步骤

1. **重新启动应用**
2. **连接到 SFTP 服务器**
3. **尝试下载不同类型的文件**:
   - 普通文件名
   - 包含特殊字符的文件名
   - 大文件
4. **检查下载位置**:
   - Windows: `%USERPROFILE%\Downloads`
   - macOS/Linux: `$HOME/Downloads`
5. **查看详细日志**确认下载过程

## 预期结果

- ✅ 自动使用系统默认下载目录
- ✅ 自动创建必要的目录
- ✅ 处理文件名中的特殊字符
- ✅ 跨平台路径兼容性
- ✅ 详细的下载进度和结果反馈
- ✅ 清晰的错误信息和调试日志

这个修复应该完全解决文件下载失败的问题，让下载功能在所有平台上都能正常工作。
