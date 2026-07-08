<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const eventKeys = ['1993', '1993-2018', '2018', '2032', '2035'] as const
</script>

<template>
  <section id="history" ref="sectionRef" class="section history">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('history.label') }}</span>
        <h2 class="section-title">{{ t('history.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          {{ t('history.subtitle') }}
        </p>
      </div>

      <div class="timeline">
        <div class="timeline-line"></div>

        <div
          v-for="(key, index) in eventKeys"
          :key="key"
          class="timeline-item reveal"
          :class="{ highlight: key === '2018' || key === '2035', right: index % 2 !== 0 }"
        >
          <div class="timeline-dot">
            <div class="dot-inner"></div>
          </div>
          <div class="timeline-content">
            <span class="timeline-year">{{ key }}</span>
            <h3 class="timeline-title">{{ t(`history.events.${key}.title`) }}</h3>
            <p class="timeline-desc">{{ t(`history.events.${key}.description`) }}</p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.history {
  background: var(--color-bg-primary);
}

.timeline {
  position: relative;
  max-width: 800px;
  margin: 0 auto;
  padding: var(--space-xl) 0;
}

.timeline-line {
  position: absolute;
  left: 50%;
  top: 0;
  bottom: 0;
  width: 1px;
  background: linear-gradient(to bottom, transparent, var(--color-border-strong), var(--color-border-strong), transparent);
  transform: translateX(-50%);
}

.timeline-item {
  position: relative;
  display: flex;
  align-items: flex-start;
  margin-bottom: var(--space-3xl);
  padding-right: calc(50% + 30px);
}

.timeline-item.right {
  padding-right: 0;
  padding-left: calc(50% + 30px);
  flex-direction: row-reverse;
}

.timeline-dot {
  position: absolute;
  left: 50%;
  top: 0;
  width: 16px;
  height: 16px;
  transform: translateX(-50%);
  display: flex;
  align-items: center;
  justify-content: center;
}

.dot-inner {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: var(--color-accent-dark);
  border: 2px solid var(--color-border);
  transition: all var(--transition-base);
}

.timeline-item.highlight .dot-inner {
  background: var(--color-accent);
  box-shadow: 0 0 12px rgba(74, 232, 122, 0.4);
  width: 10px;
  height: 10px;
}

.timeline-content {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-xl);
  transition: all var(--transition-base);
  width: 100%;
}

.timeline-content:hover {
  border-color: var(--color-border-hover);
  box-shadow: var(--shadow-accent);
}

.timeline-year {
  font-family: var(--font-mono);
  font-size: 0.8rem;
  color: var(--color-accent);
  letter-spacing: 0.1em;
}

.timeline-title {
  font-size: 1.15rem;
  font-weight: 600;
  margin: var(--space-xs) 0 var(--space-sm);
  color: var(--color-text-primary);
}

.timeline-desc {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  line-height: 1.7;
}

@media (max-width: 768px) {
  .timeline-line {
    left: 16px;
  }

  .timeline-item,
  .timeline-item.right {
    padding-left: 42px;
    padding-right: 0;
    flex-direction: row;
    margin-bottom: var(--space-xl);
  }

  .timeline-dot {
    left: 16px;
  }

  .timeline-content {
    padding: var(--space-lg);
  }
}

@media (max-width: 480px) {
  .timeline-content {
    padding: var(--space-md);
  }

  .timeline-title {
    font-size: 1rem;
  }

  .timeline-desc {
    font-size: 0.85rem;
  }
}
</style>
