# 多个传输弹窗问题修复

## 问题描述
下载大文件时，右下角传输进度弹窗出现多个重复项，弹窗不停地闪烁和重复出现，影响用户体验。

## 问题根源分析

### 1. 重复的传输事件触发
- 进度更新时重复发送 `transferStart` 事件
- 每次进度更新都创建新的传输项
- 缺少区分"开始传输"和"更新进度"的事件类型

### 2. 传输状态管理混乱
- 同一个传输ID对应多个传输项
- 完成时重复发送传输事件
- 缺少统一的传输状态更新机制

### 3. 事件处理逻辑问题
- 主应用组件只处理 `transferStart` 事件
- 进度更新和完成状态都使用相同事件
- 导致传输列表中出现重复项

## 解决方案

### 1. 区分传输事件类型

**新增事件类型**:
- `transferStart`: 仅在传输开始时触发一次
- `transferUpdate`: 用于进度更新，更新现有传输项
- `transferComplete`: 传输完成时清理

**事件定义**:
```typescript
const emit = defineEmits<{
  transferStart: [transfer: any];    // 开始传输
  transferUpdate: [transfer: any];   // 更新进度
  transferComplete: [transferId: string]; // 完成传输
}>();
```

### 2. 优化传输状态管理

**下载函数修改**:
```javascript
// 只在开始时发送一次传输事件
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
emit('transferStart', transferInfo); // 只发送一次
```

**进度更新处理**:
```javascript
// 进度监听器中使用 transferUpdate
progressUnlisten = await listen('download_progress', (event) => {
  const progressData = event.payload;
  
  if (progressData.transfer_id && activeTransfers.value.has(progressData.transfer_id)) {
    const updatedTransfer = {
      ...transfer,
      transferred: progressData.bytes_copied,
      total_size: progressData.total_size,
      progress: progressData.progress || 0
    };
    
    activeTransfers.value.set(progressData.transfer_id, updatedTransfer);
    
    // 使用 transferUpdate 而不是 transferStart
    emit('transferUpdate', updatedTransfer);
  }
});
```

### 3. 主应用组件事件处理优化

**新增处理函数**:
```javascript
const handleTransferStart = (transfer: any) => {
  appState.transfers.push(transfer); // 添加新传输项
};

const handleTransferUpdate = (transfer: any) => {
  const index = appState.transfers.findIndex(t => t.id === transfer.id);
  if (index > -1) {
    appState.transfers[index] = transfer; // 更新现有传输项
  }
};

const handleTransferComplete = (transferId: string) => {
  const index = appState.transfers.findIndex(t => t.id === transferId);
  if (index > -1) {
    appState.transfers.splice(index, 1); // 移除完成的传输项
  }
};
```

**模板绑定**:
```vue
<FileExplorer
  @transfer-start="handleTransferStart"
  @transfer-update="handleTransferUpdate"
  @transfer-complete="handleTransferComplete"
/>
```

### 4. 传输完成处理优化

**下载完成处理**:
```javascript
// 在进度监听器中处理完成状态
if (progressData.completed) {
  updatedTransfer.status = 'completed';
  activeTransfers.value.set(progressData.transfer_id, updatedTransfer);
  emit('transferUpdate', updatedTransfer);
  
  // 5秒后清理
  setTimeout(() => {
    activeTransfers.value.delete(progressData.transfer_id);
    emit('transferComplete', progressData.transfer_id);
  }, 5000);
}
```

**移除重复的完成事件**:
```javascript
// 下载函数中移除重复的完成处理
// 传输完成状态会由进度监听器处理，这里不需要重复发送
```

## 技术实现要点

### 1. 事件类型分离
- **transferStart**: 创建新传输项，只调用一次
- **transferUpdate**: 更新现有传输项，可多次调用
- **transferComplete**: 清理传输项，标记完成

### 2. 状态管理优化
- 使用 Map 存储活跃传输，避免重复
- 统一的传输ID管理
- 明确的生命周期管理

### 3. 进度更新策略
- 进度更新只更新现有传输项
- 避免创建新的传输项
- 合理的清理时机

### 4. 错误处理
- 确保传输ID的唯一性
- 处理异常情况下的清理
- 避免内存泄漏

## 用户体验改进

### 修复前
- ❌ 下载时出现多个重复的传输弹窗
- ❌ 弹窗不停闪烁和跳动
- ❌ 传输列表混乱，难以跟踪进度
- ❌ 完成后弹窗不消失

### 修复后
- ✅ 每个传输只有一个弹窗
- ✅ 进度平滑更新，无闪烁
- ✅ 清晰的传输状态显示
- ✅ 完成后自动清理

## 测试验证

### 测试场景
1. **单文件下载**: 验证只有一个传输弹窗
2. **大文件下载**: 验证进度更新不创建新弹窗
3. **多文件下载**: 验证每个文件独立的传输弹窗
4. **传输完成**: 验证弹窗正确清理
5. **传输中断**: 验证异常情况处理

### 预期行为
- ✅ 每个传输操作只显示一个弹窗
- ✅ 进度条平滑更新，无重复创建
- ✅ 传输完成后弹窗自动消失
- ✅ 多个传输时各自独立显示

## 代码结构优化

### 事件流程
```
下载开始 → transferStart → 创建传输项
进度更新 → transferUpdate → 更新传输项
传输完成 → transferUpdate (completed) → 标记完成
清理延迟 → transferComplete → 移除传输项
```

### 状态管理
```
activeTransfers (Map) → 本地传输状态管理
appState.transfers (Array) → 全局传输列表显示
TransferProgress → UI 组件显示
```

这个修复确保了每个传输操作只显示一个弹窗，进度更新平滑无闪烁，提供了清晰一致的传输状态显示。
