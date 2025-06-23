import { ref, reactive } from 'vue';

export interface SavedConnection {
  id: string;
  name: string;
  host: string;
  port: number;
  username: string;
  password?: string; // 可选，用户可以选择是否保存密码
  savePassword: boolean;
  createdAt: string;
  lastUsed?: string;
  description?: string;
}

const STORAGE_KEY = 'sftp_saved_connections';

// 响应式的已保存连接列表
const savedConnections = ref<SavedConnection[]>([]);

export function useConnectionStorage() {
  // 从本地存储加载连接配置
  const loadConnections = () => {
    try {
      const stored = localStorage.getItem(STORAGE_KEY);
      if (stored) {
        const connections = JSON.parse(stored) as SavedConnection[];
        savedConnections.value = connections.sort((a, b) => {
          // 按最后使用时间排序，未使用的按创建时间排序
          const aTime = a.lastUsed || a.createdAt;
          const bTime = b.lastUsed || b.createdAt;
          return new Date(bTime).getTime() - new Date(aTime).getTime();
        });
      }
    } catch (error) {
      console.error('加载连接配置失败:', error);
      savedConnections.value = [];
    }
  };

  // 保存连接配置到本地存储
  const saveConnections = () => {
    try {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(savedConnections.value));
    } catch (error) {
      console.error('保存连接配置失败:', error);
      throw new Error('保存连接配置失败');
    }
  };

  // 添加新的连接配置
  const saveConnection = (connection: Omit<SavedConnection, 'id' | 'createdAt'>) => {
    const id = `conn_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    const newConnection: SavedConnection = {
      id,
      createdAt: new Date().toISOString(),
      ...connection,
    };

    // 检查是否已存在相同的连接（基于host、port、username）
    const existingIndex = savedConnections.value.findIndex(
      conn => conn.host === connection.host && 
              conn.port === connection.port && 
              conn.username === connection.username
    );

    if (existingIndex > -1) {
      // 更新现有连接
      savedConnections.value[existingIndex] = {
        ...savedConnections.value[existingIndex],
        ...newConnection,
        id: savedConnections.value[existingIndex].id, // 保持原有ID
        createdAt: savedConnections.value[existingIndex].createdAt, // 保持原有创建时间
      };
    } else {
      // 添加新连接
      savedConnections.value.unshift(newConnection);
    }

    saveConnections();
    return newConnection.id;
  };

  // 更新连接配置
  const updateConnection = (id: string, updates: Partial<SavedConnection>) => {
    const index = savedConnections.value.findIndex(conn => conn.id === id);
    if (index > -1) {
      savedConnections.value[index] = {
        ...savedConnections.value[index],
        ...updates,
      };
      saveConnections();
      return true;
    }
    return false;
  };

  // 删除连接配置
  const deleteConnection = (id: string) => {
    const index = savedConnections.value.findIndex(conn => conn.id === id);
    if (index > -1) {
      savedConnections.value.splice(index, 1);
      saveConnections();
      return true;
    }
    return false;
  };

  // 获取连接配置
  const getConnection = (id: string) => {
    return savedConnections.value.find(conn => conn.id === id);
  };

  // 更新最后使用时间
  const updateLastUsed = (id: string) => {
    const index = savedConnections.value.findIndex(conn => conn.id === id);
    if (index > -1) {
      savedConnections.value[index].lastUsed = new Date().toISOString();
      saveConnections();
      
      // 重新排序
      savedConnections.value.sort((a, b) => {
        const aTime = a.lastUsed || a.createdAt;
        const bTime = b.lastUsed || b.createdAt;
        return new Date(bTime).getTime() - new Date(aTime).getTime();
      });
    }
  };

  // 清空所有连接配置
  const clearAllConnections = () => {
    savedConnections.value = [];
    saveConnections();
  };

  // 导出连接配置
  const exportConnections = () => {
    const dataStr = JSON.stringify(savedConnections.value, null, 2);
    const dataBlob = new Blob([dataStr], { type: 'application/json' });
    const url = URL.createObjectURL(dataBlob);
    
    const link = document.createElement('a');
    link.href = url;
    link.download = `sftp_connections_${new Date().toISOString().split('T')[0]}.json`;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(url);
  };

  // 导入连接配置
  const importConnections = (file: File): Promise<number> => {
    return new Promise((resolve, reject) => {
      const reader = new FileReader();
      reader.onload = (e) => {
        try {
          const content = e.target?.result as string;
          const importedConnections = JSON.parse(content) as SavedConnection[];
          
          let addedCount = 0;
          importedConnections.forEach(conn => {
            // 检查是否已存在
            const exists = savedConnections.value.some(
              existing => existing.host === conn.host && 
                         existing.port === conn.port && 
                         existing.username === conn.username
            );
            
            if (!exists) {
              const newId = `conn_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
              savedConnections.value.push({
                ...conn,
                id: newId,
                createdAt: conn.createdAt || new Date().toISOString(),
              });
              addedCount++;
            }
          });
          
          saveConnections();
          resolve(addedCount);
        } catch (error) {
          reject(new Error('导入文件格式错误'));
        }
      };
      reader.onerror = () => reject(new Error('读取文件失败'));
      reader.readAsText(file);
    });
  };

  // 获取连接统计信息
  const getConnectionStats = () => {
    const total = savedConnections.value.length;
    const withPassword = savedConnections.value.filter(conn => conn.savePassword).length;
    const recentlyUsed = savedConnections.value.filter(conn => {
      if (!conn.lastUsed) return false;
      const lastUsed = new Date(conn.lastUsed);
      const weekAgo = new Date();
      weekAgo.setDate(weekAgo.getDate() - 7);
      return lastUsed > weekAgo;
    }).length;

    return {
      total,
      withPassword,
      recentlyUsed,
    };
  };

  // 初始化时加载连接配置
  loadConnections();

  return {
    savedConnections,
    loadConnections,
    saveConnection,
    updateConnection,
    deleteConnection,
    getConnection,
    updateLastUsed,
    clearAllConnections,
    exportConnections,
    importConnections,
    getConnectionStats,
  };
}
