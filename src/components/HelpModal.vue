<template>
  <div v-if="isVisible" class="modal modal-open">
    <div class="modal-box max-w-2xl">
      <div class="flex items-center justify-between mb-6">
        <h3 class="font-bold text-xl bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
          帮助与快捷键
        </h3>
        <button @click="close" class="btn btn-ghost btn-sm">
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
          </svg>
        </button>
      </div>

      <div class="space-y-6">
        <!-- 快捷键部分 -->
        <div>
          <h4 class="font-semibold text-lg mb-3 text-gray-800">键盘快捷键</h4>
          <div class="space-y-2">
            <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <span class="text-gray-700">刷新文件列表</span>
              <kbd class="kbd kbd-sm">F5</kbd>
            </div>
            <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <span class="text-gray-700">返回连接管理</span>
              <kbd class="kbd kbd-sm">Esc</kbd>
            </div>
            <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <span class="text-gray-700">新建连接</span>
              <div class="space-x-1">
                <kbd class="kbd kbd-sm">Ctrl</kbd>
                <span>+</span>
                <kbd class="kbd kbd-sm">N</kbd>
              </div>
            </div>
            <div class="flex items-center justify-between p-3 bg-gray-50 rounded-lg">
              <span class="text-gray-700">显示帮助</span>
              <kbd class="kbd kbd-sm">F1</kbd>
            </div>
          </div>
        </div>

        <!-- 功能说明 -->
        <div>
          <h4 class="font-semibold text-lg mb-3 text-gray-800">功能说明</h4>
          <div class="space-y-3">
            <div class="p-4 border border-blue-200 rounded-lg bg-blue-50">
              <h5 class="font-medium text-blue-800 mb-2">连接管理</h5>
              <p class="text-sm text-blue-700">
                支持 SFTP 协议连接，可以保存多个连接配置，支持连接测试功能。
              </p>
            </div>
            
            <div class="p-4 border border-green-200 rounded-lg bg-green-50">
              <h5 class="font-medium text-green-800 mb-2">文件操作</h5>
              <p class="text-sm text-green-700">
                支持文件和文件夹的浏览、下载、上传、删除操作。双击文件夹进入，支持拖拽上传。
              </p>
            </div>
            
            <div class="p-4 border border-purple-200 rounded-lg bg-purple-50">
              <h5 class="font-medium text-purple-800 mb-2">传输管理</h5>
              <p class="text-sm text-purple-700">
                实时显示文件传输进度，支持多文件并发传输，可以取消正在进行的传输。
              </p>
            </div>
          </div>
        </div>

        <!-- 使用技巧 -->
        <div>
          <h4 class="font-semibold text-lg mb-3 text-gray-800">使用技巧</h4>
          <div class="space-y-2 text-sm text-gray-600">
            <div class="flex items-start space-x-2">
              <svg class="w-4 h-4 mt-0.5 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
              </svg>
              <span>双击文件夹可以快速进入</span>
            </div>
            <div class="flex items-start space-x-2">
              <svg class="w-4 h-4 mt-0.5 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
              </svg>
              <span>拖拽文件到上传区域可以快速上传</span>
            </div>
            <div class="flex items-start space-x-2">
              <svg class="w-4 h-4 mt-0.5 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
              </svg>
              <span>连接前可以先测试连接确保配置正确</span>
            </div>
            <div class="flex items-start space-x-2">
              <svg class="w-4 h-4 mt-0.5 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd"/>
              </svg>
              <span>使用面包屑导航可以快速跳转到上级目录</span>
            </div>
          </div>
        </div>
      </div>

      <div class="modal-action">
        <button @click="close" class="btn btn-primary">
          知道了
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Props
defineProps<{
  isVisible: boolean;
}>();

// Emits
const emit = defineEmits<{
  close: [];
}>();

const close = () => {
  emit('close');
};
</script>
