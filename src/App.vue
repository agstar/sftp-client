<script setup lang="ts">
import { ref, reactive, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import ConnectionManager from "./components/ConnectionManager.vue";
import FileExplorer from "./components/FileExplorer.vue";
import TransferProgress from "./components/TransferProgress.vue";
import NotificationContainer from "./components/NotificationContainer.vue";
import HelpModal from "./components/HelpModal.vue";
import { useNotification } from "./composables/useNotification";
import { useKeyboardShortcuts } from "./composables/useKeyboardShortcuts";

// 通知系统
const { success, error, warning, info } = useNotification();

// 应用状态
const appState = reactive({
  currentView: 'connections', // 'connections' | 'explorer'
  activeConnection: null as any,
  isConnecting: false,
  connections: [] as any[],
  transfers: [] as any[]
});

const showHelp = ref(false);

// 连接管理
const handleConnectionSuccess = (connection: any) => {
  appState.activeConnection = connection;
  appState.currentView = 'explorer';
  appState.connections.push(connection);
  success('连接成功', `已成功连接到 ${connection.name}`);
};

const handleDisconnect = async () => {
  if (appState.activeConnection) {
    try {
      await invoke('disconnect_sftp', { connectionId: appState.activeConnection.id });
      const connectionName = appState.activeConnection.name;
      appState.activeConnection = null;
      appState.currentView = 'connections';
      info('连接已断开', `已断开与 ${connectionName} 的连接`);
    } catch (err) {
      error('断开连接失败', err as string);
    }
  }
};

// 切换视图
const switchToConnections = () => {
  appState.currentView = 'connections';
};

const switchToExplorer = () => {
  if (appState.activeConnection) {
    appState.currentView = 'explorer';
  }
};

// 传输管理
const handleTransferStart = (transfer: any) => {
  appState.transfers.push(transfer);
  // 移除这里的通知，因为文件操作组件已经有持久化通知了
  // info('开始传输', `正在传输文件: ${transfer.filename}`);
};

const handleTransferUpdate = (transfer: any) => {
  const index = appState.transfers.findIndex(t => t.id === transfer.id);
  if (index > -1) {
    // 更新现有传输项
    appState.transfers[index] = transfer;
  }
};

const handleTransferComplete = (transferId: string) => {
  const transferIndex = appState.transfers.findIndex(t => t.id === transferId);
  if (transferIndex > -1) {
    const transfer = appState.transfers[transferIndex];
    appState.transfers.splice(transferIndex, 1);
    // 移除这里的通知，因为文件操作组件已经有持久化通知了
    // if (transfer.status === 'completed') {
    //   success('传输完成', `文件 ${transfer.filename} 传输完成`);
    // }
  }
};

// 键盘快捷键
useKeyboardShortcuts([
  {
    key: 'F5',
    callback: () => {
      if (appState.currentView === 'explorer' && appState.activeConnection) {
        // 刷新文件列表的逻辑需要通过事件或 ref 来实现
        info('刷新', '按 F5 刷新文件列表');
      }
    },
    description: '刷新文件列表'
  },
  {
    key: 'Escape',
    callback: () => {
      if (appState.activeConnection) {
        switchToConnections();
      }
    },
    description: '返回连接管理'
  },
  {
    key: 'n',
    ctrl: true,
    callback: () => {
      if (appState.currentView === 'connections') {
        // 聚焦到连接表单
        info('快捷键', 'Ctrl+N 新建连接');
      }
    },
    description: '新建连接'
  },
  {
    key: 'F1',
    callback: () => {
      showHelp.value = true;
    },
    description: '显示帮助'
  }
]);

onMounted(() => {
  // 初始化应用
  info('欢迎使用', 'SFTP 客户端已启动，请连接到服务器开始使用');
});
</script>

<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50">
    <!-- 顶部导航栏 -->
    <header class="card-elegant mx-4 mt-4 p-4 animate-fade-in">
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <div class="flex items-center space-x-2">
            <svg class="w-8 h-8 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                    d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"/>
            </svg>
            <h1 class="text-2xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
              SFTP 客户端
            </h1>
          </div>
        </div>

        <div class="flex items-center space-x-3">
          <!-- 导航按钮 -->
          <div class="btn-group">
            <button
              @click="switchToConnections"
              :class="['btn btn-sm', appState.currentView === 'connections' ? 'btn-primary' : 'btn-ghost']"
            >
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M8.111 16.404a5.5 5.5 0 017.778 0M12 20h.01m-7.08-7.071c3.904-3.905 10.236-3.905 14.141 0M1.394 9.393c5.857-5.857 15.355-5.857 21.213 0"/>
              </svg>
              连接管理
            </button>
            <button
              @click="switchToExplorer"
              :disabled="!appState.activeConnection"
              :class="['btn btn-sm', appState.currentView === 'explorer' ? 'btn-primary' : 'btn-ghost']"
            >
              <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2H5a2 2 0 00-2-2z"/>
              </svg>
              文件浏览
            </button>
          </div>

          <!-- 连接状态和帮助按钮 -->
          <div class="flex items-center space-x-3">
            <button @click="showHelp = true" class="btn btn-ghost btn-sm" title="帮助 (F1)">
              <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.228 9c.549-1.165 2.03-2 3.772-2 2.21 0 4 1.343 4 3 0 1.4-1.278 2.575-3.006 2.907-.542.104-.994.54-.994 1.093m0 3h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
              </svg>
            </button>

            <div v-if="appState.activeConnection" class="flex items-center space-x-2 text-sm">
              <div class="w-2 h-2 bg-green-500 rounded-full animate-pulse"></div>
              <span class="text-gray-600">{{ appState.activeConnection.name }}</span>
              <button @click="handleDisconnect" class="btn btn-ghost btn-xs text-red-500">
                断开
              </button>
            </div>
          </div>
        </div>
      </div>
    </header>

    <!-- 主内容区域 -->
    <main class="p-4">
      <div class="animate-slide-up">
        <!-- 连接管理视图 -->
        <ConnectionManager
          v-if="appState.currentView === 'connections'"
          @connection-success="handleConnectionSuccess"
          :is-connecting="appState.isConnecting"
        />

        <!-- 文件浏览视图 -->
        <FileExplorer
          v-else-if="appState.currentView === 'explorer' && appState.activeConnection"
          :connection="appState.activeConnection"
          @transfer-start="handleTransferStart"
          @transfer-update="handleTransferUpdate"
          @transfer-complete="handleTransferComplete"
        />
      </div>
    </main>

    <!-- 传输进度面板 -->
    <TransferProgress
      v-if="appState.transfers.length > 0"
      :transfers="appState.transfers"
      @transfer-complete="handleTransferComplete"
    />

    <!-- 通知容器 -->
    <NotificationContainer />

    <!-- 帮助模态框 -->
    <HelpModal
      :is-visible="showHelp"
      @close="showHelp = false"
    />
  </div>
</template>

