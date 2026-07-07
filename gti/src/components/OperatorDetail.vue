<script setup lang="ts">
import { useI18n } from 'vue-i18n'

interface OperatorData {
  key: string
  class: string
  color: string
}

const props = defineProps<{
  operator: OperatorData
}>()

const emit = defineEmits<{
  close: []
}>()

const { t } = useI18n()

const hasProfile = () => {
  try {
    return !!t(`operators.list.${props.operator.key}.profile.background`)
  } catch {
    return false
  }
}
</script>

<template>
  <Teleport to="body">
    <div class="detail-overlay" @click.self="emit('close')">
      <div class="detail-panel">
        <button class="detail-close" @click="emit('close')" :aria-label="t('operators.detail.close')">
          <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
            <path d="M18 6L6 18M6 6l12 12"/>
          </svg>
        </button>

        <div class="detail-header">
          <div class="detail-avatar" :style="{ '--avatar-color': operator.color }">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2"/>
              <circle cx="12" cy="7" r="4"/>
            </svg>
          </div>
          <div class="detail-identity">
            <span class="detail-codename">{{ t(`operators.list.${operator.key}.codename`) }}</span>
            <span class="detail-name">{{ t(`operators.list.${operator.key}.name`) }}</span>
            <span class="detail-role" :style="{ color: operator.color }">{{ t(`operators.list.${operator.key}.role`) }}</span>
          </div>
          <div class="detail-class-badge" :style="{ background: operator.color }">
            {{ t(`operators.classes.${operator.class}.name`) }}
          </div>
        </div>

        <div v-if="hasProfile()" class="detail-body">
          <div class="detail-meta-grid">
            <div class="meta-item">
              <span class="meta-label">{{ t('operators.detail.rank') }}</span>
              <span class="meta-value">{{ t(`operators.list.${operator.key}.profile.rank`) }}</span>
            </div>
            <div class="meta-item">
              <span class="meta-label">{{ t('operators.detail.age') }}</span>
              <span class="meta-value">{{ t(`operators.list.${operator.key}.profile.age`) }}</span>
            </div>
            <div class="meta-item">
              <span class="meta-label">{{ t('operators.detail.dob') }}</span>
              <span class="meta-value">{{ t(`operators.list.${operator.key}.profile.dob`) }}</span>
            </div>
            <div class="meta-item">
              <span class="meta-label">{{ t('operators.detail.height') }}</span>
              <span class="meta-value">{{ t(`operators.list.${operator.key}.profile.height`) }}</span>
            </div>
            <div class="meta-item">
              <span class="meta-label">{{ t('operators.detail.weight') }}</span>
              <span class="meta-value">{{ t(`operators.list.${operator.key}.profile.weight`) }}</span>
            </div>
            <div class="meta-item">
              <span class="meta-label">{{ t('operators.detail.affiliation') }}</span>
              <span class="meta-value">{{ t(`operators.list.${operator.key}.profile.affiliation`) }}</span>
            </div>
          </div>

          <div class="detail-section">
            <h4 class="detail-section-title">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253"/></svg>
              {{ t('operators.detail.background') }}
            </h4>
            <p class="detail-text">{{ t(`operators.list.${operator.key}.profile.background`) }}</p>
            <p class="detail-text">{{ t(`operators.list.${operator.key}.profile.desertSpring`) }}</p>
            <p class="detail-text">{{ t(`operators.list.${operator.key}.profile.joingti`) }}</p>
          </div>

          <div class="detail-section">
            <h4 class="detail-section-title">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M13 2L3 14h9l-1 8 10-12h-9l1-8z"/></svg>
              {{ t(`operators.list.${operator.key}.profile.skillsTitle`) }}
            </h4>
            <p class="detail-text">{{ t(`operators.list.${operator.key}.profile.skillsIntro`) }}</p>
            <div class="skill-list">
              <div class="skill-item">
                <span class="skill-name">{{ t(`operators.list.${operator.key}.profile.equipment.name`) }}</span>
                <p class="skill-desc">{{ t(`operators.list.${operator.key}.profile.equipment.desc`) }}</p>
              </div>
              <div class="skill-item">
                <span class="skill-name">{{ t(`operators.list.${operator.key}.profile.active.name`) }}</span>
                <p class="skill-desc">{{ t(`operators.list.${operator.key}.profile.active.desc`) }}</p>
              </div>
              <div class="skill-item">
                <span class="skill-name">{{ t(`operators.list.${operator.key}.profile.throwable.name`) }}</span>
                <p class="skill-desc">{{ t(`operators.list.${operator.key}.profile.throwable.desc`) }}</p>
              </div>
              <div class="skill-item">
                <span class="skill-name">{{ t(`operators.list.${operator.key}.profile.passive.name`) }}</span>
                <p class="skill-desc">{{ t(`operators.list.${operator.key}.profile.passive.desc`) }}</p>
              </div>
            </div>
          </div>

          <div class="detail-section">
            <h4 class="detail-section-title">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5"><path d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"/><path d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z"/></svg>
              {{ t(`operators.list.${operator.key}.profile.playstyleTitle`) }}
            </h4>
            <p class="detail-text">{{ t(`operators.list.${operator.key}.profile.playstyle`) }}</p>
            <p class="detail-text">{{ t(`operators.list.${operator.key}.profile.engagement`) }}</p>
            <p class="detail-text">{{ t(`operators.list.${operator.key}.profile.weapons`) }}</p>
          </div>
        </div>

        <div v-else class="detail-body detail-placeholder">
          <div class="placeholder-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1">
              <path d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"/>
            </svg>
          </div>
          <p class="placeholder-text">{{ t('operators.detailclassified') }}</p>
          <p class="placeholder-sub">{{ t('operators.detail.classifiedSub') }}</p>
        </div>
      </div>
    </div>
  </Teleport>
</template>

<style scoped>
.detail-overlay {
  position: fixed;
  inset: 0;
  z-index: 1000;
  background: rgba(0, 0, 0, 0.85);
  backdrop-filter: blur(8px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: var(--space-xl);
  animation: fadeIn 200ms ease;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from { opacity: 0; transform: translateY(24px); }
  to { opacity: 1; transform: translateY(0); }
}

.detail-panel {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-xl);
  max-width: 720px;
  width: 100%;
  max-height: 85vh;
  overflow-y: auto;
  position: relative;
  animation: slideUp 300ms ease;
}

.detail-close {
  position: absolute;
  top: var(--space-lg);
  right: var(--space-lg);
  width: 36px;
  height: 36px;
  border-radius: 50%;
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  color: var(--color-text-muted);
  cursor: pointer;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all var(--transition-fast);
  z-index: 1;
}

.detail-close:hover {
  color: var(--color-text-primary);
  border-color: var(--color-border-hover);
}

.detail-close svg {
  width: 18px;
  height: 18px;
}

.detail-header {
  padding: var(--space-2xl) var(--space-2xl) var(--space-lg);
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  border-bottom: 1px solid var(--color-border);
  position: relative;
}

.detail-avatar {
  width: 72px;
  height: 72px;
  border-radius: 50%;
  background: var(--color-accent-glow);
  border: 2px solid var(--avatar-color, var(--color-accent));
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--avatar-color, var(--color-accent));
  flex-shrink: 0;
}

.detail-avatar svg {
  width: 36px;
  height: 36px;
}

.detail-identity {
  display: flex;
  flex-direction: column;
  gap: 2px;
  flex: 1;
}

.detail-codename {
  font-size: 1.6rem;
  font-weight: 800;
  color: var(--color-text-primary);
  letter-spacing: 0.02em;
}

.detail-name {
  font-size: 0.9rem;
  color: var(--color-text-muted);
}

.detail-role {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  letter-spacing: 0.05em;
}

.detail-class-badge {
  position: absolute;
  top: var(--space-lg);
  right: var(--space-2xl);
  margin-right: 52px;
  font-family: var(--font-mono);
  font-size: 0.65rem;
  letter-spacing: 0.15em;
  text-transform: uppercase;
  color: var(--color-bg-primary);
  padding: var(--space-xs) var(--space-md);
  border-radius: var(--radius-sm);
  font-weight: 700;
}

.detail-body {
  padding: var(--space-2xl);
  display: flex;
  flex-direction: column;
  gap: var(--space-2xl);
}

.detail-meta-grid {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-md);
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-lg);
}

.meta-item {
  display: flex;
  flex-direction: column;
  gap: 2px;
}

.meta-label {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  letter-spacing: 0.1em;
  text-transform: uppercase;
  color: var(--color-text-muted);
}

.meta-value {
  font-size: 0.9rem;
  color: var(--color-text-primary);
  font-weight: 500;
}

.detail-section {
  display: flex;
  flex-direction: column;
  gap: var(--space-md);
}

.detail-section-title {
  font-size: 0.85rem;
  font-weight: 700;
  color: var(--color-accent);
  letter-spacing: 0.1em;
  text-transform: uppercase;
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding-bottom: var(--space-sm);
  border-bottom: 1px solid var(--color-border);
}

.detail-section-title svg {
  width: 18px;
  height: 18px;
  flex-shrink: 0;
}

.detail-text {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  line-height: 1.8;
}

.skill-list {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
  margin-top: var(--space-sm);
}

.skill-item {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  padding: var(--space-lg);
}

.skill-name {
  display: block;
  font-size: 0.85rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-xs);
}

.skill-desc {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  line-height: 1.6;
}

.detail-placeholder {
  align-items: center;
  text-align: center;
  padding: var(--space-4xl) var(--space-2xl);
}

.placeholder-icon {
  width: 64px;
  height: 64px;
  color: var(--color-text-muted);
  opacity: 0.4;
  margin-bottom: var(--space-lg);
}

.placeholder-icon svg {
  width: 100%;
  height: 100%;
}

.placeholder-text {
  font-size: 1rem;
  color: var(--color-text-muted);
  font-weight: 600;
}

.placeholder-sub {
  font-size: 0.85rem;
  color: var(--color-text-muted);
  opacity: 0.6;
  margin-top: var(--space-xs);
}

@media (max-width: 640px) {
  .detail-overlay {
    padding: var(--space-md);
    align-items: flex-end;
  }

  .detail-panel {
    max-height: 90vh;
    border-radius: var(--radius-xl) var(--radius-xl) 0 0;
  }

  .detail-header {
    padding: var(--space-xl) var(--space-lg) var(--space-md);
    flex-wrap: wrap;
  }

  .detail-class-badge {
    position: static;
    margin-right: 0;
  }

  .detail-body {
    padding: var(--space-lg);
  }

  .detail-meta-grid {
    grid-template-columns: repeat(2, 1fr);
  }

  .detail-codename {
    font-size: 1.3rem;
  }
}
</style>
