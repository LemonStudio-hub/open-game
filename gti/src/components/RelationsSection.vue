<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const factionKeys = ['havoc', 'assara', 'un'] as const
const factionMeta: Record<string, { color: string; icon: string }> = {
  havoc: { color: 'var(--color-havoc)', icon: '⚔' },
  assara: { color: 'var(--color-assara)', icon: '⛊' },
  un: { color: 'var(--color-text-secondary)', icon: '◈' },
}
</script>

<template>
  <section id="relations" ref="sectionRef" class="section relations">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('relations.label') }}</span>
        <h2 class="section-title">{{ t('relations.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          {{ t('relations.subtitle') }}
        </p>
      </div>

      <div class="relations-grid stagger-children">
        <div
          v-for="key in factionKeys"
          :key="key"
          class="faction-card"
          :style="{ '--faction-color': factionMeta[key].color }"
        >
          <div class="faction-header">
            <div class="faction-icon">{{ factionMeta[key].icon }}</div>
            <div class="faction-identity">
              <h3 class="faction-name">{{ t(`relations.factions.${key}.name`) }}</h3>
              <span class="faction-name-en">{{ t(`relations.factions.${key}.nameEn`) }}</span>
            </div>
            <span class="faction-relation">{{ t(`relations.factions.${key}.relation`) }}</span>
          </div>
          <p class="faction-desc">{{ t(`relations.factions.${key}.description`) }}</p>
          <div class="faction-indicator"></div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.relations {
  background: var(--color-bg-primary);
}

.relations-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(min(320px, 100%), 1fr));
  gap: var(--space-xl);
}

.faction-card {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-2xl);
  transition: all var(--transition-base);
  position: relative;
  overflow: hidden;
}

.faction-card:hover {
  border-color: var(--color-border-hover);
  transform: translateY(-4px);
  box-shadow: var(--shadow-accent);
}

.faction-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  margin-bottom: var(--space-lg);
}

.faction-icon {
  font-size: 1.5rem;
  color: var(--faction-color);
  width: 40px;
  text-align: center;
}

.faction-identity {
  flex: 1;
}

.faction-name {
  font-size: 1.15rem;
  font-weight: 700;
  color: var(--color-text-primary);
}

.faction-name-en {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  color: var(--color-text-muted);
  letter-spacing: 0.1em;
}

.faction-relation {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  color: var(--faction-color);
  padding: var(--space-xs) var(--space-sm);
  border: 1px solid var(--faction-color);
  border-radius: var(--radius-sm);
  letter-spacing: 0.05em;
  opacity: 0.8;
}

.faction-desc {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  line-height: 1.7;
}

.faction-indicator {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--faction-color);
  opacity: 0.3;
}

.faction-card:hover .faction-indicator {
  opacity: 1;
}

@media (max-width: 768px) {
  .relations-grid {
    grid-template-columns: 1fr;
  }

  .faction-card {
    padding: var(--space-lg);
  }
}

@media (max-width: 480px) {
  .faction-header {
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .faction-relation {
    order: -1;
    width: 100%;
    text-align: center;
  }
}
</style>
