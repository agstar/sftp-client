<template>
  <div class="max-w-7xl mx-auto space-y-6">
    <!-- 路径导航 -->
    <div class="card-elegant p-4 animate-fade-in">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-2">
          <button 
            @click="navigateUp"
            :disabled="currentPath === '/'"
            class="btn btn-ghost btn-sm"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"/>
            </svg>
            返回上级
          </button>
          
          <div class="breadcrumbs text-sm">
            <ul>
              <li v-for="(segment, index) in pathSegments" :key="index">
                <button
                  @click="navigateToPath(getPathUpTo(index))"
                  class="hover:text-blue-600 transition-colors"
                >
                  {{ segment || '根目录' }}
                </button>
              </li>
            </ul>
          </div>

          <!-- 调试信息 -->
          <div class="text-xs text-gray-500 mt-1">
            当前路径: {{ currentPath }} | 文件数量: {{ files.length }}
          </div>
        </div>

        <div class="flex items-center space-x-2">
          <button @click="refreshDirectory" class="btn btn-ghost btn-sm">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"/>
            </svg>
            刷新
          </button>

          <button @click="diagnoseConnection" class="btn btn-ghost btn-sm">
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            诊断
          </button>
          
          <button @click="showCreateDialog = true" class="btn btn-primary btn-sm">
            <svg class="w-4 h-4 mr-1" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 6v6m0 0v6m0-6h6m-6 0H6"/>
            </svg>
            新建文件夹
          </button>
        </div>
      </div>
    </div>

    <!-- 文件列表 -->
    <div class="card-elegant animate-scale-in">
      <div class="overflow-x-auto">
        <table class="table table-zebra w-full">
          <thead>
            <tr class="bg-gradient-to-r from-blue-50 to-purple-50">
              <th class="font-semibold text-gray-700">名称</th>
              <th class="font-semibold text-gray-700">大小</th>
              <th class="font-semibold text-gray-700">权限</th>
              <th class="font-semibold text-gray-700">操作</th>
            </tr>
          </thead>
          <tbody>
            <tr v-if="isLoading">
              <td colspan="4" class="text-center py-8">
                <div class="flex items-center justify-center space-x-2">
                  <svg class="animate-spin w-5 h-5 text-blue-600" fill="none" viewBox="0 0 24 24">
                    <circle class="opacity-25" cx="12" cy="12" r="10" stroke="currentColor" stroke-width="4"></circle>
                    <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
                  </svg>
                  <span class="text-gray-600">加载中...</span>
                </div>
              </td>
            </tr>
            
            <tr v-else-if="files.length === 0">
              <td colspan="4" class="text-center py-8">
                <div class="space-y-3">
                  <svg class="w-12 h-12 mx-auto text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z"/>
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 9l6 6m0-6l-6 6"/>
                  </svg>
                  <div>
                    <p class="text-gray-500 font-medium">此目录为空</p>
                    <p class="text-sm text-gray-400 mt-1">可能的原因：</p>
                    <ul class="text-xs text-gray-400 mt-2 space-y-1">
                      <li>• 目录确实为空</li>
                      <li>• 权限不足，无法读取文件</li>
                      <li>• 连接问题</li>
                    </ul>
                    <button @click="diagnoseConnection" class="btn btn-sm btn-outline mt-3">
                      诊断连接
                    </button>
                  </div>
                </div>
              </td>
            </tr>
            
            <tr 
              v-else
              v-for="file in files" 
              :key="file.path"
              class="hover:bg-blue-50 transition-colors cursor-pointer"
              @dblclick="handleFileDoubleClick(file)"
            >
              <td class="flex items-center space-x-3">
                <div class="flex-shrink-0">
                  <svg v-if="file.is_dir" class="w-5 h-5 text-blue-500" fill="currentColor" viewBox="0 0 20 20">
                    <path d="M2 6a2 2 0 012-2h5l2 2h5a2 2 0 012 2v6a2 2 0 01-2 2H4a2 2 0 01-2-2V6z"/>
                  </svg>
                  <svg v-else class="w-5 h-5 text-gray-500" fill="currentColor" viewBox="0 0 20 20">
                    <path fill-rule="evenodd" d="M4 4a2 2 0 012-2h4.586A2 2 0 0112 2.586L15.414 6A2 2 0 0116 7.414V16a2 2 0 01-2 2H6a2 2 0 01-2-2V4zm2 6a1 1 0 011-1h6a1 1 0 110 2H7a1 1 0 01-1-1zm1 3a1 1 0 100 2h6a1 1 0 100-2H7z" clip-rule="evenodd"/>
                  </svg>
                </div>
                <div>
                  <div class="font-medium text-gray-900">{{ file.name }}</div>
                  <div class="text-sm text-gray-500">{{ file.path }}</div>
                  <!-- 调试信息 -->
                  <div class="text-xs text-red-500" v-if="file.name === '未知'">
                    警告: 文件名无效
                  </div>
                </div>
              </td>
              
              <td class="text-gray-600">
                {{ file.is_dir ? '-' : formatFileSize(file.size) }}
              </td>
              
              <td class="text-gray-600 font-mono text-sm">
                {{ file.permissions }}
              </td>
              
              <td>
                <div class="flex items-center space-x-2">
                  <button 
                    v-if="!file.is_dir"
                    @click="downloadFile(file)"
                    class="btn btn-ghost btn-xs text-green-600 hover:text-green-700"
                    title="下载"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 10v6m0 0l-3-3m3 3l3-3m2 8H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
                    </svg>
                  </button>
                  
                  <button 
                    @click="deleteFile(file)"
                    class="btn btn-ghost btn-xs text-red-600 hover:text-red-700"
                    title="删除"
                  >
                    <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                      <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"/>
                    </svg>
                  </button>
                </div>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>

    <!-- 拖拽上传区域 -->
    <div
      class="card-elegant p-8 border-2 border-dashed border-blue-300 hover:border-blue-500 transition-colors animate-slide-up"
      @dragover.prevent
      @drop.prevent="handleFileDrop"
      @click="triggerFileSelect"
    >
      <div class="text-center cursor-pointer">
        <svg class="w-12 h-12 mx-auto text-blue-500 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 16a4 4 0 01-.88-7.903A5 5 0 1115.9 6L16 6a5 5 0 011 9.9M15 13l-3-3m0 0l-3 3m3-3v12"/>
        </svg>
        <h3 class="text-lg font-semibold text-gray-800 mb-2">拖拽文件到此处上传</h3>
        <p class="text-gray-600">或者
          <span class="text-blue-600 hover:text-blue-700 font-medium">点击选择文件</span>
        </p>
      </div>

      <!-- 隐藏的文件输入 -->
      <input
        ref="fileInput"
        type="file"
        multiple
        class="hidden"
        @change="handleFileSelect"
      />
    </div>

    <!-- 新建文件夹对话框 -->
    <div v-if="showCreateDialog" class="modal modal-open">
      <div class="modal-box">
        <h3 class="font-bold text-lg mb-4">新建文件夹</h3>
        <div class="form-control">
          <label class="label">
            <span class="label-text">文件夹名称</span>
          </label>
          <input 
            v-model="newFolderName"
            type="text" 
            placeholder="输入文件夹名称"
            class="input input-bordered w-full"
            @keyup.enter="createFolder"
          />
        </div>
        <div class="modal-action">
          <button @click="showCreateDialog = false" class="btn btn-ghost">取消</button>
          <button @click="createFolder" class="btn btn-primary">创建</button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted, watch } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { openPath } from '@tauri-apps/plugin-opener';
import { useNotification } from '../composables/useNotification';

// Props
const props = defineProps<{
  connection: any;
}>();

// Emits
const emit = defineEmits<{
  transferStart: [transfer: any];
  transferUpdate: [transfer: any];
  transferComplete: [transferId: string];
}>();

// 通知系统
const { success, error, warning, info, createOrUpdatePersistent, removeNotification } = useNotification();

// 响应式数据
const currentPath = ref('/');
const files = ref<any[]>([]);
const isLoading = ref(false);
const showCreateDialog = ref(false);
const newFolderName = ref('');
const fileInput = ref<HTMLInputElement>();

// 传输状态管理
const activeTransfers = ref<Map<string, any>>(new Map());
let progressUnlisten: (() => void) | null = null;

// 计算属性
const pathSegments = computed(() => {
  if (currentPath.value === '/') {
    return [''];
  }
  return currentPath.value.split('/').filter(Boolean);
});

// 方法
const getPathUpTo = (index: number) => {
  if (index === 0 && pathSegments.value[0] === '') {
    return '/';
  }
  const segments = pathSegments.value.slice(0, index + 1);
  return '/' + segments.join('/');
};

const navigateUp = () => {
  console.log('返回上级目录，当前路径:', currentPath.value);

  // 检查是否已经在根目录
  if (currentPath.value === '/' || currentPath.value === '') {
    console.log('已在根目录，无法返回上级');
    return;
  }

  // 检查是否是 Windows 盘符根目录
  if (currentPath.value.match(/^[A-Z]:\/$/i) || currentPath.value.match(/^[A-Z]:$/i)) {
    console.log('在 Windows 盘符根目录，返回盘符列表');
    navigateToPath('/');
    return;
  }

  // 移除末尾的斜杠（如果有的话）
  let path = currentPath.value.endsWith('/') &&
             !currentPath.value.match(/^[A-Z]:\/$/i) &&
             currentPath.value !== '/'
    ? currentPath.value.slice(0, -1)
    : currentPath.value;

  // 找到最后一个斜杠的位置
  const lastSlashIndex = path.lastIndexOf('/');

  if (lastSlashIndex === 0) {
    // Unix 风格：根目录下的文件夹，返回根目录
    navigateToPath('/');
  } else if (lastSlashIndex > 0) {
    // 返回上一级目录
    const parentPath = path.substring(0, lastSlashIndex);

    // 检查是否返回到 Windows 盘符根目录
    if (parentPath.match(/^[A-Z]:$/i)) {
      navigateToPath(parentPath + '/');
    } else {
      navigateToPath(parentPath);
    }
  } else {
    // 没有斜杠，可能是特殊情况
    console.log('特殊路径格式，返回根目录');
    navigateToPath('/');
  }
};

const navigateToPath = (path: string) => {
  // 处理不同的路径格式
  let normalizedPath = path;

  console.log('导航到路径 - 原始:', path);

  // 检查是否是 Windows 盘符路径
  if (path.match(/^[A-Z]:$/i)) {
    // Windows 盘符，如 "C:"
    normalizedPath = path + '/';
    console.log('识别为 Windows 盘符:', normalizedPath);
  } else if (path.includes('\\')) {
    // Windows 路径，转换为正斜杠
    normalizedPath = path.replace(/\\/g, '/');
    console.log('Windows 路径转换:', normalizedPath);
  } else if (!normalizedPath.startsWith('/') && !normalizedPath.match(/^[A-Z]:/i)) {
    // Unix 风格路径，确保以 / 开头
    normalizedPath = '/' + normalizedPath;
  }

  // 移除末尾的斜杠（除非是根目录或 Windows 盘符根目录）
  if (normalizedPath !== '/' &&
      !normalizedPath.match(/^[A-Z]:\/$/i) &&
      normalizedPath.endsWith('/')) {
    normalizedPath = normalizedPath.slice(0, -1);
  }

  console.log('标准化路径:', normalizedPath);
  currentPath.value = normalizedPath;
  loadDirectory();
};

const loadDirectory = async () => {
  isLoading.value = true;
  console.log('加载目录:', currentPath.value, '连接ID:', props.connection.id);

  try {
    const result = await invoke('list_directory', {
      connectionId: props.connection.id,
      path: currentPath.value
    });
    files.value = result as any[];
    console.log('目录加载成功，文件数量:', files.value.length);
  } catch (err) {
    console.error('加载目录失败 - 路径:', currentPath.value, '错误:', err);
    error('加载目录失败', `路径: ${currentPath.value}, 错误: ${err}`);

    // 如果不是根目录，尝试返回上级目录
    if (currentPath.value !== '/') {
      warning('目录访问失败', '正在返回上级目录');
      navigateUp();
    }
  } finally {
    isLoading.value = false;
  }
};

const refreshDirectory = () => {
  loadDirectory();
};

// 诊断连接状态
const diagnoseConnection = async () => {
  try {
    info('开始诊断', '正在检查连接状态...');

    const result = await invoke('get_connection_info', {
      connectionId: props.connection.id
    });

    success('连接诊断', result as string);
    console.log('连接诊断结果:', result);

    // 同时尝试加载目录
    await loadDirectory();

  } catch (err) {
    error('连接诊断失败', err as string);
    console.error('连接诊断失败:', err);
  }
};

const handleFileDoubleClick = (file: any) => {
  if (file.is_dir) {
    console.log('双击文件夹详情:', {
      name: file.name,
      path: file.path,
      currentPath: currentPath.value
    });

    // 检查文件名是否有效
    if (!file.name || file.name === '未知') {
      error('无效文件名', '无法进入该文件夹，文件名无效');
      return;
    }

    // 优先使用 file.path，如果不可用则构建路径
    let newPath;

    if (file.path && file.path !== file.name) {
      // 使用服务器返回的完整路径
      newPath = file.path;
      console.log('使用服务器路径:', newPath);
    } else {
      // 构建新路径
      if (file.name.match(/^[A-Z]盘$/i)) {
        // Windows 盘符，如 "C盘"
        const driveLetter = file.name.charAt(0);
        newPath = driveLetter + ':';
        console.log('Windows 盘符路径:', newPath);
      } else if (currentPath.value === '/' || currentPath.value === '') {
        newPath = '/' + file.name;
      } else if (currentPath.value.match(/^[A-Z]:$/i)) {
        // 当前在 Windows 盘符根目录
        newPath = currentPath.value + '/' + file.name;
      } else {
        newPath = currentPath.value + '/' + file.name;
      }

      // 清理路径，移除重复的斜杠
      newPath = newPath.replace(/\/+/g, '/');
      console.log('构建的路径:', newPath);
    }

    console.log('双击文件夹:', file.name, '当前路径:', currentPath.value, '新路径:', newPath);
    navigateToPath(newPath);
  }
};

const downloadFile = async (file: any) => {
  const notificationKey = `download_${file.name}_${Date.now()}`;

  try {
    // 获取系统默认下载目录
    const downloadsPath = await invoke('get_downloads_directory') as string;
    console.log('下载目录:', downloadsPath);

    // 构建本地文件路径，处理文件名中的特殊字符
    const sanitizedFileName = file.name.replace(/[<>:"/\\|?*]/g, '_');
    const localPath = `${downloadsPath}${downloadsPath.includes('\\') ? '\\' : '/'}${sanitizedFileName}`;

    console.log('下载文件:', {
      remotePath: file.path,
      localPath: localPath,
      fileName: file.name,
      sanitizedFileName: sanitizedFileName
    });

    // 创建唯一的下载通知
    const notificationId = createOrUpdatePersistent(notificationKey, {
      type: 'info',
      title: '正在下载',
      message: `正在下载文件: ${file.name} (${formatFileSize(file.size)})`
    });

    console.log('创建下载通知:', notificationId);

    // 发送传输开始事件（用于传输面板显示）
    const transferId = `download_${Date.now()}`;
    const transferInfo = {
      id: transferId,
      filename: file.name,
      type: 'download',
      status: 'transferring',
      total_size: file.size,
      transferred: 0,
      speed: 0
    };

    // 存储传输信息以便进度更新
    activeTransfers.value.set(transferId, transferInfo);

    // 只在开始时发送一次传输事件
    emit('transferStart', transferInfo);

    // 使用带进度的下载函数
    const result = await invoke('download_file_with_progress', {
      connectionId: props.connection.id,
      remotePath: file.path,
      localPath,
      transferId
    });

    // 创建打开文件夹的函数
    const openFileFolder = async () => {
      try {
        // 获取文件所在目录
        const fileDir = localPath.substring(0, localPath.lastIndexOf(localPath.includes('\\') ? '\\' : '/'));
        await openPath(fileDir);
        console.log('已打开文件夹:', fileDir);
      } catch (err) {
        console.error('打开文件夹失败:', err);
        error('打开失败', '无法打开文件所在文件夹');
      }
    };

    // 更新为成功状态，添加打开文件夹按钮
    createOrUpdatePersistent(notificationKey, {
      type: 'success',
      title: '下载完成',
      message: `${result}\n保存位置: ${localPath}`,
      actions: [
        {
          label: '打开文件夹',
          action: openFileFolder,
          style: 'primary'
        }
      ]
    });

    // 传输完成状态会由进度监听器处理，这里不需要重复发送

    // 3秒后自动移除成功通知
    setTimeout(() => {
      console.log('尝试移除下载通知:', notificationId, '通知键:', notificationKey);
      removeNotification(notificationKey);
    }, 3000);

  } catch (err) {
    // 更新为错误状态
    createOrUpdatePersistent(notificationKey, {
      type: 'error',
      title: '下载失败',
      message: `文件 ${file.name} 下载失败: ${err}`
    });

    console.error('下载失败详情:', {
      file: file,
      error: err
    });
  }
};

const deleteFile = async (file: any) => {
  if (!confirm(`确定要删除 ${file.name} 吗？`)) return;

  const notificationKey = `delete_${file.name}`;

  try {
    // 创建删除通知
    const notificationId = createOrUpdatePersistent(notificationKey, {
      type: 'info',
      title: '正在删除',
      message: `正在删除 ${file.is_dir ? '文件夹' : '文件'}: ${file.name}`
    });

    await invoke('delete_file', {
      connectionId: props.connection.id,
      path: file.path,
      isDir: file.is_dir
    });

    // 更新为成功状态
    createOrUpdatePersistent(notificationKey, {
      type: 'success',
      title: '删除成功',
      message: `${file.is_dir ? '文件夹' : '文件'} ${file.name} 已删除`
    });

    await loadDirectory();

    // 1秒后移除通知
    setTimeout(() => {
      console.log('尝试移除删除通知:', notificationKey);
      removeNotification(notificationKey);
    }, 1000);

  } catch (err) {
    // 更新为错误状态
    createOrUpdatePersistent(notificationKey, {
      type: 'error',
      title: '删除失败',
      message: `删除 ${file.name} 失败: ${err}`
    });

    console.error('删除失败:', err);
  }
};

const createFolder = async () => {
  if (!newFolderName.value.trim()) {
    warning('名称不能为空', '请输入文件夹名称');
    return;
  }

  const notificationKey = `create_${newFolderName.value}`;

  try {
    const folderPath = `${currentPath.value}/${newFolderName.value}`.replace('//', '/');

    // 创建文件夹通知
    const notificationId = createOrUpdatePersistent(notificationKey, {
      type: 'info',
      title: '正在创建',
      message: `正在创建文件夹: ${newFolderName.value}`
    });

    await invoke('create_directory', {
      connectionId: props.connection.id,
      path: folderPath
    });

    // 更新为成功状态
    createOrUpdatePersistent(notificationKey, {
      type: 'success',
      title: '创建成功',
      message: `文件夹 ${newFolderName.value} 创建成功`
    });

    showCreateDialog.value = false;
    newFolderName.value = '';
    await loadDirectory();

    // 1秒后移除通知
    setTimeout(() => {
      console.log('尝试移除创建文件夹通知:', notificationKey);
      removeNotification(notificationKey);
    }, 1000);

  } catch (err) {
    // 更新为错误状态
    createOrUpdatePersistent(notificationKey, {
      type: 'error',
      title: '创建失败',
      message: `创建文件夹失败: ${err}`
    });

    console.error('创建文件夹失败:', err);
  }
};

// 触发文件选择
const triggerFileSelect = () => {
  fileInput.value?.click();
};

// 处理文件选择
const handleFileSelect = (event: Event) => {
  const target = event.target as HTMLInputElement;
  const selectedFiles = target.files;
  if (selectedFiles && selectedFiles.length > 0) {
    handleFileUpload(Array.from(selectedFiles));
  }
  // 清空输入，允许重复选择同一文件
  target.value = '';
};

// 处理拖拽文件
const handleFileDrop = async (event: DragEvent) => {
  const droppedFiles = event.dataTransfer?.files;
  if (droppedFiles && droppedFiles.length > 0) {
    handleFileUpload(Array.from(droppedFiles));
  }
};

// 处理文件上传
const handleFileUpload = async (fileList: File[]) => {
  const batchNotificationKey = 'upload_batch';

  // 创建批量上传通知
  createOrUpdatePersistent(batchNotificationKey, {
    type: 'info',
    title: '批量上传',
    message: `准备上传 ${fileList.length} 个文件`
  });

  let completedCount = 0;
  let failedCount = 0;

  for (const file of fileList) {
    try {
      await uploadFile(file);
      completedCount++;
    } catch (err) {
      failedCount++;
    }

    // 更新批量上传进度
    createOrUpdatePersistent(batchNotificationKey, {
      type: 'info',
      title: '批量上传进行中',
      message: `进度: ${completedCount + failedCount}/${fileList.length} (成功: ${completedCount}, 失败: ${failedCount})`
    });
  }

  // 最终结果
  const finalType = failedCount === 0 ? 'success' : (completedCount === 0 ? 'error' : 'warning');
  const finalTitle = failedCount === 0 ? '上传完成' : (completedCount === 0 ? '上传失败' : '上传部分完成');
  const finalMessage = `总计: ${fileList.length} 个文件，成功: ${completedCount}，失败: ${failedCount}`;

  createOrUpdatePersistent(batchNotificationKey, {
    type: finalType,
    title: finalTitle,
    message: finalMessage
  });

  // 1秒后移除批量通知
  setTimeout(() => {
    console.log('尝试移除批量上传通知:', batchNotificationKey);
    removeNotification(batchNotificationKey);
  }, 1000);
};

const uploadFile = async (file: File) => {
  const notificationKey = `upload_${file.name}_${Date.now()}`;

  try {
    const remotePath = `${currentPath.value}/${file.name}`.replace('//', '/');

    // 创建唯一的上传通知
    const notificationId = createOrUpdatePersistent(notificationKey, {
      type: 'info',
      title: '正在上传',
      message: `正在上传文件: ${file.name} (${formatFileSize(file.size)})`
    });

    console.log('创建上传通知:', notificationId);

    // 发送传输开始事件（仅用于传输面板显示，不产生额外通知）
    const transferId = `upload_${Date.now()}`;
    emit('transferStart', {
      id: transferId,
      filename: file.name,
      type: 'upload',
      status: 'transferring',
      total_size: file.size,
      transferred: 0
    });

    // 将文件转换为 ArrayBuffer 然后转为 base64
    const arrayBuffer = await file.arrayBuffer();
    const uint8Array = new Uint8Array(arrayBuffer);
    const base64Data = btoa(String.fromCharCode(...uint8Array));

    // 调用后端上传函数
    await invoke('upload_file_data', {
      connectionId: props.connection.id,
      remotePath,
      fileData: base64Data,
      fileName: file.name
    });

    // 更新为成功状态
    createOrUpdatePersistent(notificationKey, {
      type: 'success',
      title: '上传完成',
      message: `文件 ${file.name} 上传完成`
    });

    // 更新传输状态为完成
    emit('transferUpdate', {
      id: transferId,
      filename: file.name,
      type: 'upload',
      status: 'completed',
      total_size: file.size,
      transferred: file.size
    });

    // 刷新目录
    await loadDirectory();

    // 1秒后自动移除成功通知
    setTimeout(() => {
      console.log('尝试移除上传通知:', notificationId, '通知键:', notificationKey);
      removeNotification(notificationKey); // 使用 notificationKey 而不是 notificationId
    }, 1000);

  } catch (err) {
    // 更新为错误状态
    createOrUpdatePersistent(notificationKey, {
      type: 'error',
      title: '上传失败',
      message: `文件 ${file.name} 上传失败: ${err}`
    });

    console.error('上传失败:', err);
    throw err; // 重新抛出错误，让批量上传能够统计失败数量
  }
};

const formatFileSize = (bytes: number) => {
  if (bytes === 0) return '0 B';
  const k = 1024;
  const sizes = ['B', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
};

// 生命周期
onMounted(async () => {
  loadDirectory();

  // 监听下载进度事件
  progressUnlisten = await listen('download_progress', (event: any) => {
    const progressData = event.payload;
    console.log('收到下载进度:', progressData);

    if (progressData.transfer_id && activeTransfers.value.has(progressData.transfer_id)) {
      const transfer = activeTransfers.value.get(progressData.transfer_id);

      // 更新传输信息
      const updatedTransfer = {
        ...transfer,
        transferred: progressData.bytes_copied,
        total_size: progressData.total_size,
        progress: progressData.progress || 0
      };

      // 计算传输速度（简化版本）
      if (transfer.lastUpdate) {
        const timeDiff = Date.now() - transfer.lastUpdate;
        const bytesDiff = progressData.bytes_copied - (transfer.transferred || 0);
        if (timeDiff > 0) {
          updatedTransfer.speed = Math.round((bytesDiff / timeDiff) * 1000); // bytes/second
        }
      }
      updatedTransfer.lastUpdate = Date.now();

      activeTransfers.value.set(progressData.transfer_id, updatedTransfer);

      // 发送更新事件到传输面板
      emit('transferUpdate', updatedTransfer);

      // 处理取消状态
      if (progressData.cancelled) {
        updatedTransfer.status = 'cancelled';
        activeTransfers.value.set(progressData.transfer_id, updatedTransfer);
        emit('transferUpdate', updatedTransfer);
        
        // 更新通知
        const notificationKey = `download_${updatedTransfer.filename}_${progressData.transfer_id.split('_')[1]}`;
        createOrUpdatePersistent(notificationKey, {
          type: 'warning',
          title: '下载已取消',
          message: `文件 ${updatedTransfer.filename} 下载已取消`
        });
        
        // 2秒后移除通知和清理传输记录
        setTimeout(() => {
          removeNotification(notificationKey);
          activeTransfers.value.delete(progressData.transfer_id);
          emit('transferComplete', progressData.transfer_id);
        }, 2000);
      }

      // 如果下载完成，清理传输记录
      if (progressData.completed) {
        // 标记为完成状态
        updatedTransfer.status = 'completed';
        activeTransfers.value.set(progressData.transfer_id, updatedTransfer);
        emit('transferUpdate', updatedTransfer);

        // 立即发送完成事件，让传输弹窗能够正确处理完成状态
        setTimeout(() => {
          activeTransfers.value.delete(progressData.transfer_id);
          emit('transferComplete', progressData.transfer_id);
        }, 3000); // 3秒后清理，给用户足够时间看到完成状态
      }
    }
  });
});

onUnmounted(() => {
  // 清理事件监听器
  if (progressUnlisten) {
    progressUnlisten();
  }
});

// 监听连接变化
watch(() => props.connection, () => {
  currentPath.value = '/';
  loadDirectory();
});

// 添加取消传输方法
const cancelTransfer = async (transferId: string) => {
  try {
    const transfer = activeTransfers.value.get(transferId);
    if (!transfer) return;
    
    await invoke('cancel_transfer', { transferId });
    console.log('已发送取消请求:', transferId);
  } catch (err) {
    console.error('取消传输失败:', err);
  }
};

// 导出取消方法供传输面板使用
defineExpose({
  cancelTransfer
});
</script>


