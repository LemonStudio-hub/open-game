<script setup lang="ts">
</script>

<template>
  <div class="swipe-indicator">
    <div class="bar"></div>
  </div>
</template>

<style scoped>
.swipe-indicator {
  display: flex;
  justify-content: center;
  padding-bottom: 8px;
}

.bar {
  width: 134px;
  height: 4px;
  background: var(--color-text-muted);
  border-radius: 2px;
  position: relative;
  overflow: hidden;
  opacity: 0.5;
}

.bar::after {
  content: '';
  position: absolute;
  top: 0;
  left: -100%;
  width: 100%;
  height: 100%;
  background: linear-gradient(90deg, transparent, rgba(0, 0, 0, 0.2), transparent);
  animation: shimmer 2.5s ease-in-out infinite;
}

@media (min-width: 768px) {
  .bar {
    width: 160px;
    height: 5px;
  }
}
</style>
