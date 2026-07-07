<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t, tm } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)
</script>

<template>
  <section id="structure" ref="sectionRef" class="section structure">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('structure.label') }}</span>
        <h2 class="section-title">{{ t('structure.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          {{ t('structure.subtitle') }}
        </p>
      </div>

      <div class="structure-grid">
        <div class="structure-main reveal">
          <div class="command-card">
            <div class="command-header">
              <div class="command-icon">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path d="M12 2L2 7l10 5 10-5-10-5z"/>
                  <path d="M2 17l10 5 10-5M2 12l10 5 10-5"/>
                </svg>
              </div>
              <div>
                <h3 class="command-title">{{ t('structure.commandTitle') }}</h3>
                <p class="command-subtitle">{{ t('structure.commandSubtitle') }}</p>
              </div>
            </div>
            <div class="command-detail">
              <p>{{ t('structure.commandDetail') }}</p>
            </div>
          </div>

          <div class="commander-info reveal">
            <div class="commander-avatar">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"/>
                <circle cx="12" cy="7" r="4"/>
              </svg>
            </div>
            <div>
              <span class="commander-label">{{ t('structure.commanderLabel') }}</span>
              <h4 class="commander-name">{{ t('structure.commanderName') }}</h4>
            </div>
          </div>
        </div>

        <div class="structure-side">
          <div class="departments reveal">
            <h4 class="side-title">{{ t('structure.departmentsTitle') }}</h4>
            <div class="dept-list">
              <div v-for="(dept, idx) in (tm('structure.departments') as any[])" :key="idx" class="dept-item">
                <span class="dept-icon">
                  <svg v-if="idx === 0" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z"/>
                  </svg>
                  <svg v-else-if="idx === 1" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M14.5 17.5L3 6V3h3l11.5 11.5M13 7l4-4 4 4-4 4M3 17l4 4 4-4"/>
                  </svg>
                  <svg v-else-if="idx === 2" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <circle cx="12" cy="12" r="3"/>
                    <path d="M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4M4.93 19.07l2.83-2.83M16.24 7.76l2.83-2.83"/>
                  </svg>
                  <svg v-else viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                    <path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/>
                  </svg>
                </span>
                <div>
                  <h5 class="dept-name">{{ dept.name }}</h5>
                  <p class="dept-desc">{{ dept.desc }}</p>
                </div>
              </div>
            </div>
          </div>

          <div class="branches reveal">
            <h4 class="side-title">{{ t('structure.branchesTitle') }}</h4>
            <div class="branch-grid">
              <div v-for="(branch, idx) in (tm('structure.branches') as string[])" :key="idx" class="branch-tag">
                {{ branch }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.structure {
  background: var(--color-bg-secondary);
}

.structure-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-2xl);
  align-items: start;
}

.command-card {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-2xl);
  margin-bottom: var(--space-xl);
}

.command-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  margin-bottom: var(--space-lg);
}

.command-icon {
  width: 48px;
  height: 48px;
  color: var(--color-accent);
}

.command-icon svg {
  width: 100%;
  height: 100%;
}

.command-title {
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--color-text-primary);
}

.command-subtitle {
  font-size: 0.85rem;
  color: var(--color-text-muted);
  font-family: var(--font-mono);
}

.command-detail p {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  line-height: 1.7;
  margin-bottom: var(--space-md);
}

.commander-info {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-lg);
}

.commander-avatar {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: var(--color-accent-glow);
  border: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-accent);
}

.commander-avatar svg {
  width: 28px;
  height: 28px;
}

.commander-label {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  color: var(--color-text-muted);
  letter-spacing: 0.1em;
}

.commander-name {
  font-size: 1rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-top: var(--space-xs);
}

.side-title {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  letter-spacing: 0.15em;
  color: var(--color-accent);
  margin-bottom: var(--space-lg);
}

.departments {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-2xl);
  margin-bottom: var(--space-xl);
}

.dept-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
}

.dept-item {
  display: flex;
  gap: var(--space-md);
  align-items: flex-start;
}

.dept-icon {
  color: var(--color-accent);
  width: 24px;
  height: 24px;
  flex-shrink: 0;
  margin-top: 2px;
}

.dept-icon svg {
  width: 100%;
  height: 100%;
}

.dept-name {
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-xs);
}

.dept-desc {
  font-size: 0.8rem;
  color: var(--color-text-secondary);
  line-height: 1.5;
}

.branches {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-2xl);
}

.branch-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: var(--space-sm);
}

.branch-tag {
  font-family: var(--font-mono);
  font-size: 0.8rem;
  color: var(--color-text-secondary);
  padding: var(--space-sm) var(--space-md);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  text-align: center;
  transition: all var(--transition-base);
}

.branch-tag:hover {
  border-color: var(--color-border-hover);
  color: var(--color-accent);
  background: var(--color-accent-glow);
}

@media (max-width: 768px) {
  .structure-grid {
    grid-template-columns: 1fr;
  }

  .command-card,
  .departments,
  .branches {
    padding: var(--space-lg);
  }

  .commander-info {
    padding: var(--space-md);
  }
}

@media (max-width: 480px) {
  .branch-grid {
    grid-template-columns: 1fr;
  }
}
</style>
