# 修复说明

## 已修复的问题

### 1. 文件浏览导航问题

**问题**: 点击"返回上级"直接回到顶层目录，而不是上一级目录。

**修复**:
- 重写了 `navigateUp` 函数的逻辑
- 正确处理路径分割和上级目录计算
- 修复了面包屑导航的路径计算

**修复代码**:
```javascript
const navigateUp = () => {
  if (currentPath.value === '/') return;
  
  // 移除末尾的斜杠（如果有的话）
  let path = currentPath.value.endsWith('/') && currentPath.value !== '/' 
    ? currentPath.value.slice(0, -1) 
    : currentPath.value;
  
  // 找到最后一个斜杠的位置
  const lastSlashIndex = path.lastIndexOf('/');
  
  // 如果是根目录下的文件夹，返回根目录
  if (lastSlashIndex === 0) {
    navigateToPath('/');
  } else if (lastSlashIndex > 0) {
    // 否则返回上一级目录
    const parentPath = path.substring(0, lastSlashIndex);
    navigateToPath(parentPath);
  }
};
```

### 2. 文件下载问题

**问题**: 点击文件下载时出现报错。

**修复**:
- 改进了下载文件的路径处理
- 添加了默认下载目录设置
- 改进了错误处理和用户反馈

**修复代码**:
```javascript
const downloadFile = async (file: any) => {
  try {
    info('开始下载', `正在下载文件: ${file.name}`);
    
    // 获取下载路径
    const downloadsPath = await getDownloadsPath();
    const localPath = `${downloadsPath}/${file.name}`;
    
    // 调用后端下载
    await invoke('download_file', {
      connectionId: props.connection.id,
      remotePath: file.path,
      localPath
    });
    
    success('下载完成', `文件 ${file.name} 已下载到 Downloads 文件夹`);
  } catch (err) {
    error('下载失败', `文件 ${file.name} 下载失败: ${err}`);
  }
};
```

### 3. 文件上传问题

**问题**: 选择文件上传没有反应。

**修复**:
- 添加了文件选择器功能
- 实现了拖拽上传和点击上传
- 添加了新的后端函数处理 base64 文件数据
- 改进了上传进度反馈

**前端修复**:
```javascript
// 添加文件输入元素
<input 
  ref="fileInput"
  type="file" 
  multiple 
  class="hidden" 
  @change="handleFileSelect"
/>

// 处理文件选择和上传
const handleFileSelect = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const selectedFiles = target.files;
  if (selectedFiles && selectedFiles.length > 0) {
    handleFileUpload(Array.from(selectedFiles));
  }
  target.value = '';
};

const uploadFile = async (file: File) => {
  // 将文件转换为 base64
  const arrayBuffer = await file.arrayBuffer();
  const uint8Array = new Uint8Array(arrayBuffer);
  const base64Data = btoa(String.fromCharCode(...uint8Array));
  
  // 调用后端上传
  await invoke('upload_file_data', {
    connectionId: props.connection.id,
    remotePath,
    fileData: base64Data,
    fileName: file.name
  });
};
```

**后端修复**:
```rust
// 新增上传文件数据函数
#[tauri::command]
async fn upload_file_data(
    connection_id: String,
    remote_path: String,
    file_data: String,
    file_name: String,
) -> Result<String, String> {
    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();
        let session = connections.get(&connection_id)
            .ok_or("连接不存在")?;
        
        let sftp = session.sftp()
            .map_err(|e| format!("创建 SFTP 会话失败: {}", e))?;
        
        // 解码 base64 数据
        let decoded_data = base64::decode(&file_data)
            .map_err(|e| format!("解码文件数据失败: {}", e))?;
        
        let mut remote_file = sftp.create(Path::new(&remote_path))
            .map_err(|e| format!("创建远程文件失败: {}", e))?;
        
        remote_file.write_all(&decoded_data)
            .map_err(|e| format!("写入文件数据失败: {}", e))?;
        
        Ok(format!("文件 {} 上传完成", file_name))
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}
```

## 其他改进

### 路径处理优化
- 统一了路径格式处理
- 确保路径始终以 `/` 开头
- 正确处理根目录和子目录的切换

### 用户体验改进
- 添加了更详细的错误提示
- 改进了加载状态显示
- 优化了文件传输进度反馈

### 依赖更新
- 添加了 `base64` 依赖用于文件数据编码
- 更新了 Tauri 命令处理器

## 测试建议

1. **导航测试**:
   - 进入多级子目录
   - 使用"返回上级"按钮逐级返回
   - 使用面包屑导航快速跳转

2. **文件操作测试**:
   - 下载不同大小的文件
   - 上传单个和多个文件
   - 测试拖拽上传功能

3. **错误处理测试**:
   - 测试网络断开情况
   - 测试权限不足的操作
   - 测试无效路径访问

## 新增修复：双击盘符加载目录失败

### 4. 双击盘符加载目录失败问题

**问题**: 双击文件夹时出现"加载目录失败"错误。

**原因分析**:
- 路径构建逻辑不正确
- 错误处理不够详细
- 缺少调试信息

**修复内容**:

1. **改进双击处理逻辑**:
```javascript
const handleFileDoubleClick = (file: any) => {
  if (file.is_dir) {
    // 构建新路径 - 总是基于当前路径和文件名
    let newPath;
    if (currentPath.value === '/') {
      newPath = '/' + file.name;
    } else {
      newPath = currentPath.value + '/' + file.name;
    }

    // 清理路径，移除重复的斜杠
    newPath = newPath.replace(/\/+/g, '/');

    console.log('双击文件夹:', file.name, '当前路径:', currentPath.value, '新路径:', newPath);
    navigateToPath(newPath);
  }
};
```

2. **增强错误处理和调试**:
```javascript
const loadDirectory = async () => {
  isLoading.value = true;
  console.log('加载目录:', currentPath.value, '连接ID:', props.connection.id);

  try {
    const result = await invoke('list_directory', {
      connectionId: props.connection.id,
      path: currentPath.value
    });
    files.value = result as any[];
    console.log('目录加载成功，文件数量:', files.value.length);
  } catch (err) {
    console.error('加载目录失败 - 路径:', currentPath.value, '错误:', err);
    error('加载目录失败', `路径: ${currentPath.value}, 错误: ${err}`);

    // 如果不是根目录，尝试返回上级目录
    if (currentPath.value !== '/') {
      warning('目录访问失败', '正在返回上级目录');
      navigateUp();
    }
  } finally {
    isLoading.value = false;
  }
};
```

3. **后端错误处理改进**:
```rust
// 列出目录内容
#[tauri::command]
async fn list_directory(
    connection_id: String,
    path: String,
) -> Result<Vec<FileInfo>, String> {
    println!("列出目录: {} (连接ID: {})", path, connection_id);

    tokio::task::spawn_blocking(move || {
        let connections = CONNECTIONS.lock().unwrap();
        let session = connections.get(&connection_id)
            .ok_or_else(|| format!("连接不存在: {}", connection_id))?;

        let sftp = session.sftp()
            .map_err(|e| format!("创建 SFTP 会话失败: {}", e))?;

        // 验证路径格式
        let normalized_path = if path.is_empty() || path == "/" {
            "/"
        } else {
            &path
        };

        println!("尝试读取目录: {}", normalized_path);

        let entries = sftp.readdir(Path::new(normalized_path))
            .map_err(|e| {
                let error_msg = format!("读取目录失败 [{}]: {}", normalized_path, e);
                println!("{}", error_msg);
                error_msg
            })?;

        // ... 处理文件列表

        println!("成功读取目录，文件数量: {}", files.len());
        Ok(files)
    }).await.map_err(|e| format!("任务执行失败: {}", e))?
}
```

4. **添加调试信息显示**:
```html
<!-- 调试信息 -->
<div class="text-xs text-gray-500 mt-1">
  当前路径: {{ currentPath }}
</div>
```

5. **修复 base64 编码问题**:
```rust
use base64::{Engine as _, engine::general_purpose};

// 解码 base64 数据
let decoded_data = general_purpose::STANDARD.decode(&file_data)
    .map_err(|e| format!("解码文件数据失败: {}", e))?;
```

### 调试建议

1. **查看控制台日志**: 打开浏览器开发者工具，查看控制台输出的详细路径信息
2. **检查后端日志**: 查看 Tauri 应用的后端日志，了解 SFTP 操作的详细情况
3. **验证路径格式**: 确保路径格式正确，特别是斜杠的使用
4. **测试权限**: 确认 SFTP 用户有访问目标目录的权限

### 测试步骤

1. 连接到 SFTP 服务器
2. 观察根目录的文件列表
3. 双击任意文件夹
4. 查看控制台输出的路径信息
5. 如果失败，检查错误消息和路径格式

这些修复应该解决双击文件夹时的加载失败问题，并提供更好的错误诊断信息。
