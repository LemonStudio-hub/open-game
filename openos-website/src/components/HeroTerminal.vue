<template>
  <div class="terminal">
    <div class="terminal-header">
      <div class="terminal-dots">
        <span class="dot red"></span>
        <span class="dot yellow"></span>
        <span class="dot green"></span>
      </div>
      <div class="terminal-title">Terminal</div>
    </div>
    <div class="terminal-body">
      <div class="terminal-line" v-for="(line, index) in lines" :key="index" :class="{ visible: line.visible }">
        <span class="prompt">$</span>
        <span class="command">{{ line.text }}</span>
        <span v-if="line.typing" class="cursor">█</span>
      </div>
      <div class="scanline"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';

interface TerminalLine {
  text: string;
  typing: boolean;
  visible: boolean;
}

const lines = ref<TerminalLine[]>([
  { text: 'boot OpenOS', typing: false, visible: false },
  { text: 'Loading kernel...', typing: false, visible: false },
  { text: 'Initializing drivers...', typing: false, visible: false },
  { text: 'Starting services...', typing: false, visible: false },
  { text: 'System ready. Welcome to OpenOS!', typing: false, visible: false },
]);

onMounted(() => {
  let delay = 500;

  lines.value.forEach((line, index) => {
    setTimeout(() => {
      line.visible = true;
      line.typing = true;

      setTimeout(() => {
        line.typing = false;
      }, 300);
    }, delay + index * 800);
  });
});
</script>

<style scoped>
.terminal {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-xl);
  overflow: hidden;
  box-shadow: var(--shadow-xl);
  max-width: 600px;
  margin: 0 auto;
}

.terminal-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 0.75rem 1rem;
  background: var(--color-bg-tertiary);
  border-bottom: 1px solid var(--color-border);
}

.terminal-dots {
  display: flex;
  gap: 0.5rem;
}

.dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.dot.red { background: #ef4444; }
.dot.yellow { background: #eab308; }
.dot.green { background: #22c55e; }

.terminal-title {
  color: var(--color-text-muted);
  font-size: 0.75rem;
  font-family: var(--font-mono);
}

.terminal-body {
  padding: 1.5rem;
  min-height: 200px;
  position: relative;
  overflow: hidden;
}

.terminal-line {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.5rem;
  font-family: var(--font-mono);
  font-size: 0.875rem;
  opacity: 0;
  transform: translateY(10px);
  transition: all 0.3s ease;
}

.terminal-line.visible {
  opacity: 1;
  transform: translateY(0);
}

.prompt {
  color: var(--color-primary-light);
  font-weight: 600;
}

.command {
  color: var(--color-text-primary);
}

.cursor {
  color: var(--color-primary-light);
  animation: terminalBlink 1s infinite;
}

.scanline {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 4px;
  background: linear-gradient(90deg, transparent, rgba(99, 102, 241, 0.3), transparent);
  animation: scanline 3s linear infinite;
}

@keyframes terminalBlink {
  0%, 100% { opacity: 1; }
  50% { opacity: 0; }
}

@keyframes scanline {
  0% { transform: translateY(-100%); }
  100% { transform: translateY(200px); }
}
</style>
