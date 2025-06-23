<template>
  <div class="max-w-4xl mx-auto space-y-6">
    <!-- 页面标题 -->
    <div class="text-center space-y-2 animate-fade-in">
      <h2 class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
        连接到 SFTP 服务器
      </h2>
      <p class="text-gray-600">安全、快速的文件传输体验</p>
    </div>

    <!-- 连接表单 -->
    <div class="card-elegant p-8 animate-scale-in">
      <form @submit.prevent="handleConnect" class="space-y-6">
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- 连接名称 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-medium text-gray-700">连接名称</span>
            </label>
            <input 
              v-model="connectionForm.name"
              type="text" 
              placeholder="我的服务器"
              class="input input-bordered input-elegant w-full"
              required
            />
          </div>

          <!-- 服务器地址 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-medium text-gray-700">服务器地址</span>
            </label>
            <input 
              v-model="connectionForm.host"
              type="text" 
              placeholder="192.168.1.100"
              class="input input-bordered input-elegant w-full"
              required
            />
          </div>

          <!-- 端口 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-medium text-gray-700">端口</span>
            </label>
            <input 
              v-model.number="connectionForm.port"
              type="number" 
              placeholder="22"
              class="input input-bordered input-elegant w-full"
              required
            />
          </div>

          <!-- 用户名 -->
          <div class="form-control">
            <label class="label">
              <span class="label-text font-medium text-gray-700">用户名</span>
            </label>
            <input 
              v-model="connectionForm.username"
              type="text" 
              placeholder="root"
              class="input input-bordered input-elegant w-full"
              required
            />
          </div>

          <!-- 密码 -->
          <div class="form-control md:col-span-2">
            <label class="label">
              <span class="label-text font-medium text-gray-700">密码</span>
            </label>
            <input
              v-model="connectionForm.password"
              type="password"
              placeholder="••••••••"
              class="input input-bordered input-elegant w-full"
              required
            />
          </div>

          <!-- 保存连接选项 -->
          <div class="form-control md:col-span-2">
            <label class="label cursor-pointer justify-start space-x-3">
              <input
                v-model="connectionForm.saveConnection"
                type="checkbox"
                class="checkbox checkbox-primary"
              />
              <div>
                <span class="label-text font-medium text-gray-700">保存此连接配置</span>
                <div class="text-sm text-gray-500">保存连接信息以便下次快速连接</div>
              </div>
            </label>
          </div>

          <!-- 保存密码选项（仅在保存连接时显示） -->
          <div v-if="connectionForm.saveConnection" class="form-control md:col-span-2">
            <label class="label cursor-pointer justify-start space-x-3">
              <input
                v-model="connectionForm.savePassword"
                type="checkbox"
                class="checkbox checkbox-primary"
              />
              <div>
                <span class="label-text font-medium text-gray-700">保存密码</span>
                <div class="text-sm text-gray-500">密码将安全存储在本地，下次连接时无需重新输入</div>
              </div>
            </label>
          </div>

          <!-- 连接描述（仅在保存连接时显示） -->
          <div v-if="connectionForm.saveConnection" class="form-control md:col-span-2">
            <label class="label">
              <span class="label-text font-medium text-gray-700">描述（可选）</span>
            </label>
            <textarea
              v-model="connectionForm.description"
              placeholder="添加连接描述，如：生产服务器、测试环境等"
              class="textarea textarea-bordered textarea-elegant w-full"
              rows="2"
            ></textarea>
          </div>
        </div>

        <!-- 操作按钮 -->
        <div class="flex justify-center space-x-4 pt-4">
          <button
            type="button"
            @click="clearForm"
            :disabled="isConnecting || isTesting"
            class="btn btn-ghost btn-elegant"
          >
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
            </svg>
            清空表单
          </button>

          <button
            type="button"
            @click="testConnection"
            :disabled="isConnecting || isTesting"
            class="btn btn-outline btn-elegant"
          >
            <svg v-if="isTesting" class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <svg v-else class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            {{ isTesting ? '测试中...' : '测试连接' }}
          </button>

          <button
            type="submit"
            :disabled="isConnecting || isTesting"
            class="btn btn-primary btn-elegant"
          >
            <svg v-if="isConnecting" class="animate-spin w-4 h-4 mr-2" fill="none" viewBox="0 0 24 24">
              <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 714 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <svg v-else class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
            </svg>
            {{ isConnecting ? '连接中...' : (editingConnection ? '更新连接' : '立即连接') }}
          </button>
        </div>
      </form>

      <!-- 状态消息 -->
      <div v-if="statusMessage" class="mt-6">
        <div 
          :class="[
            'alert animate-fade-in',
            statusType === 'success' ? 'alert-success' : 
            statusType === 'error' ? 'alert-error' : 'alert-info'
          ]"
        >
          <svg v-if="statusType === 'success'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
          </svg>
          <svg v-else-if="statusType === 'error'" class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
          </svg>
          <span>{{ statusMessage }}</span>
        </div>
      </div>
    </div>

    <!-- 已保存的连接 -->
    <div v-if="savedConnections.length > 0" class="card-elegant p-6 animate-slide-up">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-xl font-semibold text-gray-800">已保存的连接</h3>
        <div class="flex items-center space-x-2">
          <span class="text-sm text-gray-500">{{ savedConnections.length }} 个连接</span>
          <button
            @click="showSavedConnections = !showSavedConnections"
            class="btn btn-ghost btn-sm"
          >
            <svg
              :class="['w-4 h-4 transition-transform', showSavedConnections ? 'rotate-180' : '']"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"/>
            </svg>
          </button>
        </div>
      </div>

      <div v-show="showSavedConnections" class="space-y-3">
        <div
          v-for="connection in savedConnections"
          :key="connection.id"
          class="flex items-center justify-between p-4 bg-gray-50 rounded-lg hover:bg-gray-100 transition-colors"
        >
          <div class="flex-1">
            <div class="flex items-center space-x-3">
              <div class="w-10 h-10 bg-gradient-to-br from-blue-500 to-purple-600 rounded-full flex items-center justify-center">
                <svg class="w-5 h-5 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 12h14M5 12l4-4m-4 4l4 4"/>
                </svg>
              </div>
              <div>
                <h4 class="font-medium text-gray-800">{{ connection.name }}</h4>
                <p class="text-sm text-gray-600">{{ connection.username }}@{{ connection.host }}:{{ connection.port }}</p>
                <p v-if="connection.description" class="text-xs text-gray-500 mt-1">{{ connection.description }}</p>
                <div class="flex items-center space-x-4 mt-1">
                  <span class="text-xs text-gray-400">
                    创建于 {{ formatDate(connection.createdAt) }}
                  </span>
                  <span v-if="connection.lastUsed" class="text-xs text-gray-400">
                    最后使用 {{ formatDate(connection.lastUsed) }}
                  </span>
                  <span v-if="connection.savePassword" class="text-xs text-green-600">
                    已保存密码
                  </span>
                </div>
              </div>
            </div>
          </div>

          <div class="flex items-center space-x-2">
            <button
              @click="loadConnection(connection)"
              class="btn btn-sm btn-primary"
              title="快速连接"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
              </svg>
              连接
            </button>
            <button
              @click="editConnection(connection)"
              class="btn btn-sm btn-ghost"
              title="编辑连接"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"/>
              </svg>
            </button>
            <button
              @click="confirmDeleteConnection(connection)"
              class="btn btn-sm btn-ghost text-red-500 hover:text-red-700"
              title="删除连接"
            >
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
              </svg>
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- 快速连接卡片 -->
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 animate-slide-up">
      <div class="card-elegant p-6 text-center hover:scale-105 transition-transform cursor-pointer">
        <div class="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-blue-500 to-purple-600 rounded-full flex items-center justify-center">
          <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6V4m0 2a2 2 0 100 4m0-4a2 2 0 110 4m-6 8a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4m6 6v10m6-2a2 2 0 100-4m0 4a2 2 0 100 4m0-4v2m0-6V4"/>
          </svg>
        </div>
        <h3 class="font-semibold text-gray-800 mb-2">安全连接</h3>
        <p class="text-sm text-gray-600">使用 SSH 协议确保数据传输安全</p>
      </div>

      <div class="card-elegant p-6 text-center hover:scale-105 transition-transform cursor-pointer">
        <div class="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-green-500 to-teal-600 rounded-full flex items-center justify-center">
          <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
          </svg>
        </div>
        <h3 class="font-semibold text-gray-800 mb-2">高速传输</h3>
        <p class="text-sm text-gray-600">优化的传输算法，提供极速体验</p>
      </div>

      <div class="card-elegant p-6 text-center hover:scale-105 transition-transform cursor-pointer md:col-span-2 lg:col-span-1">
        <div class="w-16 h-16 mx-auto mb-4 bg-gradient-to-br from-orange-500 to-red-600 rounded-full flex items-center justify-center">
          <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
          </svg>
        </div>
        <h3 class="font-semibold text-gray-800 mb-2">文件管理</h3>
        <p class="text-sm text-gray-600">直观的文件浏览和管理界面</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, reactive, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotification } from '../composables/useNotification';
import { useConnectionStorage, type SavedConnection } from '../composables/useConnectionStorage';

// Props
defineProps<{
  isConnecting: boolean;
}>();

// Emits
const emit = defineEmits<{
  connectionSuccess: [connection: any];
}>();

// 通知系统
const { error, createOrUpdatePersistent, removeNotification } = useNotification();

// 连接存储
const {
  savedConnections,
  saveConnection,
  updateLastUsed,
  deleteConnection,
  getConnection
} = useConnectionStorage();

// 响应式数据
const isTesting = ref(false);
const statusMessage = ref('');
const statusType = ref<'success' | 'error' | 'info'>('info');
const showSavedConnections = ref(true);
const editingConnection = ref<SavedConnection | null>(null);

const connectionForm = reactive({
  name: '',
  host: '',
  port: 22,
  username: '',
  password: '',
  saveConnection: false,
  savePassword: false,
  description: ''
});

// 测试连接
const testConnection = async () => {
  if (!connectionForm.host || !connectionForm.username || !connectionForm.password) {
    error('信息不完整', '请填写完整的连接信息');
    return;
  }

  const notificationKey = `test_${connectionForm.host}`;
  isTesting.value = true;
  statusMessage.value = '';

  try {
    // 创建测试连接通知
    const notificationId = createOrUpdatePersistent(notificationKey, {
      type: 'info',
      title: '正在测试连接',
      message: `正在连接到 ${connectionForm.host}:${connectionForm.port}...`
    });

    const result = await invoke('test_sftp_connection', {
      host: connectionForm.host,
      port: connectionForm.port,
      username: connectionForm.username,
      password: connectionForm.password
    });

    // 更新为成功状态
    createOrUpdatePersistent(notificationKey, {
      type: 'success',
      title: '测试成功',
      message: result as string
    });

    showStatus(result as string, 'success');

    // 1秒后移除通知
    setTimeout(() => {
      console.log('尝试移除测试连接通知:', notificationKey);
      removeNotification(notificationKey);
    }, 1000);

  } catch (err) {
    // 更新为错误状态
    createOrUpdatePersistent(notificationKey, {
      type: 'error',
      title: '测试失败',
      message: err as string
    });

    showStatus(err as string, 'error');
  } finally {
    isTesting.value = false;
  }
};

// 建立连接
const handleConnect = async () => {
  if (!connectionForm.name || !connectionForm.host || !connectionForm.username || !connectionForm.password) {
    error('信息不完整', '请填写完整的连接信息');
    return;
  }

  const notificationKey = `connect_${connectionForm.host}`;
  const connectionInfo = {
    id: `conn_${Date.now()}`,
    name: connectionForm.name,
    host: connectionForm.host,
    port: connectionForm.port,
    username: connectionForm.username,
    password: connectionForm.password,
    connected: false
  };

  try {
    // 创建连接通知
    createOrUpdatePersistent(notificationKey, {
      type: 'info',
      title: '正在连接',
      message: `正在连接到 ${connectionInfo.name} (${connectionInfo.host}:${connectionInfo.port})...`
    });

    const connectionId = await invoke('connect_sftp', { connectionInfo });

    connectionInfo.connected = true;
    connectionInfo.id = connectionId as string;

    // 保存连接配置（如果用户选择保存）
    if (connectionForm.saveConnection) {
      try {
        const savedConnectionId = saveConnection({
          name: connectionForm.name,
          host: connectionForm.host,
          port: connectionForm.port,
          username: connectionForm.username,
          password: connectionForm.savePassword ? connectionForm.password : undefined,
          savePassword: connectionForm.savePassword,
          description: connectionForm.description,
        });

        // 更新最后使用时间
        updateLastUsed(savedConnectionId);

        console.log('连接配置已保存:', savedConnectionId);
      } catch (saveErr) {
        console.error('保存连接配置失败:', saveErr);
      }
    }

    // 更新为成功状态
    createOrUpdatePersistent(notificationKey, {
      type: 'success',
      title: '连接成功',
      message: `已成功连接到 ${connectionInfo.name}`
    });

    showStatus('连接成功！', 'success');
    emit('connectionSuccess', connectionInfo);

    // 1秒后移除通知
    setTimeout(() => {
      console.log('尝试移除连接通知:', notificationKey);
      removeNotification(notificationKey);
    }, 1000);

  } catch (err) {
    // 更新为错误状态
    createOrUpdatePersistent(notificationKey, {
      type: 'error',
      title: '连接失败',
      message: `连接到 ${connectionInfo.name} 失败: ${err}`
    });

    showStatus(err as string, 'error');
  }
};

// 显示状态消息
const showStatus = (message: string, type: 'success' | 'error' | 'info') => {
  statusMessage.value = message;
  statusType.value = type;

  if (type === 'success') {
    setTimeout(() => {
      statusMessage.value = '';
    }, 3000);
  }
};

// 加载已保存的连接到表单
const loadConnection = (connection: SavedConnection) => {
  connectionForm.name = connection.name;
  connectionForm.host = connection.host;
  connectionForm.port = connection.port;
  connectionForm.username = connection.username;
  connectionForm.password = connection.savePassword ? (connection.password || '') : '';
  connectionForm.description = connection.description || '';
  connectionForm.saveConnection = true;
  connectionForm.savePassword = connection.savePassword;

  // 更新最后使用时间
  updateLastUsed(connection.id);

  showStatus(`已加载连接配置: ${connection.name}`, 'info');
};

// 编辑连接配置
const editConnection = (connection: SavedConnection) => {
  editingConnection.value = connection;
  loadConnection(connection);
  showStatus(`正在编辑连接: ${connection.name}`, 'info');
};

// 确认删除连接
const confirmDeleteConnection = (connection: SavedConnection) => {
  if (confirm(`确定要删除连接 "${connection.name}" 吗？此操作无法撤销。`)) {
    deleteConnection(connection.id);
    showStatus(`已删除连接: ${connection.name}`, 'success');
  }
};

// 格式化日期
const formatDate = (dateString: string) => {
  const date = new Date(dateString);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) {
    return '今天';
  } else if (diffDays === 1) {
    return '昨天';
  } else if (diffDays < 7) {
    return `${diffDays} 天前`;
  } else {
    return date.toLocaleDateString('zh-CN');
  }
};

// 清空表单
const clearForm = () => {
  connectionForm.name = '';
  connectionForm.host = '';
  connectionForm.port = 22;
  connectionForm.username = '';
  connectionForm.password = '';
  connectionForm.saveConnection = false;
  connectionForm.savePassword = false;
  connectionForm.description = '';
  editingConnection.value = null;
  statusMessage.value = '';
};
</script>
