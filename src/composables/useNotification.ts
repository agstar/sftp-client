import { ref, reactive } from 'vue';

export interface Notification {
  id: string;
  type: 'success' | 'error' | 'warning' | 'info';
  title: string;
  message?: string;
  duration?: number;
  persistent?: boolean;
}

const notifications = ref<Notification[]>([]);

export function useNotification() {
  const addNotification = (notification: Omit<Notification, 'id'>) => {
    const id = `notification_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
    const newNotification: Notification = {
      id,
      duration: 1000,
      persistent: false,
      ...notification,
    };

    notifications.value.push(newNotification);

    // 自动移除非持久化通知
    if (!newNotification.persistent && newNotification.duration) {
      setTimeout(() => {
        removeNotification(id);
      }, newNotification.duration);
    }

    return id;
  };

  const updateNotification = (id: string, updates: Partial<Omit<Notification, 'id'>>) => {
    const index = notifications.value.findIndex(n => n.id === id);
    if (index > -1) {
      notifications.value[index] = { ...notifications.value[index], ...updates };
      return true;
    }
    return false;
  };

  const removeNotification = (id: string) => {
    const index = notifications.value.findIndex(n => n.id === id);
    if (index > -1) {
      notifications.value.splice(index, 1);
    }
  };

  const clearAll = () => {
    notifications.value = [];
  };

  // 便捷方法
  const success = (title: string, message?: string, options?: Partial<Notification>) => {
    return addNotification({ type: 'success', title, message, ...options });
  };

  const error = (title: string, message?: string, options?: Partial<Notification>) => {
    return addNotification({ type: 'error', title, message, ...options });
  };

  const warning = (title: string, message?: string, options?: Partial<Notification>) => {
    return addNotification({ type: 'warning', title, message, ...options });
  };

  const info = (title: string, message?: string, options?: Partial<Notification>) => {
    return addNotification({ type: 'info', title, message, ...options });
  };

  // 创建或更新持久化通知
  const createOrUpdatePersistent = (
    key: string,
    notification: Omit<Notification, 'id' | 'persistent'>
  ) => {
    // 查找是否已存在完全匹配 key 的通知
    const existingIndex = notifications.value.findIndex(n => n.id === key);

    if (existingIndex > -1) {
      // 更新现有通知
      notifications.value[existingIndex] = {
        ...notifications.value[existingIndex],
        ...notification,
        persistent: true
      };
      return notifications.value[existingIndex].id;
    } else {
      // 创建新的持久化通知，使用传入的 key 作为 id
      const id = key;
      const newNotification: Notification = {
        id,
        persistent: false,
        duration: 1000,
        ...notification,
      };
      notifications.value.push(newNotification);
      return id;
    }
  };

  return {
    notifications,
    addNotification,
    updateNotification,
    removeNotification,
    clearAll,
    createOrUpdatePersistent,
    success,
    error,
    warning,
    info,
  };
}

