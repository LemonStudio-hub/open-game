<script setup lang="ts">
import StatusBar from '../StatusBar.vue'
import { useSwipeGestures } from '../../composables/useSwipeGestures'

const emit = defineEmits<{
  'go-lock': []
  'show-cards': []
  'open-settings': []
  'open-camera': []
  'open-photos': []
}>()

const { targetRef, dragProgress, swipeDirection, isDragging } =
  useSwipeGestures({
    onSwipeDown: () => emit('go-lock'),
    onSwipeUp: () => emit('show-cards'),
  })

const handleSettingsClick = () => {
  emit('open-settings')
}

const handleCameraClick = () => {
  emit('open-camera')
}

const handlePhotosClick = () => {
  emit('open-photos')
}

// --- Home bar swipe-up gesture (lower threshold) ---
const { targetRef: barTargetRef } =
  useSwipeGestures({
    onSwipeUp: () => emit('go-lock'),
    threshold: 40,
  })
</script>

<template>
  <div
    :ref="targetRef"
    class="home-screen"
    :class="{ dragging: isDragging }"
  >
    <!-- Wallpaper -->
    <div class="wallpaper"></div>

    <!-- Content -->
    <div
      class="content"
      :style="{
        transform: `translateY(${swipeDirection === 'down' ? dragProgress * 30 : swipeDirection === 'up' ? -dragProgress * 30 : 0}px)`,
        opacity: 1 - dragProgress * 0.2,
      }"
    >
      <StatusBar />

      <div class="app-grid">
        <button class="app-icon" @click="handleCameraClick">
          <div class="icon-wrapper camera-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z" stroke-linecap="round" stroke-linejoin="round"/>
              <circle cx="12" cy="13" r="4"/>
            </svg>
          </div>
          <span class="app-name">Camera</span>
        </button>

        <button class="app-icon" @click="handlePhotosClick">
          <div class="icon-wrapper photos-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <rect x="3" y="3" width="18" height="18" rx="2" stroke-linecap="round" stroke-linejoin="round"/>
              <circle cx="8.5" cy="8.5" r="1.5"/>
              <path d="M21 15l-5-5L5 21" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <span class="app-name">Photos</span>
        </button>

        <button class="app-icon" @click="handleSettingsClick">
          <div class="icon-wrapper settings-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z" stroke-linecap="round" stroke-linejoin="round"/>
              <path d="M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" stroke-linecap="round" stroke-linejoin="round"/>
            </svg>
          </div>
          <span class="app-name">Settings</span>
        </button>
      </div>

      <!-- Home bar -->
      <div
        :ref="barTargetRef"
        class="home-bar-area"
      >
        <div class="home-bar"></div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-screen {
  width: 100%;
  height: 100%;
  position: relative;
  overflow: hidden;
  user-select: none;
  cursor: grab;
}

.home-screen.dragging {
  cursor: grabbing;
}

/* Warm white wallpaper */
.wallpaper {
  position: absolute;
  inset: 0;
  background: var(--color-bg);
  animation: breathe 8s ease-in-out infinite;
}

@keyframes breathe {
  0%, 100% { background-color: #faf9f6; }
  50% { background-color: #f5f2ec; }
}

.content {
  position: relative;
  z-index: 1;
  width: 100%;
  height: 100%;
  display: flex;
  flex-direction: column;
  transition: transform 0.1s ease-out, opacity 0.1s ease-out;
}

/* App Grid */
.app-grid {
  flex: 1;
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  grid-template-rows: repeat(6, auto);
  gap: 24px 16px;
  padding: 24px 32px;
  align-content: start;
}

.app-icon {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 6px;
  background: none;
  border: none;
  cursor: pointer;
  -webkit-tap-highlight-color: transparent;
  transition: transform 0.2s ease;
}

.app-icon:active {
  transform: scale(0.9);
}

.icon-wrapper {
  width: 60px;
  height: 60px;
  border-radius: 14px;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.06);
}

.camera-icon {
  background: linear-gradient(135deg, #e6e2dc 0%, #d8d3cc 100%);
  color: #5a5650;
}

.photos-icon {
  background: linear-gradient(135deg, #e2ddd6 0%, #d4cec6 100%);
  color: #6b6560;
}

.settings-icon {
  background: linear-gradient(135deg, #e8e6e1 0%, #d5d2cb 100%);
  color: #4a4744;
}

.icon-wrapper svg {
  width: 30px;
  height: 30px;
}

.app-name {
  font-size: 11px;
  font-weight: 500;
  color: var(--color-text);
  letter-spacing: 0.2px;
  max-width: 70px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

/* Home bar */
.home-bar-area {
  padding: 12px 0 calc(env(safe-area-inset-bottom, 12px) + 4px);
  display: flex;
  justify-content: center;
  cursor: grab;
}

.home-bar {
  width: 134px;
  height: 5px;
  border-radius: 3px;
  background: rgba(0, 0, 0, 0.15);
}

@media (min-width: 768px) {
  .icon-wrapper {
    width: 68px;
    height: 68px;
    border-radius: 16px;
  }

  .icon-wrapper svg {
    width: 34px;
    height: 34px;
  }

  .app-name {
    font-size: 12px;
  }
}
</style>
