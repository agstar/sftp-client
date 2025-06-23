# 通知自动消失问题修复

## 问题描述
上传和下载完成后，成功通知没有按预期自动消失，导致通知堆积在界面上。

## 问题根源分析

### 1. 通知 ID 不匹配
- `createOrUpdatePersistent` 函数返回的 ID 与实际通知 ID 可能不一致
- 在更新通知时，ID 可能发生变化
- `removeNotification` 使用错误的 ID 无法找到对应通知

### 2. 通知键值与 ID 的混淆
- 我们使用 `notificationKey` 创建通知
- 但在移除时使用了 `notificationId`
- 两者可能不相同，导致移除失败

### 3. 异步操作时机问题
- 通知更新和移除操作的时机可能有冲突
- setTimeout 中的闭包可能捕获了错误的变量值

## 修复方案

### 1. 统一使用通知键值
**修复前**:
```javascript
const notificationId = createOrUpdatePersistent(notificationKey, {...});
setTimeout(() => {
  removeNotification(notificationId); // ❌ 可能不匹配
}, 3000);
```

**修复后**:
```javascript
const notificationId = createOrUpdatePersistent(notificationKey, {...});
setTimeout(() => {
  removeNotification(notificationKey); // ✅ 使用键值
}, 3000);
```

### 2. 添加调试日志
为了便于调试，在所有移除操作中添加了日志：
```javascript
setTimeout(() => {
  console.log('尝试移除下载通知:', notificationKey);
  removeNotification(notificationKey);
}, 3000);
```

### 3. 确保通知键值的一致性
**createOrUpdatePersistent 函数优化**:
```javascript
const createOrUpdatePersistent = (key: string, notification) => {
  const existingIndex = notifications.value.findIndex(n => n.id === key || n.id.startsWith(key + '_'));
  
  if (existingIndex > -1) {
    // 更新现有通知，保持相同的 ID
    notifications.value[existingIndex] = {
      ...notifications.value[existingIndex],
      ...notification,
      persistent: true
    };
    return notifications.value[existingIndex].id;
  } else {
    // 创建新通知，使用传入的 key 作为 ID
    const id = key;
    const newNotification = { id, persistent: true, ...notification };
    notifications.value.push(newNotification);
    return id;
  }
};
```

## 修复的操作类型

### 1. 文件下载
- **通知键**: `download_${file.name}_${Date.now()}`
- **自动移除**: 3秒后
- **日志**: "尝试移除下载通知"

### 2. 文件上传
- **通知键**: `upload_${file.name}_${Date.now()}`
- **自动移除**: 2秒后
- **日志**: "尝试移除上传通知"

### 3. 批量上传
- **通知键**: `upload_batch`
- **自动移除**: 3秒后
- **日志**: "尝试移除批量上传通知"

### 4. 文件删除
- **通知键**: `delete_${file.name}`
- **自动移除**: 1.5秒后
- **日志**: "尝试移除删除通知"

### 5. 创建文件夹
- **通知键**: `create_${folderName}`
- **自动移除**: 1.5秒后
- **日志**: "尝试移除创建文件夹通知"

### 6. 连接测试
- **通知键**: `test_${host}`
- **自动移除**: 2秒后
- **日志**: "尝试移除测试连接通知"

### 7. 建立连接
- **通知键**: `connect_${host}`
- **自动移除**: 2秒后
- **日志**: "尝试移除连接通知"

## 调试方法

### 1. 浏览器控制台日志
现在所有的通知移除操作都会在控制台输出日志：
```
创建下载通知: download_file.txt_1234567890
尝试移除下载通知: download_file.txt_1234567890 通知键: download_file.txt_1234567890
```

### 2. 检查通知状态
可以在浏览器开发者工具中检查通知组件的状态：
- 查看 `notifications` 数组
- 确认通知的 `id` 和 `persistent` 属性
- 验证通知是否被正确移除

### 3. 验证移除逻辑
如果通知仍然没有自动消失：
1. 检查控制台是否有移除日志
2. 确认 `removeNotification` 函数是否被调用
3. 验证通知 ID 是否匹配

## 预期行为

### 成功操作
1. **创建通知**: 显示操作进行中的状态
2. **更新通知**: 操作完成后更新为成功状态
3. **自动移除**: 指定时间后自动消失
4. **控制台日志**: 显示移除操作的详细信息

### 失败操作
1. **创建通知**: 显示操作进行中的状态
2. **更新通知**: 操作失败后更新为错误状态
3. **保持显示**: 错误通知不自动移除，供用户查看
4. **手动关闭**: 用户可以手动关闭错误通知

## 测试验证

### 测试步骤
1. **重新启动应用**
2. **执行文件操作**（下载、上传、删除等）
3. **观察通知行为**：
   - 操作开始时显示进行中状态
   - 操作完成时更新为成功状态
   - 指定时间后自动消失
4. **检查控制台日志**确认移除操作执行

### 预期结果
- ✅ 成功操作的通知会自动消失
- ✅ 失败操作的通知保持显示
- ✅ 控制台显示详细的调试日志
- ✅ 界面保持清爽，无通知堆积

这个修复确保了所有成功的操作通知都能按预期自动消失，提供更好的用户体验。
