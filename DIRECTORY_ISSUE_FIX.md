# 双击目录加载失败问题修复

## 问题分析

**错误信息**: `路径: /未知, 错误: 读取目录失败 [/未知]: [SFTP(2)] no such file`

**根本原因**:
1. 后端在处理文件名时，当无法获取有效文件名时使用了 `"未知"` 作为默认值
2. 前端双击时使用了这个无效的文件名来构建路径
3. 导致尝试访问 `/未知` 这样的无效路径

## 修复方案

### 1. 后端修复 (src-tauri/src/lib.rs)

**问题代码**:
```rust
let name = entry_path.file_name()
    .and_then(|n| n.to_str())
    .unwrap_or("未知")  // 这里导致了问题
    .to_string();
```

**修复后**:
```rust
// 获取文件名，如果失败则跳过这个文件
let name = match entry_path.file_name().and_then(|n| n.to_str()) {
    Some(n) => n.to_string(),
    None => {
        println!("警告: 无法获取文件名，跳过: {:?}", entry_path);
        continue;  // 跳过无效文件，而不是使用"未知"
    }
};

println!("文件: {} -> 路径: {} (目录: {})", file_info.name, file_info.path, file_info.is_dir);
```

### 2. 前端修复 (src/components/FileExplorer.vue)

**添加文件名验证**:
```javascript
const handleFileDoubleClick = (file: any) => {
  if (file.is_dir) {
    console.log('双击文件夹详情:', {
      name: file.name,
      path: file.path,
      currentPath: currentPath.value
    });
    
    // 检查文件名是否有效
    if (!file.name || file.name === '未知') {
      error('无效文件名', '无法进入该文件夹，文件名无效');
      return;
    }
    
    // 构建新路径...
  }
};
```

**添加调试信息显示**:
```html
<!-- 在文件列表中显示警告 -->
<div class="text-xs text-red-500" v-if="file.name === '未知'">
  警告: 文件名无效
</div>

<!-- 在路径栏显示更多信息 -->
<div class="text-xs text-gray-500 mt-1">
  当前路径: {{ currentPath }} | 文件数量: {{ files.length }}
</div>
```

## 测试步骤

1. **重新启动应用**:
   ```bash
   pnpm tauri dev
   ```

2. **连接到 SFTP 服务器**

3. **观察文件列表**:
   - 检查是否还有显示"未知"的文件
   - 查看控制台输出的文件信息

4. **测试双击**:
   - 双击有效的文件夹
   - 观察路径变化和错误处理

## 预期结果

- ✅ 不再出现"未知"文件名
- ✅ 无效文件会被自动跳过
- ✅ 双击文件夹能正常进入
- ✅ 详细的调试信息帮助诊断问题

## 调试信息

现在应用会输出详细的调试信息：

**后端日志**:
```
列出目录: / (连接ID: conn_xxx)
尝试读取目录: /
文件: folder1 -> 路径: /folder1 (目录: true)
文件: file1.txt -> 路径: /file1.txt (目录: false)
成功读取目录，文件数量: 2
```

**前端控制台**:
```
加载目录: / 连接ID: conn_xxx
目录加载成功，文件数量: 2
双击文件夹详情: {name: "folder1", path: "/folder1", currentPath: "/"}
双击文件夹: folder1 当前路径: / 新路径: /folder1
```

## 如果问题仍然存在

1. **检查 SFTP 服务器编码**: 某些服务器可能使用非 UTF-8 编码
2. **检查文件权限**: 确保 SFTP 用户有读取目录的权限
3. **查看完整错误日志**: 检查后端和前端的详细日志输出
4. **测试不同目录**: 尝试访问不同的目录，看是否是特定目录的问题

这个修复应该彻底解决 `/未知` 路径错误的问题。
