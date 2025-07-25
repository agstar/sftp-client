<template>
  <div class="fixed bottom-4 right-4 w-96 space-y-2 z-50">
    <div
      v-for="transfer in activeTransfers"
      :key="transfer.id"
      class="card-elegant p-4 animate-slide-up"
    >
      <div class="flex items-center mb-2">
        <!-- 文件信息区域，使用 flex-1 和 min-width: 0 确保可以正确截断 -->
        <div class="flex items-center space-x-2 flex-1 min-w-0">
          <svg
            v-if="transfer.type === 'upload'"
            class="w-5 h-5 text-blue-600 flex-shrink-0"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M9 11l3-3m0 0l3 3m-3-3v8"/>
          </svg>
          <svg
            v-else
            class="w-5 h-5 text-green-600 flex-shrink-0"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
          </svg>
          <!-- 文件名使用 truncate 和 title 属性显示完整名称 -->
          <span
            class="font-medium text-gray-800 truncate"
            :title="transfer.filename"
          >
            {{ transfer.filename }}
          </span>
        </div>

        <!-- 按钮区域，使用 flex-shrink-0 确保按钮不会被压缩 -->
        <div class="flex-shrink-0 ml-2">
          <!-- 只在传输进行中时显示取消按钮 -->
          <button
            v-if="transfer.status === 'transferring' || transfer.status === 'pending'"
            @click="cancelTransfer(transfer.id)"
            class="btn btn-ghost btn-xs text-gray-500 hover:text-red-600"
            title="取消传输"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>

          <!-- 完成状态显示关闭按钮 -->
          <button
            v-else-if="transfer.status === 'completed' || transfer.status === 'error' || transfer.status === 'cancelled'"
            @click="emit('transferComplete', transfer.id)"
            class="btn btn-ghost btn-xs text-gray-500 hover:text-gray-700"
            title="关闭"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>
      </div>
      
      <div class="space-y-2">
        <!-- 进度条 -->
        <div class="w-full bg-gray-200 rounded-full h-2">
          <div 
            class="bg-gradient-to-r from-blue-500 to-purple-600 h-2 rounded-full transition-all duration-300"
            :style="{ width: `${getProgressPercentage(transfer)}%` }"
          ></div>
        </div>
        
        <!-- 传输信息 -->
        <div class="flex justify-between text-xs text-gray-600">
          <span>{{ formatFileSize(transfer.transferred || 0) }} / {{ formatFileSize(transfer.total_size || 0) }}</span>
          <span>{{ getProgressPercentage(transfer) }}%</span>
        </div>
        
        <!-- 传输速度和状态 -->
        <div class="flex justify-between text-xs">
          <span class="text-gray-500">
            {{ transfer.speed ? formatSpeed(transfer.speed) : '计算中...' }}
          </span>
          <span 
            :class="[
              'font-medium',
              transfer.status === 'completed' ? 'text-green-600' :
              transfer.status === 'error' ? 'text-red-600' :
              transfer.status === 'cancelled' ? 'text-gray-500' :
              'text-blue-600'
            ]"
          >
            {{ getStatusText(transfer.status) }}
          </span>
        </div>
      </div>
    </div>
    
    <!-- 传输完成的通知 -->
    <div 
      v-if="completedTransfers.length > 0"
      class="card-elegant p-3 bg-green-50 border-green-200"
    >
      <div class="flex items-center space-x-2">
        <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
        </svg>
        <span class="text-sm text-green-800 font-medium">
          {{ completedTransfers.length }} 个文件传输完成
        </span>
        <button 
          @click="clearCompleted"
          class="btn btn-ghost btn-xs text-green-600 ml-auto"
        >
          清除
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

// Props
const props = defineProps<{
  transfers: any[];
}>();

// Emits
const emit = defineEmits<{
  transferComplete: [id: string];
}>();

// 计算属性
const completedTransfers = computed(() => {
  return props.transfers.filter(t => t.status === 'completed');
});

// 活跃的传输（正在进行或等待中的）
const activeTransfers = computed(() => {
  return props.transfers.filter(t =>
    t.status === 'transferring' ||
    t.status === 'pending' ||
    t.status === 'completed' ||
    t.status === 'error' ||
    t.status === 'cancelled'
  );
});

// 方法
const getProgressPercentage = (transfer: any) => {
  if (!transfer.total_size || transfer.total_size === 0) return 0;
  return Math.round((transfer.transferred / transfer.total_size) * 100);
};

const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

const formatSpeed = (bytesPerSecond: number) => {
  return formatFileSize(bytesPerSecond) + '/s';
};

const getStatusText = (status: string) => {
  const statusMap: Record<string, string> = {
    'pending': '等待中',
    'transferring': '传输中',
    'completed': '已完成',
    'error': '传输失败',
    'cancelled': '已取消'
  };
  return statusMap[status] || status;
};

const cancelTransfer = async (transferId: string) => {
  try {
    // 调用后端取消传输的方法
    await invoke('cancel_transfer', { transferId });
    console.log('传输已取消:', transferId);

    // 发送完成事件，让App.vue清理传输记录
    emit('transferComplete', transferId);
  } catch (err) {
    console.error('取消传输失败:', err);
    // 即使取消失败，也从UI中移除
    emit('transferComplete', transferId);
  }
};

const clearCompleted = () => {
  completedTransfers.value.forEach(transfer => {
    emit('transferComplete', transfer.id);
  });
};
</script>
