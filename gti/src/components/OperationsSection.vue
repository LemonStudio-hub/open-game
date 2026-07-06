<script setup lang="ts">
import { ref } from 'vue'
import { useScrollReveal } from '../composables/useScrollReveal'
import { operations } from '../data/operations'

const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const getStatusLabel = (status: string) => {
  const map: Record<string, string> = {
    success: '成功',
    failure: '失利',
    ongoing: '进行中',
  }
  return map[status] || status
}
</script>

<template>
  <section id="operations" ref="sectionRef" class="section operations">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">// OPERATIONS</span>
        <h2 class="section-title">主要行动</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          2035年阿萨拉地区关键军事行动记录
        </p>
      </div>

      <div class="operations-list">
        <div
          v-for="(op, index) in operations"
          :key="op.name"
          class="operation-card reveal"
        >
          <div class="op-index">
            <span class="op-number">{{ String(index + 1).padStart(2, '0') }}</span>
          </div>
          <div class="op-content">
            <div class="op-header">
              <div>
                <span class="op-year">{{ op.year }}</span>
                <h3 class="op-name">{{ op.name }}</h3>
              </div>
              <span
                class="op-status"
                :class="op.status"
              >
                {{ getStatusLabel(op.status) }}
              </span>
            </div>
            <p class="op-desc">{{ op.description }}</p>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.operations {
  background: var(--color-bg-secondary);
}

.operations-list {
  max-width: 800px;
  margin: 0 auto;
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
}

.operation-card {
  display: flex;
  gap: var(--space-xl);
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-xl);
  transition: all var(--transition-base);
}

.operation-card:hover {
  border-color: var(--color-border-hover);
  box-shadow: var(--shadow-accent);
}

.op-index {
  flex-shrink: 0;
  width: 48px;
  display: flex;
  align-items: flex-start;
  justify-content: center;
}

.op-number {
  font-family: var(--font-mono);
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-accent-dark);
  opacity: 0.5;
}

.op-content {
  flex: 1;
}

.op-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: var(--space-md);
  margin-bottom: var(--space-sm);
}

.op-year {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  color: var(--color-text-muted);
  letter-spacing: 0.1em;
}

.op-name {
  font-size: 1.15rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-top: var(--space-xs);
}

.op-status {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  letter-spacing: 0.05em;
  padding: var(--space-xs) var(--space-sm);
  border-radius: var(--radius-sm);
  flex-shrink: 0;
}

.op-status.success {
  color: #4ae87a;
  background: rgba(74, 232, 122, 0.1);
  border: 1px solid rgba(74, 232, 122, 0.2);
}

.op-status.failure {
  color: #e8534a;
  background: rgba(232, 83, 74, 0.1);
  border: 1px solid rgba(232, 83, 74, 0.2);
}

.op-status.ongoing {
  color: #e8a84a;
  background: rgba(232, 168, 74, 0.1);
  border: 1px solid rgba(232, 168, 74, 0.2);
}

.op-desc {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  line-height: 1.7;
}

@media (max-width: 768px) {
  .operation-card {
    flex-direction: column;
    gap: var(--space-md);
  }

  .op-index {
    width: auto;
    justify-content: flex-start;
  }
}
</style>
