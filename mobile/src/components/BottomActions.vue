<script setup lang="ts">
import { ref } from 'vue'

const flashOn = ref(false)
const toggleFlash = () => { flashOn.value = !flashOn.value }
</script>

<template>
  <div class="bottom-actions">
    <button
      class="action-btn"
      :class="{ active: flashOn }"
      @click="toggleFlash"
      aria-label="Flashlight"
    >
      <svg viewBox="0 0 24 24" fill="currentColor">
        <path d="M6 2h12v6l-4 4v8c0 1.1-.9 2-2 2s-2-.9-2-2v-8L6 8V2zm2 2v3l4 4 4-4V4H8z"/>
      </svg>
    </button>

    <button class="action-btn" aria-label="Camera">
      <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
        <path d="M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z" stroke-linecap="round" stroke-linejoin="round"/>
        <circle cx="12" cy="13" r="4"/>
      </svg>
    </button>
  </div>
</template>

<style scoped>
.bottom-actions {
  display: flex;
  justify-content: space-between;
  padding: 0 48px;
  width: 100%;
  max-width: 360px;
  margin: 0 auto;
}

.action-btn {
  width: 50px;
  height: 50px;
  border-radius: 50%;
  background: var(--glass-bg);
  backdrop-filter: blur(12px);
  -webkit-backdrop-filter: blur(12px);
  border: 1px solid var(--color-border);
  color: var(--color-text);
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
  box-shadow: var(--shadow-sm);
}

.action-btn:hover {
  background: var(--glass-bg-hover);
  box-shadow: var(--shadow-md);
}

.action-btn:active {
  transform: scale(0.95);
}

.action-btn.active {
  background: var(--color-text);
  color: var(--color-bg);
  border-color: var(--color-text);
}

.action-btn svg {
  width: 20px;
  height: 20px;
}

@media (max-width: 380px) {
  .bottom-actions {
    padding: 0 32px;
  }
  .action-btn {
    width: 46px;
    height: 46px;
  }
}
</style>
