# 通知自动消失时间统一调整为1秒

## 修改概述
将所有右上角弹窗通知的自动消失时间统一调整为1秒，提供更快速、简洁的用户体验。

## 修改的文件和位置

### 1. 通知系统默认配置 (src/composables/useNotification.ts)
**修改内容**: 将默认通知持续时间从5秒改为1秒
```javascript
// 修改前
duration: 5000,

// 修改后  
duration: 1000,
```
**影响范围**: 所有使用 `success()`, `error()`, `warning()`, `info()` 方法创建的通知

### 2. 文件浏览器组件 (src/components/FileExplorer.vue)

#### 文件下载通知
```javascript
// 修改前: 3秒后自动移除
setTimeout(() => { removeNotification(notificationKey); }, 3000);

// 修改后: 1秒后自动移除  
setTimeout(() => { removeNotification(notificationKey); }, 1000);
```

#### 文件上传通知
```javascript
// 修改前: 2秒后自动移除
setTimeout(() => { removeNotification(notificationKey); }, 2000);

// 修改后: 1秒后自动移除
setTimeout(() => { removeNotification(notificationKey); }, 1000);
```

#### 文件删除通知
```javascript
// 修改前: 1.5秒后移除
setTimeout(() => { removeNotification(notificationKey); }, 1500);

// 修改后: 1秒后移除
setTimeout(() => { removeNotification(notificationKey); }, 1000);
```

#### 创建文件夹通知
```javascript
// 修改前: 1.5秒后移除
setTimeout(() => { removeNotification(notificationKey); }, 1500);

// 修改后: 1秒后移除
setTimeout(() => { removeNotification(notificationKey); }, 1000);
```

#### 批量上传通知
```javascript
// 修改前: 3秒后移除批量通知
setTimeout(() => { removeNotification(batchNotificationKey); }, 3000);

// 修改后: 1秒后移除批量通知
setTimeout(() => { removeNotification(batchNotificationKey); }, 1000);
```

### 3. 连接管理器组件 (src/components/ConnectionManager.vue)

#### 测试连接通知
```javascript
// 修改前: 2秒后移除通知
setTimeout(() => { removeNotification(notificationKey); }, 2000);

// 修改后: 1秒后移除通知
setTimeout(() => { removeNotification(notificationKey); }, 1000);
```

#### 建立连接通知
```javascript
// 修改前: 2秒后移除通知
setTimeout(() => { removeNotification(notificationKey); }, 2000);

// 修改后: 1秒后移除通知
setTimeout(() => { removeNotification(notificationKey); }, 1000);
```

## 影响的通知类型

### 自动移除的成功通知 (1秒后消失)
- ✅ 文件下载完成
- ✅ 文件上传完成
- ✅ 批量上传完成
- ✅ 文件删除成功
- ✅ 创建文件夹成功
- ✅ 连接测试成功
- ✅ 建立连接成功
- ✅ 应用启动欢迎信息
- ✅ 所有其他成功操作

### 保持显示的通知 (不自动消失)
- ❌ 错误通知 (需要用户手动关闭)
- ❌ 警告通知 (需要用户确认)
- ❌ 重要信息通知 (标记为 persistent: true)

## 用户体验改进

### 优化前
- 成功通知显示时间过长 (1.5-5秒)
- 界面可能被多个通知占据
- 用户需要等待较长时间才能看到清爽界面

### 优化后
- ✅ 统一的1秒自动消失时间
- ✅ 快速的视觉反馈
- ✅ 界面保持清爽
- ✅ 减少视觉干扰
- ✅ 提高操作效率

## 设计理念

### 1. 快速反馈
- 1秒足够用户看到操作结果
- 避免过长的通知停留时间
- 提供即时的成功确认

### 2. 界面简洁
- 快速清理成功通知
- 保持界面整洁
- 减少视觉噪音

### 3. 错误突出
- 成功通知快速消失
- 错误通知保持显示
- 突出需要用户关注的问题

## 测试验证

### 测试场景
1. **文件操作**: 下载、上传、删除文件
2. **文件夹操作**: 创建、删除文件夹
3. **连接操作**: 测试连接、建立连接
4. **批量操作**: 多文件上传
5. **错误场景**: 验证错误通知不自动消失

### 预期行为
- ✅ 所有成功操作通知在1秒后自动消失
- ✅ 错误通知保持显示供用户查看
- ✅ 界面响应快速，视觉干扰最小
- ✅ 用户可以快速连续执行多个操作

## 注意事项

### 1. 错误通知处理
错误通知仍然保持原有行为，不会自动消失，确保用户能够看到错误信息。

### 2. 重要信息通知
标记为 `persistent: true` 的通知不受此修改影响，仍需用户手动关闭。

### 3. 批量操作
批量操作的汇总通知也会在1秒后消失，但用户仍能在传输面板中查看详细信息。

这个调整提供了更快速、简洁的用户体验，让界面保持清爽的同时确保用户能够及时获得操作反馈。
