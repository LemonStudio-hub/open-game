<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const time = ref('')
let timer: ReturnType<typeof setInterval>

const updateTime = () => {
  const now = new Date()
  const h = now.getHours().toString().padStart(2, '0')
  const m = now.getMinutes().toString().padStart(2, '0')
  time.value = `${h}:${m}`
}

onMounted(() => {
  updateTime()
  timer = setInterval(updateTime, 1000)
})

onUnmounted(() => clearInterval(timer))
</script>

<template>
  <header class="status-bar">
    <span class="time">{{ time }}</span>
    <div class="island"></div>
    <div class="icons">
      <svg class="icon" viewBox="0 0 24 24" fill="currentColor">
        <path d="M1 9l2 2c4.97-4.97 13.03-4.97 18 0l2-2C16.93 2.93 7.08 2.93 1 9zm8 8l3 3 3-3c-1.65-1.66-4.34-1.66-6 0zm-4-4l2 2c2.76-2.76 7.24-2.76 10 0l2-2C15.14 9.14 8.87 9.14 5 13z"/>
      </svg>
      <div class="battery">
        <div class="battery-body">
          <div class="battery-fill"></div>
        </div>
        <div class="battery-cap"></div>
      </div>
    </div>
  </header>
</template>

<style scoped>
.status-bar {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 24px 12px;
  color: var(--color-text);
}

.time {
  font-size: 14px;
  font-weight: 600;
  min-width: 80px;
  letter-spacing: 0.3px;
  color: var(--color-text);
}

.island {
  width: 120px;
  height: 32px;
  background: rgba(255, 253, 245, 0.65);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  border-radius: 20px;
  border: 1px solid rgba(0, 0, 0, 0.06);
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.04);
}

.icons {
  display: flex;
  align-items: center;
  gap: 6px;
  min-width: 80px;
  justify-content: flex-end;
}

.icon {
  width: 14px;
  height: 14px;
  color: var(--color-text);
  opacity: 0.7;
}

.battery {
  display: flex;
  align-items: center;
  gap: 1px;
}

.battery-body {
  width: 20px;
  height: 10px;
  border: 1.5px solid var(--color-text-muted);
  border-radius: 2px;
  padding: 1.5px;
}

.battery-fill {
  width: 100%;
  height: 100%;
  background: var(--color-text-secondary);
  border-radius: 1px;
}

.battery-cap {
  width: 2px;
  height: 4px;
  background: var(--color-text-muted);
  border-radius: 0 1px 1px 0;
}

@media (max-width: 380px) {
  .status-bar {
    padding: 12px 16px 8px;
  }
}
</style>
