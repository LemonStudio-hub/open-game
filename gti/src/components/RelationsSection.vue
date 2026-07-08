<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const activeFaction = ref<string | null>(null)
const hoveredFaction = ref<string | null>(null)
const particles = ref<{ id: number; x: number; y: number; delay: number; duration: number }[]>([])

const factions = [
  { key: 'havoc', color: '#e84a4a', angle: -30 },
  { key: 'assara', color: '#4a9ee8', angle: 90 },
  { key: 'un', color: '#9a9690', angle: 210 },
]

const centerX = 300
const centerY = 250
const orbitRadius = 170

const getFactionPos = (angle: number) => {
  const rad = (angle * Math.PI) / 180
  return {
    x: centerX + orbitRadius * Math.cos(rad),
    y: centerY + orbitRadius * Math.sin(rad),
  }
}

const threatColors: Record<string, string> = {
  Critical: '#e84a4a',
  Moderate: '#e8a84a',
  'N/A': '#5a5650',
  '极高': '#e84a4a',
  '中等': '#e8a84a',
  '无': '#5a5650',
}

const threatLevels: Record<string, number> = {
  havoc: 95,
  assara: 50,
  un: 0,
  '极高': 95,
  '中等': 50,
  '无': 0,
}

onMounted(() => {
  const tempParticles = []
  for (let i = 0; i < 24; i++) {
    tempParticles.push({
      id: i,
      x: 0,
      y: 0,
      delay: Math.random() * 4,
      duration: 2 + Math.random() * 3,
    })
  }
  particles.value = tempParticles
})

const toggleFaction = (key: string) => {
  activeFaction.value = activeFaction.value === key ? null : key
}
</script>

<template>
  <section id="relations" ref="sectionRef" class="section relations">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('relations.label') }}</span>
        <h2 class="section-title">{{ t('relations.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">{{ t('relations.subtitle') }}</p>
      </div>

      <div class="relations-visualization reveal-scale">
        <div class="viz-container">
          <svg viewBox="0 0 600 500" class="relations-svg">
            <defs>
              <radialGradient id="centerGlow" cx="50%" cy="50%" r="50%">
                <stop offset="0%" stop-color="#c9a84c" stop-opacity="0.3" />
                <stop offset="100%" stop-color="#c9a84c" stop-opacity="0" />
              </radialGradient>

              <filter id="glow">
                <feGaussianBlur stdDeviation="3" result="blur" />
                <feMerge>
                  <feMergeNode in="blur" />
                  <feMergeNode in="SourceGraphic" />
                </feMerge>
              </filter>

              <linearGradient v-for="f in factions" :key="f.key" :id="`grad-${f.key}`" x1="0%" y1="0%" x2="100%" y2="100%">
                <stop offset="0%" :stop-color="f.color" stop-opacity="0.8" />
                <stop offset="100%" :stop-color="f.color" stop-opacity="0.3" />
              </linearGradient>

              <linearGradient v-for="f in factions" :key="`line-${f.key}`" :id="`line-${f.key}`">
                <stop offset="0%" stop-color="#c9a84c" stop-opacity="0.6" />
                <stop offset="50%" :stop-color="f.color" stop-opacity="0.8" />
                <stop offset="100%" stop-color="#c9a84c" stop-opacity="0.6" />
              </linearGradient>
            </defs>

            <!-- Orbit rings -->
            <circle :cx="centerX" :cy="centerY" :r="orbitRadius" class="orbit-ring" />
            <circle :cx="centerX" :cy="centerY" :r="orbitRadius - 30" class="orbit-ring-inner" />
            <circle :cx="centerX" :cy="centerY" :r="orbitRadius + 30" class="orbit-ring-outer" />

            <!-- Connection lines -->
            <g v-for="f in factions" :key="`line-${f.key}`">
              <line
                :x1="centerX"
                :y1="centerY"
                :x2="getFactionPos(f.angle).x"
                :y2="getFactionPos(f.angle).y"
                :class="['connection-line', { active: activeFaction === f.key || hoveredFaction === f.key }]"
                :stroke="`url(#line-${f.key})`"
                stroke-width="2"
              />
              <line
                :x1="centerX"
                :y1="centerY"
                :x2="getFactionPos(f.angle).x"
                :y2="getFactionPos(f.angle).y"
                class="connection-line-glow"
                :stroke="f.color"
                stroke-width="6"
                :opacity="activeFaction === f.key || hoveredFaction === f.key ? 0.15 : 0.05"
              />
            </g>

            <!-- Particles along connections -->
            <g v-for="f in factions" :key="`particles-${f.key}`">
              <circle
                v-for="p in particles.slice(0, 8)"
                :key="p.id"
                r="2"
                :fill="f.color"
                :opacity="activeFaction === f.key || hoveredFaction === f.key ? 0.8 : 0.3"
                class="particle"
              >
                <animateMotion
                  :dur="`${p.duration}s`"
                  :begin="`${p.delay}s`"
                  repeatCount="indefinite"
                  :path="`M${centerX},${centerY} L${getFactionPos(f.angle).x},${getFactionPos(f.angle).y}`"
                />
              </circle>
            </g>

            <!-- Center glow -->
            <circle :cx="centerX" :cy="centerY" r="80" fill="url(#centerGlow)" class="center-glow" />

            <!-- Center node -->
            <g class="center-node">
              <circle :cx="centerX" :cy="centerY" r="45" class="center-ring-outer" />
              <circle :cx="centerX" :cy="centerY" r="35" class="center-ring" />
              <circle :cx="centerX" :cy="centerY" r="28" class="center-bg" />
              <text :x="centerX" :y="centerY - 6" class="center-text-main">G.T.I.</text>
              <text :x="centerX" :y="centerY + 10" class="center-text-sub">{{ t('relations.centralRole') }}</text>
            </g>

            <!-- Faction nodes -->
            <g
              v-for="f in factions"
              :key="`node-${f.key}`"
              :class="['faction-node', { active: activeFaction === f.key, dimmed: activeFaction && activeFaction !== f.key }]"
              @click="toggleFaction(f.key)"
              @mouseenter="hoveredFaction = f.key"
              @mouseleave="hoveredFaction = null"
            >
              <circle
                :cx="getFactionPos(f.angle).x"
                :cy="getFactionPos(f.angle).y"
                r="38"
                :fill="f.color"
                fill-opacity="0.1"
                :stroke="f.color"
                stroke-width="1.5"
                stroke-opacity="0.3"
                class="faction-aura"
              />
              <circle
                :cx="getFactionPos(f.angle).x"
                :cy="getFactionPos(f.angle).y"
                r="28"
                class="faction-bg"
                :stroke="f.color"
                stroke-width="1"
                stroke-opacity="0.6"
              />
              <text
                :x="getFactionPos(f.angle).x"
                :y="getFactionPos(f.angle).y + 1"
                class="faction-label"
                :fill="f.color"
              >
                {{ t(`relations.factions.${f.key}.nameEn`) }}
              </text>
            </g>

            <!-- Relationship labels on connections -->
            <g v-for="f in factions" :key="`rel-label-${f.key}`">
              <rect
                :x="(centerX + getFactionPos(f.angle).x) / 2 - 40"
                :y="(centerY + getFactionPos(f.angle).y) / 2 - 10"
                width="80"
                height="20"
                rx="3"
                class="relation-tag-bg"
                :opacity="activeFaction === f.key || hoveredFaction === f.key ? 0.9 : 0.6"
              />
              <text
                :x="(centerX + getFactionPos(f.angle).x) / 2"
                :y="(centerY + getFactionPos(f.angle).y) / 2 + 4"
                class="relation-tag-text"
                :fill="f.color"
              >
                {{ t(`relations.factions.${f.key}.relation`) }}
              </text>
            </g>
          </svg>

          <!-- Faction cards -->
          <div class="faction-cards">
            <div
              v-for="f in factions"
              :key="f.key"
              :class="['faction-card', { active: activeFaction === f.key, dimmed: activeFaction && activeFaction !== f.key }]"
              :style="{ '--faction-color': f.color }"
              @click="toggleFaction(f.key)"
              @mouseenter="hoveredFaction = f.key"
              @mouseleave="hoveredFaction = null"
            >
              <div class="card-glow"></div>
              <div class="card-header">
                <div class="card-identity">
                  <h3 class="card-name">{{ t(`relations.factions.${f.key}.name`) }}</h3>
                  <span class="card-name-en">{{ t(`relations.factions.${f.key}.nameEn`) }}</span>
                </div>
                <div class="card-threat">
                  <span class="threat-label">{{ t('relations.threatLevel') }}</span>
                  <div class="threat-bar">
                    <div
                      class="threat-fill"
                      :style="{
                        width: `${threatLevels[t(`relations.factions.${f.key}.threat`)] || 0}%`,
                        background: threatColors[t(`relations.factions.${f.key}.threat`)] || '#5a5650',
                      }"
                    ></div>
                  </div>
                  <span class="threat-value" :style="{ color: threatColors[t(`relations.factions.${f.key}.threat`)] }">
                    {{ t(`relations.factions.${f.key}.threat`) }}
                  </span>
                </div>
              </div>
              <p class="card-desc">{{ t(`relations.factions.${f.key}.description`) }}</p>
              <div class="card-relation-tag" :style="{ borderColor: f.color, color: f.color }">
                {{ t(`relations.factions.${f.key}.relation`) }}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.relations {
  background: var(--color-bg-primary);
  overflow: hidden;
}

.viz-container {
  max-width: 1000px;
  margin: 0 auto;
}

.relations-svg {
  width: 100%;
  max-height: 420px;
  margin-bottom: var(--space-2xl);
}

/* Orbit rings */
.orbit-ring {
  fill: none;
  stroke: var(--color-border);
  stroke-width: 0.5;
  stroke-dasharray: 4 4;
  opacity: 0.4;
  animation: orbitSpin 60s linear infinite;
  transform-origin: 300px 250px;
}

.orbit-ring-inner {
  fill: none;
  stroke: var(--color-border);
  stroke-width: 0.3;
  opacity: 0.2;
  animation: orbitSpinReverse 45s linear infinite;
  transform-origin: 300px 250px;
}

.orbit-ring-outer {
  fill: none;
  stroke: var(--color-border);
  stroke-width: 0.3;
  opacity: 0.15;
  stroke-dasharray: 8 8;
  animation: orbitSpin 80s linear infinite;
  transform-origin: 300px 250px;
}

@keyframes orbitSpin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

@keyframes orbitSpinReverse {
  from { transform: rotate(360deg); }
  to { transform: rotate(0deg); }
}

/* Connection lines */
.connection-line {
  stroke-dasharray: 6 4;
  transition: opacity var(--duration-normal) ease;
  opacity: 0.5;
}

.connection-line.active {
  opacity: 1;
  stroke-dasharray: none;
  filter: url(#glow);
}

.connection-line-glow {
  transition: opacity var(--duration-normal) ease;
}

/* Particles */
.particle {
  filter: url(#glow);
}

/* Center node */
.center-glow {
  animation: centerPulse 3s ease-in-out infinite;
}

@keyframes centerPulse {
  0%, 100% { opacity: 0.6; transform: scale(1); }
  50% { opacity: 1; transform: scale(1.05); }
}

.center-ring-outer {
  fill: none;
  stroke: var(--color-accent);
  stroke-width: 1;
  stroke-dasharray: 3 6;
  opacity: 0.3;
  animation: orbitSpin 20s linear infinite;
  transform-origin: 300px 250px;
}

.center-ring {
  fill: none;
  stroke: var(--color-accent);
  stroke-width: 1.5;
  opacity: 0.6;
}

.center-bg {
  fill: var(--color-bg-secondary);
  stroke: var(--color-accent);
  stroke-width: 0.5;
  opacity: 0.9;
}

.center-text-main {
  font-family: var(--font-mono);
  font-size: 11px;
  font-weight: 700;
  fill: var(--color-accent);
  text-anchor: middle;
  letter-spacing: 0.1em;
}

.center-text-sub {
  font-family: var(--font-mono);
  font-size: 5px;
  fill: var(--color-text-muted);
  text-anchor: middle;
  letter-spacing: 0.05em;
}

/* Faction nodes */
.faction-node {
  cursor: pointer;
  transition: transform var(--duration-normal) var(--ease-out-expo),
              opacity var(--duration-normal) ease;
}

.faction-node:hover .faction-aura {
  r: 42;
  stroke-opacity: 0.6;
  fill-opacity: 0.15;
}

.faction-node.active .faction-aura {
  r: 44;
  stroke-opacity: 0.8;
  fill-opacity: 0.2;
  animation: auraPulse 2s ease-in-out infinite;
}

.faction-node.dimmed {
  opacity: 0.4;
}

@keyframes auraPulse {
  0%, 100% { r: 44; }
  50% { r: 48; }
}

.faction-bg {
  fill: var(--color-bg-secondary);
  transition: all var(--duration-normal) ease;
}

.faction-label {
  font-family: var(--font-mono);
  font-size: 7px;
  font-weight: 700;
  text-anchor: middle;
  dominant-baseline: middle;
  letter-spacing: 0.12em;
  pointer-events: none;
}

/* Relation tags */
.relation-tag-bg {
  fill: var(--color-bg-secondary);
  transition: opacity var(--duration-normal) ease;
}

.relation-tag-text {
  font-family: var(--font-mono);
  font-size: 6px;
  text-anchor: middle;
  dominant-baseline: middle;
  letter-spacing: 0.08em;
}

/* Faction cards */
.faction-cards {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-lg);
}

.faction-card {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-xl);
  cursor: pointer;
  position: relative;
  overflow: hidden;
  transition: all var(--duration-normal) var(--ease-out-expo);
}

.faction-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--faction-color);
  opacity: 0.3;
  transition: opacity var(--duration-normal) ease;
}

.faction-card:hover {
  border-color: var(--faction-color);
  transform: translateY(-4px);
  box-shadow: 0 8px 30px rgba(0, 0, 0, 0.3);
}

.faction-card:hover::before {
  opacity: 0.8;
}

.faction-card.active {
  border-color: var(--faction-color);
  background: var(--color-bg-card-hover);
}

.faction-card.active::before {
  opacity: 1;
  height: 3px;
}

.faction-card.dimmed {
  opacity: 0.5;
  transform: scale(0.98);
}

.card-glow {
  position: absolute;
  top: -50%;
  left: -50%;
  width: 200%;
  height: 200%;
  background: radial-gradient(circle at center, var(--faction-color), transparent 70%);
  opacity: 0;
  transition: opacity var(--duration-slow) ease;
  pointer-events: none;
}

.faction-card:hover .card-glow {
  opacity: 0.03;
}

.faction-card.active .card-glow {
  opacity: 0.05;
}

.card-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  margin-bottom: var(--space-md);
  gap: var(--space-md);
}

.card-identity {
  flex: 1;
}

.card-name {
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--color-text-primary);
  margin-bottom: 2px;
}

.card-name-en {
  font-family: var(--font-mono);
  font-size: 0.6rem;
  color: var(--color-text-muted);
  letter-spacing: 0.12em;
}

.card-threat {
  display: flex;
  flex-direction: column;
  align-items: flex-end;
  gap: 4px;
  min-width: 80px;
}

.threat-label {
  font-family: var(--font-mono);
  font-size: 0.55rem;
  color: var(--color-text-muted);
  letter-spacing: 0.1em;
  text-transform: uppercase;
}

.threat-bar {
  width: 100%;
  height: 3px;
  background: var(--color-bg-tertiary);
  border-radius: 2px;
  overflow: hidden;
}

.threat-fill {
  height: 100%;
  border-radius: 2px;
  transition: width 1s var(--ease-out-expo);
}

.threat-value {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  font-weight: 600;
  letter-spacing: 0.05em;
}

.card-desc {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  line-height: 1.7;
  margin-bottom: var(--space-md);
}

.card-relation-tag {
  display: inline-flex;
  font-family: var(--font-mono);
  font-size: 0.7rem;
  letter-spacing: 0.08em;
  padding: var(--space-xs) var(--space-sm);
  border: 1px solid;
  border-radius: var(--radius-sm);
  opacity: 0.8;
  transition: opacity var(--transition-fast) ease;
}

.faction-card:hover .card-relation-tag,
.faction-card.active .card-relation-tag {
  opacity: 1;
}

@media (max-width: 768px) {
  .relations-svg {
    max-height: 320px;
    margin-bottom: var(--space-xl);
  }

  .faction-cards {
    grid-template-columns: 1fr;
    gap: var(--space-md);
  }

  .faction-card {
    padding: var(--space-lg);
  }
}

@media (max-width: 480px) {
  .relations-svg {
    max-height: 260px;
  }

  .card-header {
    flex-direction: column;
    gap: var(--space-sm);
  }

  .card-threat {
    align-items: flex-start;
    width: 100%;
  }

  .threat-bar {
    width: 100%;
  }
}
</style>
