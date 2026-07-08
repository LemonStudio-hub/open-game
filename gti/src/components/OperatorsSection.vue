<script setup lang="ts">
import { ref, computed } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'
import { operators, operatorClasses } from '../data/operators'
import OperatorDetail from './OperatorDetail.vue'

const { t } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const activeClass = ref<string>('assault')
const selectedOperator = ref<string | null>(null)

const filteredOperators = computed(() =>
  operators.filter((op) => op.class === activeClass.value)
)

const currentClassInfo = computed(() =>
  operatorClasses.find((c) => c.id === activeClass.value)
)

const selectedOperatorData = computed(() => {
  if (!selectedOperator.value) return null
  const op = operators.find((o) => o.key === selectedOperator.value)
  if (!op) return null
  const cls = operatorClasses.find((c) => c.id === op.class)
  return { key: op.key, class: op.class, color: cls?.color ?? 'var(--color-accent)' }
})

const setClass = (classId: string) => {
  activeClass.value = classId
}

const openDetail = (opKey: string) => {
  selectedOperator.value = opKey
}

const closeDetail = () => {
  selectedOperator.value = null
}
</script>

<template>
  <section id="operators" ref="sectionRef" class="section operators">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('operators.label') }}</span>
        <h2 class="section-title">{{ t('operators.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          {{ t('operators.subtitle') }}
        </p>
      </div>

      <div class="class-tabs reveal">
        <button
          v-for="cls in operatorClasses"
          :key="cls.id"
          class="class-tab"
          :class="{ active: activeClass === cls.id }"
          :style="activeClass === cls.id ? { '--tab-color': cls.color } : {}"
          @click="setClass(cls.id)"
        >
          <span class="tab-name">{{ t(`operators.classes.${cls.id}.name`) }}</span>
          <span class="tab-name-en">{{ t(`operators.classes.${cls.id}.nameEn`) }}</span>
        </button>
      </div>

      <div class="class-description reveal">
        <p :style="{ color: currentClassInfo?.color }">{{ t(`operators.classes.${currentClassInfo?.id}.description`) }}</p>
      </div>

      <div class="operators-grid stagger-children">
        <div
          v-for="op in filteredOperators"
          :key="op.key"
          class="operator-card"
          :style="{ '--card-color': currentClassInfo?.color }"
          @click="openDetail(op.key)"
        >
          <div class="operator-avatar">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"/>
              <circle cx="12" cy="7" r="4"/>
            </svg>
          </div>
          <div class="operator-info">
            <span class="operator-codename">{{ t(`operators.list.${op.key}.codename`) }}</span>
            <span class="operator-name">{{ t(`operators.list.${op.key}.name`) }}</span>
            <span class="operator-role">{{ t(`operators.list.${op.key}.role`) }}</span>
          </div>
          <div class="operator-bio">
            <p>{{ t(`operators.list.${op.key}.bio`) }}</p>
          </div>
          <div class="operator-class-indicator" :style="{ background: currentClassInfo?.color }"></div>
        </div>
      </div>
    </div>

    <OperatorDetail
      v-if="selectedOperatorData"
      :operator="selectedOperatorData"
      @close="closeDetail"
    />
  </section>
</template>

<style scoped>
.operators {
  background: var(--color-bg-primary);
}

.class-tabs {
  display: grid;
  grid-template-columns: repeat(4, 1fr);
  gap: var(--space-sm);
  margin-bottom: var(--space-xl);
  max-width: 600px;
  margin-left: auto;
  margin-right: auto;
}

.class-tab {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 2px;
  padding: var(--space-sm) var(--space-md);
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  cursor: pointer;
  transition: all var(--transition-base);
  color: var(--color-text-secondary);
}

.class-tab:hover {
  border-color: var(--color-border-hover);
  color: var(--color-text-primary);
}

.class-tab.active {
  border-color: var(--tab-color, var(--color-accent));
  background: rgba(201, 168, 76, 0.08);
  color: var(--tab-color, var(--color-accent));
  box-shadow: 0 0 20px rgba(201, 168, 76, 0.1);
}

.tab-name {
  font-size: 1rem;
  font-weight: 600;
}

.tab-name-en {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  opacity: 0.7;
}

.class-description {
  text-align: center;
  margin-bottom: var(--space-2xl);
  font-size: 0.9rem;
}

.operators-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(min(280px, 100%), 1fr));
  gap: var(--space-lg);
}

.operator-card {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-xl);
  transition: all var(--duration-normal) var(--ease-out-expo);
  position: relative;
  overflow: hidden;
  cursor: pointer;
}

.operator-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: radial-gradient(circle at var(--mouse-x, 50%) var(--mouse-y, 50%), 
    rgba(201, 168, 76, 0.08) 0%, 
    transparent 50%);
  opacity: 0;
  transition: opacity var(--duration-normal) ease;
  pointer-events: none;
  z-index: 0;
}

.operator-card:hover {
  border-color: var(--color-border-hover);
  transform: translateY(-6px);
  box-shadow: 0 12px 40px rgba(0, 0, 0, 0.3),
              0 0 30px rgba(201, 168, 76, 0.1);
}

.operator-card:hover::before {
  opacity: 1;
}

.operator-card > * {
  position: relative;
  z-index: 1;
}

.operator-card:hover {
  border-color: var(--color-border-hover);
  transform: translateY(-4px);
  box-shadow: var(--shadow-accent);
}

.operator-avatar {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: var(--color-accent-glow);
  border: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--card-color, var(--color-accent));
  margin-bottom: var(--space-md);
}

.operator-avatar svg {
  width: 28px;
  height: 28px;
}

.operator-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  margin-bottom: var(--space-sm);
}

.operator-codename {
  font-size: 1.2rem;
  font-weight: 700;
  color: var(--color-text-primary);
}

.operator-name {
  font-size: 0.85rem;
  color: var(--color-text-muted);
}

.operator-role {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  color: var(--card-color, var(--color-accent));
  letter-spacing: 0.05em;
}

.operator-bio p {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  line-height: 1.6;
}

.operator-class-indicator {
  position: absolute;
  bottom: 0;
  left: 0;
  right: 0;
  height: 2px;
  opacity: 0.5;
}

.operator-card:hover .operator-class-indicator {
  opacity: 1;
}

@media (max-width: 480px) {
  .class-tabs {
    grid-template-columns: repeat(2, 1fr);
  }

  .class-tab {
    padding: var(--space-sm);
  }

  .operator-card {
    padding: var(--space-lg);
  }
}
</style>
