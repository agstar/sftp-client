# 重复通知问题修复

## 问题描述
在文件下载和上传过程中，出现了多个重复的通知弹窗：
- 下载文件时出现多个"开始传输"和"传输中"的通知
- 上传文件时也有类似的重复通知问题

## 问题根源分析

### 1. 双重通知机制
- **文件操作组件**: 创建持久化通知显示操作进度
- **主应用组件**: 监听传输事件，又创建了额外的通知
- **传输面板**: 可能也有自己的通知逻辑

### 2. 通知更新频率过高
- 在下载/上传过程中多次调用 `createOrUpdatePersistent`
- 每次调用可能创建新通知而不是更新现有通知

### 3. 通知键值不够唯一
- 使用文件名作为键值，可能导致冲突
- 同时下载同名文件时会互相干扰

## 修复方案

### 1. 移除重复的通知源

**主应用组件 (App.vue)**:
```javascript
// 修复前：会产生额外通知
const handleTransferStart = (transfer: any) => {
  appState.transfers.push(transfer);
  info('开始传输', `正在传输文件: ${transfer.filename}`); // ❌ 移除
};

// 修复后：只管理传输状态，不产生通知
const handleTransferStart = (transfer: any) => {
  appState.transfers.push(transfer);
  // 移除这里的通知，因为文件操作组件已经有持久化通知了
};
```

### 2. 优化通知键值唯一性

**文件操作组件**:
```javascript
// 修复前：可能重复
const notificationKey = `download_${file.name}`;

// 修复后：添加时间戳确保唯一性
const notificationKey = `download_${file.name}_${Date.now()}`;
```

### 3. 减少通知更新频率

**下载函数优化**:
```javascript
// 修复前：多次更新通知
createOrUpdatePersistent(notificationKey, { message: '准备下载...' });
createOrUpdatePersistent(notificationKey, { message: '正在下载...' });
createOrUpdatePersistent(notificationKey, { message: '下载完成' });

// 修复后：只在关键节点更新
createOrUpdatePersistent(notificationKey, { message: '正在下载...' });
// 下载完成后直接更新为最终状态
createOrUpdatePersistent(notificationKey, { message: '下载完成' });
```

### 4. 改进通知查找逻辑

**通知系统 (useNotification.ts)**:
```javascript
// 修复前：模糊匹配可能误判
const existingIndex = notifications.value.findIndex(n => n.id.includes(key));

// 修复后：精确匹配
const existingIndex = notifications.value.findIndex(n => n.id === key || n.id.startsWith(key + '_'));
```

## 修复后的通知流程

### 文件下载
1. **开始**: 创建通知 "正在下载文件: filename (size)"
2. **完成**: 更新通知 "下载完成 - 保存位置: path"
3. **自动移除**: 3秒后移除成功通知

### 文件上传
1. **开始**: 创建通知 "正在上传文件: filename (size)"
2. **完成**: 更新通知 "上传完成"
3. **自动移除**: 2秒后移除成功通知

### 批量上传
1. **开始**: 创建批量通知 "准备上传 N 个文件"
2. **进度**: 更新通知 "进度: X/N (成功: Y, 失败: Z)"
3. **完成**: 更新通知 "总计: N 个文件，成功: Y，失败: Z"
4. **自动移除**: 3秒后移除

## 技术改进要点

### 1. 通知职责分离
- **文件操作组件**: 负责操作进度通知
- **主应用组件**: 只管理传输状态，不产生通知
- **传输面板**: 显示传输列表，不产生通知

### 2. 唯一键值策略
- 使用 `操作类型_文件名_时间戳` 格式
- 确保同时操作同名文件时不冲突

### 3. 通知生命周期管理
- 成功操作：自动移除（1.5-3秒）
- 失败操作：保留供用户查看
- 批量操作：显示汇总结果

### 4. 状态更新优化
- 减少不必要的中间状态更新
- 只在关键节点更新通知内容
- 避免频繁的通知重建

## 测试验证

### 测试场景
1. **单文件下载**: 应该只有1个通知，内容从"正在下载"变为"下载完成"
2. **单文件上传**: 应该只有1个通知，内容从"正在上传"变为"上传完成"
3. **批量上传**: 应该只有1个批量通知显示进度
4. **同时操作**: 同时下载多个文件，每个文件1个独立通知
5. **同名文件**: 同时操作同名文件，通知不应冲突

### 预期结果
- ✅ 每个操作最多1个通知
- ✅ 通知内容动态更新而非重复创建
- ✅ 成功操作自动移除通知
- ✅ 失败操作保留错误信息
- ✅ 界面清爽，无通知轰炸

这个修复彻底解决了重复通知的问题，确保每个操作只产生一个清晰的进度通知。
