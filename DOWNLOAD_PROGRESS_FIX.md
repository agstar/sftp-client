# 下载进度显示修复

## 问题描述
下载大文件时，右下角传输进度弹窗没有显示进度变化，进度条一直停留在0%，无法看到实际的下载进度。

## 问题根源分析

### 1. 缺少进度回调机制
- 原有的下载函数使用 `std::io::copy` 一次性复制整个文件
- 没有分块传输和进度更新机制
- 前端无法获得实时的传输进度信息

### 2. 前端缺少进度监听
- 前端没有监听后端的进度事件
- 传输状态只在开始和结束时更新
- 缺少实时进度数据的处理逻辑

## 解决方案

### 1. 后端实现带进度的下载函数

**新增函数**: `download_file_with_progress`
- 使用分块读取 (8KB 缓冲区)
- 实时计算传输进度
- 通过 Tauri 事件系统发送进度更新

**核心实现**:
```rust
// 分块传输并更新进度
let mut buffer = [0u8; 8192]; // 8KB 缓冲区
let mut bytes_copied = 0u64;
let mut last_progress_update = std::time::Instant::now();

loop {
    match remote_file.read(&mut buffer) {
        Ok(0) => break, // EOF
        Ok(n) => {
            local_file.write_all(&buffer[..n])?;
            bytes_copied += n as u64;
            
            // 每100ms或每1MB更新一次进度
            if last_progress_update.elapsed().as_millis() >= 100 || bytes_copied % (1024 * 1024) == 0 {
                let progress = if total_size > 0 {
                    (bytes_copied as f64 / total_size as f64 * 100.0) as u32
                } else { 0 };
                
                // 发送进度更新事件
                app_handle.emit("download_progress", serde_json::json!({
                    "transfer_id": transfer_id,
                    "bytes_copied": bytes_copied,
                    "total_size": total_size,
                    "progress": progress
                }));
                
                last_progress_update = std::time::Instant::now();
            }
        }
        Err(e) => return Err(format!("读取远程文件失败: {}", e)),
    }
}
```

### 2. 前端实现进度监听

**添加进度监听器**:
```javascript
// 监听下载进度事件
progressUnlisten = await listen('download_progress', (event) => {
    const progressData = event.payload;
    
    if (progressData.transfer_id && activeTransfers.value.has(progressData.transfer_id)) {
        const transfer = activeTransfers.value.get(progressData.transfer_id);
        
        // 更新传输信息
        const updatedTransfer = {
            ...transfer,
            transferred: progressData.bytes_copied,
            total_size: progressData.total_size,
            progress: progressData.progress || 0
        };
        
        // 计算传输速度
        if (transfer.lastUpdate) {
            const timeDiff = Date.now() - transfer.lastUpdate;
            const bytesDiff = progressData.bytes_copied - (transfer.transferred || 0);
            if (timeDiff > 0) {
                updatedTransfer.speed = Math.round((bytesDiff / timeDiff) * 1000);
            }
        }
        updatedTransfer.lastUpdate = Date.now();
        
        activeTransfers.value.set(progressData.transfer_id, updatedTransfer);
        emit('transferStart', updatedTransfer);
    }
});
```

### 3. 传输状态管理优化

**添加传输状态跟踪**:
```javascript
// 传输状态管理
const activeTransfers = ref(new Map());

// 存储传输信息以便进度更新
const transferInfo = {
    id: transferId,
    filename: file.name,
    type: 'download',
    status: 'transferring',
    total_size: file.size,
    transferred: 0,
    speed: 0
};

activeTransfers.value.set(transferId, transferInfo);
```

## 技术实现要点

### 1. 进度更新频率控制
- **时间间隔**: 每100毫秒最多更新一次
- **数据间隔**: 每1MB数据更新一次
- **避免过频更新**: 防止UI卡顿和性能问题

### 2. 传输速度计算
- 基于时间差和字节差计算实时速度
- 使用简化的速度计算算法
- 显示格式: "XX MB/s"

### 3. 事件系统优化
- 使用 Tauri 的事件系统进行前后端通信
- 异步事件处理，不阻塞主线程
- 自动清理完成的传输记录

### 4. 错误处理和兼容性
- 保留原有的 `download_file` 函数作为备用
- 新函数出错时可以回退到原有实现
- 兼容不同大小的文件传输

## 用户体验改进

### 修复前
- ❌ 下载进度条始终显示0%
- ❌ 无法看到实际传输速度
- ❌ 不知道下载还需要多长时间
- ❌ 大文件下载时缺少反馈

### 修复后
- ✅ 实时显示下载进度百分比
- ✅ 显示传输速度 (MB/s)
- ✅ 显示已传输/总大小
- ✅ 平滑的进度条动画
- ✅ 完成后自动清理传输记录

## 性能优化

### 1. 缓冲区大小
- 使用8KB缓冲区平衡性能和内存使用
- 避免过小缓冲区导致的频繁系统调用
- 避免过大缓冲区导致的内存浪费

### 2. 进度更新策略
- 限制更新频率避免UI卡顿
- 使用异步事件处理
- 批量更新减少通信开销

### 3. 内存管理
- 及时清理完成的传输记录
- 避免内存泄漏
- 合理的事件监听器生命周期管理

## 测试验证

### 测试场景
1. **小文件下载** (< 1MB): 验证进度显示正常
2. **中等文件下载** (1-100MB): 验证进度更新流畅
3. **大文件下载** (> 100MB): 验证长时间传输的稳定性
4. **多文件同时下载**: 验证并发传输的进度显示
5. **网络中断**: 验证错误处理和恢复

### 预期行为
- ✅ 进度条从0%平滑增长到100%
- ✅ 传输速度实时更新
- ✅ 文件大小信息准确显示
- ✅ 完成后进度面板自动清理
- ✅ 错误情况下有适当的错误提示

这个修复为下载操作提供了完整的进度反馈，大大改善了用户体验，特别是在下载大文件时。
