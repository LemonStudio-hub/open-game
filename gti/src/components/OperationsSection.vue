<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'
import { operations } from '../data/operations'

const { t } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const expandedOp = ref<string | null>(null)

const statusColors: Record<string, string> = {
  success: '#4ae87a',
  failure: '#e8534a',
  partial: '#e8a84a',
}

const statusIcons: Record<string, string> = {
  success: 'M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z',
  failure: 'M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z',
  partial: 'M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
}

const toggleOp = (key: string) => {
  expandedOp.value = expandedOp.value === key ? null : key
}

const uniqueYears = [...new Set(operations.map(op => op.year))]
</script>

<template>
  <section id="operations" ref="sectionRef" class="section operations">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('operations.label') }}</span>
        <h2 class="section-title">{{ t('operations.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">{{ t('operations.subtitle') }}</p>
      </div>

      <!-- Timeline -->
      <div class="timeline">
        <div v-for="year in uniqueYears" :key="year" class="timeline-year reveal">
          <div class="year-marker">
            <span class="year-label">{{ year }}</span>
            <div class="year-line"></div>
          </div>

          <div class="year-operations stagger-children">
            <div
              v-for="op in operations.filter(o => o.year === year)"
              :key="op.key"
              :class="['operation-card', { expanded: expandedOp === op.key }]"
              :style="{ '--status-color': statusColors[op.status] }"
              @click="toggleOp(op.key)"
            >
              <!-- Status indicator -->
              <div class="op-status-indicator">
                <div class="status-dot"></div>
                <div class="status-pulse"></div>
              </div>

              <!-- Card content -->
              <div class="op-content">
                <div class="op-header">
                  <h3 class="op-name">{{ t(`operations.list.${op.key}.name`) }}</h3>
                  <div class="op-status-badge">
                    <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
                      <path :d="statusIcons[op.status]" />
                    </svg>
                    <span>{{ t(`operations.status.${op.status}`) }}</span>
                  </div>
                </div>

                <p class="op-desc">{{ t(`operations.list.${op.key}.description`) }}</p>

                <!-- Expanded indicator -->
                <div class="op-expand-hint">
                  <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M19 9l-7 7-7-7" />
                  </svg>
                </div>
              </div>

              <!-- Accent line -->
              <div class="op-accent"></div>
            </div>
          </div>
        </div>
      </div>

      <!-- Summary stats -->
      <div class="ops-summary reveal">
        <div class="summary-item">
          <span class="summary-count">{{ operations.length }}</span>
          <span class="summary-label">Total Ops</span>
        </div>
        <div class="summary-divider"></div>
        <div class="summary-item">
          <span class="summary-count" style="color: #4ae87a">{{ operations.filter(o => o.status === 'success').length }}</span>
          <span class="summary-label">Success</span>
        </div>
        <div class="summary-divider"></div>
        <div class="summary-item">
          <span class="summary-count" style="color: #e8a84a">{{ operations.filter(o => o.status === 'partial').length }}</span>
          <span class="summary-label">Partial</span>
        </div>
        <div class="summary-divider"></div>
        <div class="summary-item">
          <span class="summary-count" style="color: #e8534a">{{ operations.filter(o => o.status === 'failure').length }}</span>
          <span class="summary-label">Failed</span>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.operations {
  background: var(--color-bg-secondary);
  overflow: hidden;
}

.timeline {
  max-width: 800px;
  margin: 0 auto;
}

.timeline-year {
  margin-bottom: var(--space-3xl);
}

.timeline-year:last-child {
  margin-bottom: 0;
}

.year-marker {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  margin-bottom: var(--space-xl);
}

.year-label {
  font-family: var(--font-mono);
  font-size: 1.4rem;
  font-weight: 700;
  color: var(--color-accent);
  letter-spacing: 0.1em;
  flex-shrink: 0;
}

.year-line {
  flex: 1;
  height: 1px;
  background: linear-gradient(90deg, var(--color-accent), transparent);
  opacity: 0.3;
}

.year-operations {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
  padding-left: var(--space-xl);
  border-left: 1px solid var(--color-border);
  margin-left: 40px;
}

.operation-card {
  display: flex;
  gap: var(--space-lg);
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-xl);
  position: relative;
  cursor: pointer;
  transition: all var(--duration-normal) var(--ease-out-expo);
}

.operation-card:hover {
  border-color: var(--status-color);
  transform: translateX(4px);
  box-shadow: -4px 0 20px rgba(0, 0, 0, 0.2);
}

.operation-card.expanded {
  border-color: var(--status-color);
  background: var(--color-bg-card-hover);
}

.op-status-indicator {
  position: relative;
  flex-shrink: 0;
  width: 16px;
  display: flex;
  align-items: flex-start;
  padding-top: 6px;
}

.status-dot {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--status-color);
  position: relative;
  z-index: 1;
}

.status-pulse {
  position: absolute;
  top: 3px;
  left: 3px;
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: var(--status-color);
  opacity: 0;
  animation: statusPulse 2s ease-in-out infinite;
}

@keyframes statusPulse {
  0%, 100% { opacity: 0; transform: scale(1); }
  50% { opacity: 0.3; transform: scale(2); }
}

.op-content {
  flex: 1;
  min-width: 0;
}

.op-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-md);
  margin-bottom: var(--space-sm);
}

.op-name {
  font-size: 1.1rem;
  font-weight: 600;
  color: var(--color-text-primary);
}

.op-status-badge {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid var(--status-color);
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  font-size: 0.65rem;
  color: var(--status-color);
  letter-spacing: 0.05em;
  flex-shrink: 0;
}

.op-status-badge svg {
  width: 12px;
  height: 12px;
}

.op-desc {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  line-height: 1.7;
}

.op-expand-hint {
  display: flex;
  justify-content: center;
  margin-top: var(--space-sm);
  color: var(--color-text-muted);
  opacity: 0;
  transition: opacity var(--duration-fast) ease;
}

.operation-card:hover .op-expand-hint {
  opacity: 0.5;
}

.op-expand-hint svg {
  width: 16px;
  height: 16px;
  animation: bounceDown 1.5s ease-in-out infinite;
}

@keyframes bounceDown {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(3px); }
}

.op-accent {
  position: absolute;
  top: 0;
  left: 0;
  bottom: 0;
  width: 3px;
  background: var(--status-color);
  opacity: 0;
  transition: opacity var(--duration-normal) ease;
  border-radius: var(--radius-lg) 0 0 var(--radius-lg);
}

.operation-card:hover .op-accent,
.operation-card.expanded .op-accent {
  opacity: 1;
}

/* Summary */
.ops-summary {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: var(--space-xl);
  margin-top: var(--space-4xl);
  padding: var(--space-xl);
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  max-width: 500px;
  margin-left: auto;
  margin-right: auto;
}

.summary-item {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
}

.summary-count {
  font-family: var(--font-mono);
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-text-primary);
}

.summary-label {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  color: var(--color-text-muted);
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

.summary-divider {
  width: 1px;
  height: 32px;
  background: var(--color-border);
}

@media (max-width: 768px) {
  .year-operations {
    margin-left: 20px;
    padding-left: var(--space-md);
  }

  .operation-card {
    flex-direction: column;
    gap: var(--space-md);
    padding: var(--space-lg);
  }

  .op-status-indicator {
    width: auto;
    padding-top: 0;
  }

  .op-header {
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .ops-summary {
    gap: var(--space-md);
    padding: var(--space-md);
  }

  .summary-count {
    font-size: 1.2rem;
  }
}

@media (max-width: 480px) {
  .year-label {
    font-size: 1.1rem;
  }

  .year-operations {
    margin-left: 10px;
  }
}
</style>
