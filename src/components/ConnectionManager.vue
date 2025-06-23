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
        </div>

        <!-- 操作按钮 -->
        <div class="flex justify-center space-x-4 pt-4">
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
              <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
            </svg>
            <svg v-else class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"/>
            </svg>
            {{ isConnecting ? '连接中...' : '立即连接' }}
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
import { ref, reactive } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { useNotification } from '../composables/useNotification';

// Props
defineProps<{
  isConnecting: boolean;
}>();

// Emits
const emit = defineEmits<{
  connectionSuccess: [connection: any];
}>();

// 通知系统
const { success, error, info, createOrUpdatePersistent, removeNotification } = useNotification();

// 响应式数据
const isTesting = ref(false);
const statusMessage = ref('');
const statusType = ref<'success' | 'error' | 'info'>('info');

const connectionForm = reactive({
  name: '',
  host: '',
  port: 22,
  username: '',
  password: ''
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
    const notificationId = createOrUpdatePersistent(notificationKey, {
      type: 'info',
      title: '正在连接',
      message: `正在连接到 ${connectionInfo.name} (${connectionInfo.host}:${connectionInfo.port})...`
    });

    const connectionId = await invoke('connect_sftp', { connectionInfo });

    connectionInfo.connected = true;
    connectionInfo.id = connectionId as string;

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
</script>
