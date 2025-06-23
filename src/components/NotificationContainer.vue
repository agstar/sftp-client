<template>
  <div class="fixed top-4 right-4 z-50 space-y-2 max-w-sm">
    <TransitionGroup
      name="notification"
      tag="div"
      class="space-y-2"
    >
      <div
        v-for="notification in notifications"
        :key="notification.id"
        :class="[
          'card-elegant p-4 shadow-lg border-l-4 animate-slide-up',
          getNotificationClasses(notification.type)
        ]"
      >
        <div class="flex items-start space-x-3">
          <!-- 图标 -->
          <div class="flex-shrink-0 mt-0.5">
            <svg
              v-if="notification.type === 'success'"
              class="w-5 h-5 text-green-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            <svg
              v-else-if="notification.type === 'error'"
              class="w-5 h-5 text-red-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
            <svg
              v-else-if="notification.type === 'warning'"
              class="w-5 h-5 text-yellow-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-2.5L13.732 4c-.77-.833-1.964-.833-2.732 0L3.732 16.5c-.77.833.192 2.5 1.732 2.5z"/>
            </svg>
            <svg
              v-else
              class="w-5 h-5 text-blue-600"
              fill="none"
              stroke="currentColor"
              viewBox="0 0 24 24"
            >
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 16h-1v-4h-1m1-4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"/>
            </svg>
          </div>

          <!-- 内容 -->
          <div class="flex-1 min-w-0">
            <h4 :class="['font-medium text-sm', getTitleClasses(notification.type)]">
              {{ notification.title }}
            </h4>
            <p
              v-if="notification.message"
              class="text-sm text-gray-600 mt-1"
            >
              {{ notification.message }}
            </p>
          </div>

          <!-- 关闭按钮 -->
          <button
            @click="removeNotification(notification.id)"
            class="flex-shrink-0 text-gray-400 hover:text-gray-600 transition-colors"
          >
            <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"/>
            </svg>
          </button>
        </div>

        <!-- 进度条（用于显示自动关闭倒计时） -->
        <div
          v-if="!notification.persistent && notification.duration"
          class="mt-3 w-full bg-gray-200 rounded-full h-1"
        >
          <div
            class="h-1 rounded-full transition-all duration-100 ease-linear"
            :class="getProgressClasses(notification.type)"
            :style="{ 
              width: `${getProgressWidth(notification)}%`,
              animation: `shrink ${notification.duration}ms linear`
            }"
          ></div>
        </div>
      </div>
    </TransitionGroup>
  </div>
</template>

<script setup lang="ts">
import { computed, onMounted, onUnmounted } from 'vue';
import { useNotification } from '../composables/useNotification';

const { notifications, removeNotification } = useNotification();

// 计算进度宽度（这里简化处理，实际应该基于时间计算）
const getProgressWidth = (notification: any) => {
  return 100; // 简化处理，实际应该根据剩余时间计算
};

const getNotificationClasses = (type: string) => {
  const classes = {
    success: 'border-green-500 bg-green-50',
    error: 'border-red-500 bg-red-50',
    warning: 'border-yellow-500 bg-yellow-50',
    info: 'border-blue-500 bg-blue-50',
  };
  return classes[type as keyof typeof classes] || classes.info;
};

const getTitleClasses = (type: string) => {
  const classes = {
    success: 'text-green-800',
    error: 'text-red-800',
    warning: 'text-yellow-800',
    info: 'text-blue-800',
  };
  return classes[type as keyof typeof classes] || classes.info;
};

const getProgressClasses = (type: string) => {
  const classes = {
    success: 'bg-green-500',
    error: 'bg-red-500',
    warning: 'bg-yellow-500',
    info: 'bg-blue-500',
  };
  return classes[type as keyof typeof classes] || classes.info;
};
</script>

<style scoped>
/* 通知动画 */
.notification-enter-active,
.notification-leave-active {
  transition: all 0.3s ease;
}

.notification-enter-from {
  opacity: 0;
  transform: translateX(100%);
}

.notification-leave-to {
  opacity: 0;
  transform: translateX(100%);
}

.notification-move {
  transition: transform 0.3s ease;
}

/* 进度条收缩动画 */
@keyframes shrink {
  from {
    width: 100%;
  }
  to {
    width: 0%;
  }
}
</style>
