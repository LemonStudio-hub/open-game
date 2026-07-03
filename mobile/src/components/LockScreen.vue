<script setup lang="ts">
import StatusBar from './StatusBar.vue'
import TimeDisplay from './TimeDisplay.vue'
import BottomActions from './BottomActions.vue'
import SwipeIndicator from './SwipeIndicator.vue'
import { ref, onMounted, onUnmounted } from 'vue'

const emit = defineEmits<{
  unlock: []
}>()

const startY = ref(0)
const currentY = ref(0)
const isDragging = ref(false)
const dragProgress = ref(0)

const handleTouchStart = (e: TouchEvent) => {
  startY.value = e.touches[0].clientY
  currentY.value = startY.value
  isDragging.value = true
}

const handleTouchMove = (e: TouchEvent) => {
  if (!isDragging.value) return
  currentY.value = e.touches[0].clientY
  const diff = startY.value - currentY.value
  dragProgress.value = Math.min(Math.max(diff / 150, 0), 1)
}

const handleTouchEnd = () => {
  if (!isDragging.value) return
  isDragging.value = false
  if (startY.value - currentY.value > 80) {
    emit('unlock')
  }
  dragProgress.value = 0
}

const handleMouseDown = (e: MouseEvent) => {
  startY.value = e.clientY
  currentY.value = startY.value
  isDragging.value = true
}

const handleMouseMove = (e: MouseEvent) => {
  if (!isDragging.value) return
  currentY.value = e.clientY
  const diff = startY.value - currentY.value
  dragProgress.value = Math.min(Math.max(diff / 150, 0), 1)
}

const handleMouseUp = () => {
  if (!isDragging.value) return
  isDragging.value = false
  if (startY.value - currentY.value > 80) {
    emit('unlock')
  }
  dragProgress.value = 0
}

onMounted(() => {
  window.addEventListener('mousemove', handleMouseMove)
  window.addEventListener('mouseup', handleMouseUp)
})

onUnmounted(() => {
  window.removeEventListener('mousemove', handleMouseMove)
  window.removeEventListener('mouseup', handleMouseUp)
})
</script>

<template>
  <div
    class="lock-screen"
    :class="{ dragging: isDragging }"
    @touchstart="handleTouchStart"
    @touchmove="handleTouchMove"
    @touchend="handleTouchEnd"
    @mousedown="handleMouseDown"
  >
    <div
      class="content"
      :style="{
        transform: `translateY(${-dragProgress * 30}px)`,
        opacity: 1 - dragProgress * 0.2,
      }"
    >
      <StatusBar />
      <div class="main">
        <TimeDisplay />
      </div>
      <div class="bottom">
        <BottomActions />
        <SwipeIndicator />
      </div>
    </div>
  </div>
</template>

<style scoped>
.lock-screen {
  width: 100%;
  height: 100%;
  position: relative;
  overflow: hidden;
  user-select: none;
  cursor: grab;
  background: var(--color-bg);
  animation: breathe 8s ease-in-out infinite;
}

@keyframes breathe {
  0%, 100% { background-color: #faf9f6; }
  50% { background-color: #f5f2ec; }
}

.lock-screen.dragging {
  cursor: grabbing;
}

.content {
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  transition: transform 0.1s ease-out, opacity 0.1s ease-out;
}

.main {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 20px;
}

.bottom {
  display: flex;
  flex-direction: column;
  gap: 16px;
  padding-bottom: env(safe-area-inset-bottom, 24px);
  animation: fadeInUp 0.6s ease 0.2s both;
}

@media (min-width: 768px) {
  .bottom {
    gap: 20px;
    padding-bottom: 40px;
  }
}
</style>
