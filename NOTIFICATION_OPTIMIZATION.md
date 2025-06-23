# 通知系统优化 - 减少弹窗数量

## 问题描述
原来的通知系统在文件上传和下载过程中会产生多个弹窗：
- 开始操作时一个通知
- 进行中可能有多个状态更新通知
- 完成时又一个通知
- 错误时还有错误通知

这导致用户界面被大量弹窗干扰，用户体验不佳。

## 解决方案

### 1. 持久化通知系统

**新增功能**:
- `createOrUpdatePersistent()`: 创建或更新持久化通知
- `updateNotification()`: 更新现有通知内容
- 通知内容可以动态变化，而不是创建新通知

**核心逻辑**:
```javascript
const createOrUpdatePersistent = (key: string, notification: Omit<Notification, 'id' | 'persistent'>) => {
  // 查找是否已存在相同 key 的通知
  const existingIndex = notifications.value.findIndex(n => n.id.includes(key));
  
  if (existingIndex > -1) {
    // 更新现有通知
    notifications.value[existingIndex] = {
      ...notifications.value[existingIndex],
      ...notification,
      persistent: true
    };
    return notifications.value[existingIndex].id;
  } else {
    // 创建新的持久化通知
    const newNotification = { id: `${key}_${Date.now()}`, persistent: true, ...notification };
    notifications.value.push(newNotification);
    return newNotification.id;
  }
};
```

### 2. 文件下载优化

**原来的流程**:
1. "开始下载" 通知
2. "正在下载" 通知  
3. "下载完成" 通知

**优化后的流程**:
1. 创建持久化通知："准备下载文件: filename"
2. 更新通知内容："正在下载文件: filename..."
3. 更新通知内容："下载完成 - 保存位置: path"
4. 3秒后自动移除通知

**代码示例**:
```javascript
const downloadFile = async (file: any) => {
  const notificationKey = `download_${file.name}`;
  
  // 创建持久化通知
  const notificationId = createOrUpdatePersistent(notificationKey, {
    type: 'info',
    title: '正在下载',
    message: `准备下载文件: ${file.name}`
  });
  
  // 更新状态
  createOrUpdatePersistent(notificationKey, {
    type: 'info',
    title: '正在下载',
    message: `正在下载文件: ${file.name}...`
  });
  
  // 完成后更新
  createOrUpdatePersistent(notificationKey, {
    type: 'success',
    title: '下载完成',
    message: `${result}\n保存位置: ${localPath}`
  });
  
  // 自动移除
  setTimeout(() => removeNotification(notificationId), 3000);
};
```

### 3. 文件上传优化

**单文件上传**:
- 一个持久化通知显示整个上传过程
- 状态从 "准备上传" → "正在处理" → "正在传输" → "上传完成"

**批量上传**:
- 一个批量进度通知
- 显示总体进度：`进度: 2/5 (成功: 2, 失败: 0)`
- 最终显示汇总结果

**代码示例**:
```javascript
const handleFileUpload = async (fileList: File[]) => {
  const batchNotificationKey = 'upload_batch';
  
  // 创建批量上传通知
  createOrUpdatePersistent(batchNotificationKey, {
    type: 'info',
    title: '批量上传',
    message: `准备上传 ${fileList.length} 个文件`
  });
  
  // 处理每个文件并更新进度
  for (const file of fileList) {
    // ... 上传逻辑
    createOrUpdatePersistent(batchNotificationKey, {
      type: 'info',
      title: '批量上传进行中',
      message: `进度: ${completedCount + failedCount}/${fileList.length} (成功: ${completedCount}, 失败: ${failedCount})`
    });
  }
  
  // 最终结果
  createOrUpdatePersistent(batchNotificationKey, {
    type: finalType,
    title: finalTitle,
    message: `总计: ${fileList.length} 个文件，成功: ${completedCount}，失败: ${failedCount}`
  });
};
```

### 4. 其他操作优化

**文件删除**:
- "正在删除" → "删除成功" (同一个通知)
- 1.5秒后自动移除

**创建文件夹**:
- "正在创建" → "创建成功" (同一个通知)
- 1.5秒后自动移除

**连接测试**:
- "正在测试连接" → "测试成功/失败" (同一个通知)
- 2秒后自动移除

## 用户体验改进

### 优化前
- ❌ 每个操作产生 2-3 个独立通知
- ❌ 通知堆叠，界面混乱
- ❌ 用户需要手动关闭多个通知
- ❌ 无法看到操作的连续状态

### 优化后
- ✅ 每个操作只有 1 个通知
- ✅ 通知内容动态更新，显示进度
- ✅ 自动移除，无需手动操作
- ✅ 清晰的状态转换和最终结果

## 通知类型和时机

### 持久化通知 (自动更新内容)
- **文件下载**: 准备 → 下载中 → 完成 (3秒后移除)
- **文件上传**: 准备 → 处理 → 传输 → 完成 (2秒后移除)
- **批量上传**: 准备 → 进度更新 → 最终结果 (3秒后移除)
- **文件删除**: 删除中 → 完成 (1.5秒后移除)
- **创建文件夹**: 创建中 → 完成 (1.5秒后移除)
- **连接测试**: 测试中 → 结果 (2秒后移除)

### 即时通知 (传统方式)
- **表单验证错误**: 立即显示，5秒后移除
- **权限错误**: 立即显示，持久化直到用户关闭
- **系统错误**: 立即显示，持久化直到用户关闭

## 技术实现要点

1. **通知键值管理**: 使用操作类型+文件名作为唯一键
2. **状态更新**: 通过相同键值更新通知内容而非创建新通知
3. **自动清理**: 成功操作自动移除，错误操作保留供用户查看
4. **批量操作**: 特殊处理多文件操作，显示汇总进度

这个优化大大减少了通知弹窗的数量，提供了更清晰、更连贯的用户体验。
