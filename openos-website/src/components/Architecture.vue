<template>
  <section id="architecture" class="section">
    <div class="container">
      <div class="section-header">
        <h2 class="gradient-text">{{ t.architecture.title }}</h2>
        <p>{{ t.architecture.subtitle }}</p>
      </div>

      <div class="architecture-content">
        <div class="architecture-visual">
          <div
            v-for="(layer, index) in t.architecture.layers"
            :key="index"
            class="layer"
            :class="{ active: activeLayer === index }"
            @mouseenter="activeLayer = index"
            @mouseleave="activeLayer = null"
          >
            <div class="layer-content">
              <div class="layer-indicator"></div>
              <div class="layer-info">
                <h4>{{ layer.name }}</h4>
                <p>{{ layer.description }}</p>
              </div>
            </div>
            <div class="layer-glow"></div>
          </div>
        </div>

        <div class="architecture-description">
          <p>{{ t.architecture.description }}</p>
          <div class="architecture-stats">
            <div class="stat">
              <span class="stat-value">4</span>
              <span class="stat-label">Layers</span>
            </div>
            <div class="stat">
              <span class="stat-value">Zero</span>
              <span class="stat-label">Copy IPC</span>
            </div>
            <div class="stat">
              <span class="stat-value">100%</span>
              <span class="stat-label">Memory Safe</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useTranslations, type Locale } from '../i18n';

const props = defineProps<{
  locale: Locale;
}>();

const t = computed(() => useTranslations(props.locale));

const activeLayer = ref<number | null>(null);
</script>

<style scoped>
.section-header {
  text-align: center;
  margin-bottom: 4rem;
}

.section-header h2 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.section-header p {
  font-size: 1.125rem;
  color: var(--color-text-secondary);
}

.architecture-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 4rem;
  align-items: center;
}

.architecture-visual {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.layer {
  position: relative;
  padding: 1.5rem;
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  cursor: pointer;
  transition: all var(--transition-base);
  overflow: hidden;
}

.layer:hover,
.layer.active {
  background: var(--color-bg-card-hover);
  border-color: var(--color-primary);
  transform: translateX(10px);
}

.layer-content {
  display: flex;
  align-items: center;
  gap: 1rem;
  position: relative;
  z-index: 1;
}

.layer-indicator {
  width: 12px;
  height: 12px;
  background: var(--gradient-primary);
  border-radius: 50%;
  flex-shrink: 0;
}

.layer-info h4 {
  font-size: 1rem;
  margin-bottom: 0.25rem;
  color: var(--color-text-primary);
}

.layer-info p {
  font-size: 0.875rem;
  color: var(--color-text-secondary);
  margin: 0;
}

.layer-glow {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: var(--gradient-primary);
  opacity: 0;
  transition: opacity var(--transition-base);
  z-index: 0;
}

.layer:hover .layer-glow,
.layer.active .layer-glow {
  opacity: 0.05;
}

.architecture-description p {
  font-size: 1.125rem;
  color: var(--color-text-secondary);
  line-height: 1.8;
  margin-bottom: 2rem;
}

.architecture-stats {
  display: flex;
  gap: 2rem;
}

.stat {
  text-align: center;
}

.stat-value {
  display: block;
  font-size: 2rem;
  font-weight: 700;
  background: var(--gradient-primary);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
}

.stat-label {
  display: block;
  font-size: 0.875rem;
  color: var(--color-text-muted);
  margin-top: 0.25rem;
}

@media (max-width: 1024px) {
  .architecture-content {
    grid-template-columns: 1fr;
    gap: 2rem;
  }
}

@media (max-width: 640px) {
  .section-header h2 {
    font-size: 2rem;
  }

  .architecture-stats {
    flex-direction: column;
    gap: 1rem;
  }
}
</style>
