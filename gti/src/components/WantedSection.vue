<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t, tm } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const expandedCard = ref<string | null>(null)

const targetKeys = ['sayid', 'reiss', 'anais', 'hudson', 'ghroth', 'raven', 'jacob', 'youssef', 'hamka']

const threatColors: Record<string, string> = {
  critical: '#e84a4a',
  high: '#e8a84a',
  moderate: '#4a9ee8',
}

const threatIcons: Record<string, string> = {
  critical: 'M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z',
  high: 'M13 10V3L4 14h7v7l9-11h-7z',
  moderate: 'M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z',
}

const toggleCard = (key: string) => {
  expandedCard.value = expandedCard.value === key ? null : key
}
</script>

<template>
  <section id="wanted" ref="sectionRef" class="section wanted">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('wanted.label') }}</span>
        <h2 class="section-title">{{ t('wanted.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">{{ t('wanted.subtitle') }}</p>
      </div>

      <!-- Threat level legend -->
      <div class="threat-legend reveal">
        <div v-for="(color, level) in threatColors" :key="level" class="legend-item">
          <span class="legend-dot" :style="{ background: color }"></span>
          <span class="legend-label">{{ t(`wanted.threatLevels.${level}`) }}</span>
        </div>
      </div>

      <!-- Wanted cards grid -->
      <div class="wanted-grid stagger-children">
        <div
          v-for="key in targetKeys"
          :key="key"
          :class="['wanted-card', { expanded: expandedCard === key }]"
          :style="{ '--threat-color': threatColors[t(`wanted.list.${key}.threat`)] }"
          @click="toggleCard(key)"
        >
          <!-- Card top accent -->
          <div class="card-accent"></div>

          <!-- Threat level badge -->
          <div class="threat-badge">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <path :d="threatIcons[t(`wanted.list.${key}.threat`)]" />
            </svg>
            <span>{{ t(`wanted.threatLevels.${t(`wanted.list.${key}.threat`)}`) }}</span>
          </div>

          <!-- Wanted header -->
          <div class="wanted-header">
            <div class="wanted-avatar">
              <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2" />
                <circle cx="12" cy="7" r="4" />
              </svg>
              <div class="avatar-crosshair"></div>
            </div>
            <div class="wanted-identity">
              <h3 class="wanted-name">{{ t(`wanted.list.${key}.name`) }}</h3>
              <span v-if="t(`wanted.list.${key}.codename`) !== '—'" class="wanted-codename">
                "{{ t(`wanted.list.${key}.codename`) }}"
              </span>
            </div>
          </div>

          <!-- Wanted info -->
          <div class="wanted-info">
            <div class="info-row">
              <span class="info-label">{{ t('wanted.affiliation') }}</span>
              <span class="info-value">{{ t(`wanted.list.${key}.affiliation`) }}</span>
            </div>
            <div class="info-row">
              <span class="info-label">{{ t('wanted.lastSeen') }}</span>
              <div class="location-tags">
                <span
                  v-for="(loc, idx) in (tm(`wanted.list.${key}.locations`) as string[])"
                  :key="idx"
                  class="location-tag"
                >
                  {{ loc }}
                </span>
              </div>
            </div>
          </div>

          <!-- Description (expandable) -->
          <div class="wanted-desc">
            <p>{{ t(`wanted.list.${key}.description`) }}</p>
          </div>

          <!-- Scan line effect -->
          <div class="scan-effect"></div>

          <!-- Corner decorations -->
          <div class="corner corner-tl"></div>
          <div class="corner corner-tr"></div>
          <div class="corner corner-bl"></div>
          <div class="corner corner-br"></div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.wanted {
  background: var(--color-bg-primary);
  overflow: hidden;
}

.threat-legend {
  display: flex;
  justify-content: center;
  gap: var(--space-xl);
  margin-bottom: var(--space-3xl);
}

.legend-item {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.legend-dot {
  width: 8px;
  height: 8px;
  border-radius: 50%;
  animation: dotPulse 2s ease-in-out infinite;
}

@keyframes dotPulse {
  0%, 100% { opacity: 0.6; }
  50% { opacity: 1; }
}

.legend-label {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  color: var(--color-text-muted);
  letter-spacing: 0.08em;
}

.wanted-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(min(340px, 100%), 1fr));
  gap: var(--space-lg);
}

.wanted-card {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-xl);
  position: relative;
  overflow: hidden;
  cursor: pointer;
  transition: all var(--duration-normal) var(--ease-out-expo);
}

.wanted-card::before {
  content: '';
  position: absolute;
  inset: 0;
  background: radial-gradient(circle at 50% 0%, var(--threat-color), transparent 70%);
  opacity: 0;
  transition: opacity var(--duration-normal) ease;
  pointer-events: none;
}

.wanted-card:hover {
  border-color: var(--threat-color);
  transform: translateY(-4px);
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
}

.wanted-card:hover::before {
  opacity: 0.06;
}

.card-accent {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 3px;
  background: var(--threat-color);
  opacity: 0.6;
  transition: opacity var(--duration-normal) ease;
}

.wanted-card:hover .card-accent {
  opacity: 1;
}

.threat-badge {
  position: absolute;
  top: var(--space-md);
  right: var(--space-md);
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 2px 8px;
  background: rgba(0, 0, 0, 0.5);
  border: 1px solid var(--threat-color);
  border-radius: var(--radius-sm);
  font-family: var(--font-mono);
  font-size: 0.65rem;
  color: var(--threat-color);
  letter-spacing: 0.08em;
  text-transform: uppercase;
}

.threat-badge svg {
  width: 12px;
  height: 12px;
}

.wanted-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  margin-bottom: var(--space-lg);
}

.wanted-avatar {
  position: relative;
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: var(--color-bg-tertiary);
  border: 1.5px solid var(--threat-color);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-text-muted);
  flex-shrink: 0;
}

.wanted-avatar svg {
  width: 28px;
  height: 28px;
}

.avatar-crosshair {
  position: absolute;
  inset: -4px;
  border: 1px solid var(--threat-color);
  border-radius: 50%;
  opacity: 0.3;
  animation: crosshairPulse 3s ease-in-out infinite;
}

@keyframes crosshairPulse {
  0%, 100% { transform: scale(1); opacity: 0.3; }
  50% { transform: scale(1.1); opacity: 0.5; }
}

.wanted-identity {
  flex: 1;
}

.wanted-name {
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--color-text-primary);
  margin-bottom: 2px;
}

.wanted-codename {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  color: var(--threat-color);
  font-style: italic;
}

.wanted-info {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
  margin-bottom: var(--space-md);
  padding: var(--space-md);
  background: rgba(0, 0, 0, 0.2);
  border-radius: var(--radius-md);
}

.info-row {
  display: flex;
  align-items: flex-start;
  gap: var(--space-md);
}

.info-label {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  color: var(--color-text-muted);
  letter-spacing: 0.1em;
  text-transform: uppercase;
  min-width: 80px;
  flex-shrink: 0;
  padding-top: 2px;
}

.info-value {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
}

.location-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.location-tag {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  color: var(--color-text-secondary);
  padding: 2px 8px;
  background: var(--color-bg-tertiary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
}

.wanted-desc p {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  line-height: 1.7;
}

/* Scan line effect */
.scan-effect {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 1px;
  background: linear-gradient(90deg, transparent, var(--threat-color), transparent);
  opacity: 0;
  animation: scanDown 4s linear infinite;
  pointer-events: none;
}

.wanted-card:hover .scan-effect {
  opacity: 0.4;
}

@keyframes scanDown {
  0% { top: 0; }
  100% { top: 100%; }
}

/* Corner decorations */
.corner {
  position: absolute;
  width: 12px;
  height: 12px;
  border-color: var(--threat-color);
  opacity: 0.3;
  transition: opacity var(--duration-normal) ease;
}

.wanted-card:hover .corner {
  opacity: 0.7;
}

.corner-tl {
  top: 8px;
  left: 8px;
  border-top: 1px solid;
  border-left: 1px solid;
}

.corner-tr {
  top: 8px;
  right: 8px;
  border-top: 1px solid;
  border-right: 1px solid;
}

.corner-bl {
  bottom: 8px;
  left: 8px;
  border-bottom: 1px solid;
  border-left: 1px solid;
}

.corner-br {
  bottom: 8px;
  right: 8px;
  border-bottom: 1px solid;
  border-right: 1px solid;
}

@media (max-width: 768px) {
  .threat-legend {
    gap: var(--space-md);
    flex-wrap: wrap;
  }

  .wanted-grid {
    grid-template-columns: 1fr;
  }

  .wanted-card {
    padding: var(--space-lg);
  }
}

@media (max-width: 480px) {
  .info-row {
    flex-direction: column;
    gap: 2px;
  }

  .info-label {
    min-width: auto;
  }
}
</style>
