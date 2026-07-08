<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t, tm } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const hoveredNode = ref<string | null>(null)
const selectedNode = ref<string | null>(null)

const deptIcons = [
  'M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z',
  'M14.5 17.5L3 6V3h3l11.5 11.5M13 7l4-4 4 4-4 4M3 17l4 4 4-4',
  'M12 12m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0M12 2v4M12 18v4M4.93 4.93l2.83 2.83M16.24 16.24l2.83 2.83M2 12h4M18 12h4',
  'M13 2L3 14h9l-1 8 10-12h-9l1-8z',
]

const branchIcons = [
  'M3 12l3-3m0 0l3 3m-3-3v8M21 12l-3 3m0 0l-3-3m3 3V9',
  'M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zM4 12c0-1.95.7-3.74 1.87-5.13L12 13v8c-4.42-.75-8-4.54-8-9z',
  'M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z',
  'M17 21v-2a4 4 0 0 0 -4 -4h-6a4 4 0 0 0 -4 4v2M9 7a4 4 0 1 0 8 0a4 4 0 0 0 -8 0M23 21v-2a4 4 0 0 0 -3 -3.87M16 3.13a4 4 0 0 1 0 7.75',
]

const toggleNode = (key: string) => {
  selectedNode.value = selectedNode.value === key ? null : key
}
</script>

<template>
  <section id="structure" ref="sectionRef" class="section structure">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('structure.label') }}</span>
        <h2 class="section-title">{{ t('structure.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">{{ t('structure.subtitle') }}</p>
      </div>

      <!-- SVG Org Chart -->
      <div class="org-chart reveal-scale">
        <svg viewBox="0 0 900 560" class="org-svg">
          <defs>
            <filter id="orgGlow">
              <feGaussianBlur stdDeviation="4" result="blur" />
              <feMerge>
                <feMergeNode in="blur" />
                <feMergeNode in="SourceGraphic" />
              </feMerge>
            </filter>
            <radialGradient id="cmdGlow" cx="50%" cy="50%" r="50%">
              <stop offset="0%" stop-color="#4ae87a" stop-opacity="0.25" />
              <stop offset="100%" stop-color="#4ae87a" stop-opacity="0" />
            </radialGradient>
          </defs>

          <!-- Tier labels -->
          <text x="20" y="60" class="tier-label">{{ t('structure.chart.tier1') }}</text>
          <line x1="20" y1="65" x2="130" y2="65" stroke="#4ae87a" stroke-width="0.5" opacity="0.3" />

          <text x="20" y="230" class="tier-label">{{ t('structure.chart.tier2') }}</text>
          <line x1="20" y1="235" x2="130" y2="235" stroke="#4ae87a" stroke-width="0.5" opacity="0.3" />

          <text x="20" y="420" class="tier-label">{{ t('structure.chart.tier3') }}</text>
          <line x1="20" y1="425" x2="130" y2="425" stroke="#4ae87a" stroke-width="0.5" opacity="0.3" />

          <!-- Tier separator lines -->
          <line x1="100" y1="150" x2="800" y2="150" stroke="#4ae87a" stroke-width="0.3" stroke-dasharray="4 8" opacity="0.2" />
          <line x1="100" y1="350" x2="800" y2="350" stroke="#4ae87a" stroke-width="0.3" stroke-dasharray="4 8" opacity="0.2" />

          <!-- TIER 1: Standing Committee (Top Command) -->
          <g class="org-node cmd-node" @mouseenter="hoveredNode = 'cmd'" @mouseleave="hoveredNode = null">
            <circle cx="450" cy="65" r="55" fill="url(#cmdGlow)" />
            <circle cx="450" cy="65" r="42" class="cmd-ring-outer" />
            <circle cx="450" cy="65" r="34" class="cmd-ring" />
            <circle cx="450" cy="65" r="28" class="cmd-bg" />
            <path d="M450 45l5 10.2 11.2 1.6-8.1 7.9 1.9 11.2L450 71l-10 5.1 1.9-11.2-8.1-7.9L445 55.2z" fill="#4ae87a" opacity="0.9" />
            <text x="450" y="108" class="node-title-cmd">{{ (tm('structure.departments') as any[])[0]?.name }}</text>
            <text x="450" y="122" class="node-sub-cmd">{{ t('structure.commandSubtitle') }}</text>
          </g>

          <!-- Commander info attached to Standing Committee -->
          <g class="commander-badge">
            <rect x="600" y="40" width="220" height="52" rx="8" class="cmd-badge-bg" />
            <circle cx="628" cy="66" r="14" class="cmd-badge-avatar" />
            <svg x="620" y="58" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="#4ae87a" stroke-width="1.5">
              <path d="M20 21v-2a4 4 0 00-4-4H8a4 4 0 00-4 4v2" />
              <circle cx="12" cy="7" r="4" />
            </svg>
            <text x="652" y="60" class="badge-label">{{ t('structure.commanderLabel') }}</text>
            <text x="652" y="76" class="badge-name">{{ t('structure.commanderName') }}</text>
          </g>

          <!-- Connection: Standing Committee to Tier 2 departments -->
          <g class="connections-tier1">
            <path d="M450,100 L450,130 L300,130 L300,180" class="conn-line" fill="none" />
            <path d="M450,100 L450,130 L450,130 L450,180" class="conn-line" fill="none" />
            <path d="M450,100 L450,130 L600,130 L600,180" class="conn-line" fill="none" />
            <circle r="2.5" fill="#4ae87a" opacity="0.7">
              <animateMotion dur="2.5s" repeatCount="indefinite" path="M450,100 L450,130 L300,130 L300,180" />
            </circle>
            <circle r="2.5" fill="#4ae87a" opacity="0.7">
              <animateMotion dur="2.2s" repeatCount="indefinite" path="M450,100 L450,180" />
            </circle>
            <circle r="2.5" fill="#4ae87a" opacity="0.7">
              <animateMotion dur="2.8s" repeatCount="indefinite" path="M450,100 L450,130 L600,130 L600,180" />
            </circle>
          </g>

          <!-- TIER 2: Three departments (Standing Committee removed from here, it's at top) -->
          <!-- Military Staff Committee -->
          <g class="org-node dept-node" :class="{ dimmed: selectedNode && selectedNode !== 'dept-1' }" @mouseenter="hoveredNode = 'dept-1'" @mouseleave="hoveredNode = null" @click="toggleNode('dept-1')">
            <rect x="240" y="180" width="140" height="60" rx="8" class="dept-card" />
            <svg x="256" y="194" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#4a9ee8" stroke-width="1.5">
              <path :d="deptIcons[1]" />
            </svg>
            <text x="310" y="208" class="node-name">{{ (tm('structure.departments') as any[])[1]?.name }}</text>
            <text x="310" y="224" class="node-desc-short">Tactical Coord.</text>
            <line x1="250" y1="235" x2="370" y2="235" stroke="#4a9ee8" stroke-width="1" opacity="0.3" />
          </g>

          <!-- Independent Supervision Office -->
          <g class="org-node dept-node" :class="{ dimmed: selectedNode && selectedNode !== 'dept-2' }" @mouseenter="hoveredNode = 'dept-2'" @mouseleave="hoveredNode = null" @click="toggleNode('dept-2')">
            <rect x="410" y="180" width="140" height="60" rx="8" class="dept-card" />
            <svg x="426" y="194" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#e8a84a" stroke-width="1.5">
              <path :d="deptIcons[2]" />
            </svg>
            <text x="480" y="208" class="node-name">{{ (tm('structure.departments') as any[])[2]?.name }}</text>
            <text x="480" y="224" class="node-desc-short">Audit & Compliance</text>
            <line x1="420" y1="235" x2="540" y2="235" stroke="#e8a84a" stroke-width="1" opacity="0.3" />
          </g>

          <!-- Special Operations Division -->
          <g class="org-node dept-node dept-highlight" :class="{ dimmed: selectedNode && selectedNode !== 'dept-3' }" @mouseenter="hoveredNode = 'dept-3'" @mouseleave="hoveredNode = null" @click="toggleNode('dept-3')">
            <rect x="580" y="180" width="160" height="60" rx="8" class="dept-card dept-card-ops" />
            <svg x="596" y="194" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#4ae87a" stroke-width="1.5">
              <path :d="deptIcons[3]" />
            </svg>
            <text x="660" y="208" class="node-name node-name-ops">{{ (tm('structure.departments') as any[])[3]?.name }}</text>
            <text x="660" y="224" class="node-desc-short">Front-line Combat</text>
            <line x1="590" y1="235" x2="730" y2="235" stroke="#4ae87a" stroke-width="1.5" opacity="0.5" />
          </g>

          <!-- Connection: Special Ops Division down to branches -->
          <g class="connections-tier2">
            <path d="M660,240 L660,290 L200,290 L200,380" class="conn-line-ops" fill="none" />
            <path d="M660,240 L660,290 L400,290 L400,380" class="conn-line-ops" fill="none" />
            <path d="M660,240 L660,290 L600,290 L600,380" class="conn-line-ops" fill="none" />
            <path d="M660,240 L660,290 L800,290 L800,380" class="conn-line-ops" fill="none" />
            <circle r="2" fill="#4ae87a" opacity="0.6">
              <animateMotion dur="2.5s" repeatCount="indefinite" path="M660,240 L660,290 L200,290 L200,380" />
            </circle>
            <circle r="2" fill="#4ae87a" opacity="0.6">
              <animateMotion dur="2.2s" repeatCount="indefinite" path="M660,240 L660,290 L400,290 L400,380" />
            </circle>
            <circle r="2" fill="#4ae87a" opacity="0.6">
              <animateMotion dur="2.8s" repeatCount="indefinite" path="M660,240 L660,290 L600,290 L600,380" />
            </circle>
            <circle r="2" fill="#4ae87a" opacity="0.6">
              <animateMotion dur="3s" repeatCount="indefinite" path="M660,240 L660,290 L800,290 L800,380" />
            </circle>
          </g>

          <!-- TIER 3: Branch nodes (all under Special Operations Division) -->
          <g v-for="(branch, idx) in (tm('structure.branches') as string[])" :key="idx"
             class="org-node branch-node"
             :class="{ dimmed: selectedNode && !selectedNode.startsWith('branch') }"
             @mouseenter="hoveredNode = `branch-${idx}`"
             @mouseleave="hoveredNode = null">
            <rect :x="[140, 340, 540, 740][idx]" y="380" width="120" height="56" rx="8" class="branch-card" />
            <circle :cx="[170, 370, 570, 770][idx]" cy="400" r="12" class="branch-icon-circle" />
            <svg :x="[160, 360, 560, 760][idx]" y="390" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="#4ae87a" stroke-width="1.5">
              <path :d="branchIcons[idx]" />
            </svg>
            <text :x="[200, 400, 600, 800][idx]" y="405" class="node-name-branch">{{ branch }}</text>
            <line :x1="[150, 350, 550, 750][idx]" y1="431" :x2="[250, 450, 650, 850][idx]" y2="431" stroke="#4ae87a" stroke-width="0.5" opacity="0.3" />
          </g>

          <!-- Bracket indicating Special Ops oversees all branches -->
          <g class="branch-bracket">
            <line x1="160" y1="370" x2="160" y2="376" stroke="#4ae87a" stroke-width="1" opacity="0.4" />
            <line x1="860" y1="370" x2="860" y2="376" stroke="#4ae87a" stroke-width="1" opacity="0.4" />
            <line x1="160" y1="376" x2="860" y2="376" stroke="#4ae87a" stroke-width="1" opacity="0.4" stroke-dasharray="4 4" />
            <text x="510" y="373" class="bracket-label">{{ t('structure.chart.subordinate') }}</text>
          </g>
        </svg>
      </div>

      <!-- Detail cards below -->
      <div class="structure-details stagger-children">
        <!-- Standing Committee Card (Top Command) -->
        <div class="detail-card commander-card">
          <div class="card-accent-line"></div>
          <div class="commander-header">
            <div class="commander-avatar-wrap">
              <div class="avatar-ring"></div>
              <div class="avatar-inner">
                <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <path d="M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z" />
                </svg>
              </div>
            </div>
            <div class="commander-info">
              <h4 class="dept-detail-name">{{ (tm('structure.departments') as any[])[0]?.name }}</h4>
              <span class="info-label">{{ t('structure.commanderLabel') }}: {{ t('structure.commanderName') }}</span>
            </div>
          </div>
          <p class="command-detail">{{ t('structure.commandDetail') }}</p>
          <p class="command-detail">{{ (tm('structure.departments') as any[])[0]?.desc }}</p>
        </div>

        <!-- Department Cards -->
        <div v-for="(dept, idx) in (tm('structure.departments') as any[])" :key="idx" v-show="idx > 0" class="detail-card dept-detail-card" :style="{ '--dept-color': ['#4ae87a', '#4a9ee8', '#e8a84a', '#4ae87a'][idx] }">
          <div class="card-accent-line" :style="{ background: ['#4ae87a', '#4a9ee8', '#e8a84a', '#4ae87a'][idx] }"></div>
          <div class="dept-header">
            <div class="dept-icon-wrap" :style="{ borderColor: ['#4ae87a', '#4a9ee8', '#e8a84a', '#4ae87a'][idx] }">
              <svg viewBox="0 0 24 24" fill="none" :stroke="['#4ae87a', '#4a9ee8', '#e8a84a', '#4ae87a'][idx]" stroke-width="1.5">
                <path :d="deptIcons[idx]" />
              </svg>
            </div>
            <div>
              <h4 class="dept-detail-name">{{ dept.name }}</h4>
            </div>
          </div>
          <p class="dept-detail-desc">{{ dept.desc }}</p>
        </div>
      </div>

      <!-- Branches row -->
      <div class="branches-row reveal">
        <h4 class="branches-label">{{ t('structure.branchesTitle') }}</h4>
        <div class="branches-grid">
          <div v-for="(branch, idx) in (tm('structure.branches') as string[])" :key="idx" class="branch-pill" :style="{ '--delay': `${idx * 0.1}s` }">
            <span class="branch-dot"></span>
            <span class="branch-name">{{ branch }}</span>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.structure {
  background: var(--color-bg-secondary);
  overflow: hidden;
}

.org-chart {
  max-width: 960px;
  margin: 0 auto var(--space-3xl);
}

.org-svg {
  width: 100%;
  height: auto;
}

.tier-label {
  font-family: var(--font-mono);
  font-size: 8px;
  fill: var(--color-text-muted);
  letter-spacing: 0.15em;
  text-transform: uppercase;
}

/* Connection lines */
.conn-line {
  stroke: #4ae87a;
  stroke-width: 1.5;
  opacity: 0.3;
  stroke-dasharray: 6 4;
  animation: lineFlow 3s linear infinite;
}

.conn-line-ops {
  stroke: #4ae87a;
  stroke-width: 1.5;
  opacity: 0.3;
  stroke-dasharray: 6 4;
  animation: lineFlow 3s linear infinite;
}

@keyframes lineFlow {
  from { stroke-dashoffset: 0; }
  to { stroke-dashoffset: -20; }
}

/* Command node */
.cmd-ring-outer {
  fill: none;
  stroke: #4ae87a;
  stroke-width: 1;
  stroke-dasharray: 3 6;
  opacity: 0.4;
  animation: cmdSpin 25s linear infinite;
  transform-origin: 450px 65px;
}

.cmd-ring {
  fill: none;
  stroke: #4ae87a;
  stroke-width: 2;
  opacity: 0.7;
}

.cmd-bg {
  fill: var(--color-bg-secondary);
  stroke: #4ae87a;
  stroke-width: 0.5;
}

@keyframes cmdSpin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

.node-title-cmd {
  font-family: var(--font-mono);
  font-size: 9px;
  font-weight: 700;
  fill: var(--color-text-primary);
  text-anchor: middle;
  letter-spacing: 0.08em;
}

.node-sub-cmd {
  font-family: var(--font-mono);
  font-size: 6px;
  fill: var(--color-text-muted);
  text-anchor: middle;
}

/* Commander badge */
.cmd-badge-bg {
  fill: var(--color-bg-card);
  stroke: var(--color-border);
  stroke-width: 0.5;
}

.cmd-badge-avatar {
  fill: var(--color-accent-glow);
  stroke: var(--color-accent);
  stroke-width: 1;
}

.badge-label {
  font-family: var(--font-mono);
  font-size: 6px;
  fill: var(--color-text-muted);
  letter-spacing: 0.1em;
}

.badge-name {
  font-family: var(--font-mono);
  font-size: 8px;
  font-weight: 600;
  fill: var(--color-text-primary);
}

/* Department nodes */
.dept-card {
  fill: var(--color-bg-card);
  stroke: var(--color-border);
  stroke-width: 1;
  transition: all 0.3s ease;
  cursor: pointer;
}

.dept-card-ops {
  stroke: #4ae87a;
  stroke-opacity: 0.4;
  stroke-width: 1.5;
}

.dept-node:hover .dept-card {
  stroke: #4ae87a;
  stroke-opacity: 0.5;
  filter: url(#orgGlow);
}

.dept-node.dimmed {
  opacity: 0.4;
}

.node-name {
  font-family: var(--font-mono);
  font-size: 7px;
  font-weight: 600;
  fill: var(--color-text-primary);
  text-anchor: middle;
  letter-spacing: 0.03em;
}

.node-name-ops {
  fill: #4ae87a;
}

.node-desc-short {
  font-family: var(--font-mono);
  font-size: 5.5px;
  fill: var(--color-text-muted);
  text-anchor: middle;
}

/* Branch nodes */
.branch-card {
  fill: var(--color-bg-card);
  stroke: var(--color-border);
  stroke-width: 0.5;
  transition: all 0.3s ease;
  cursor: pointer;
}

.branch-node:hover .branch-card {
  stroke: #4ae87a;
  stroke-opacity: 0.4;
}

.branch-node.dimmed {
  opacity: 0.4;
}

.branch-icon-circle {
  fill: rgba(74, 232, 122, 0.1);
  stroke: #4ae87a;
  stroke-width: 0.5;
  stroke-opacity: 0.3;
}

.node-name-branch {
  font-family: var(--font-mono);
  font-size: 7px;
  font-weight: 500;
  fill: var(--color-text-secondary);
  text-anchor: middle;
}

.bracket-label {
  font-family: var(--font-mono);
  font-size: 6px;
  fill: #4ae87a;
  text-anchor: middle;
  letter-spacing: 0.1em;
  opacity: 0.6;
}

/* Detail cards below */
.structure-details {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: var(--space-lg);
  margin-bottom: var(--space-3xl);
}

.detail-card {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-xl);
  position: relative;
  overflow: hidden;
  transition: all var(--duration-normal) var(--ease-out-expo);
}

.detail-card:hover {
  border-color: var(--color-border-hover);
  transform: translateY(-3px);
  box-shadow: 0 8px 24px rgba(0, 0, 0, 0.2);
}

.card-accent-line {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: var(--color-accent);
  opacity: 0.5;
  transition: opacity 0.3s ease;
}

.detail-card:hover .card-accent-line {
  opacity: 1;
}

.commander-card {
  grid-column: 1 / -1;
}

.commander-header {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
  margin-bottom: var(--space-lg);
}

.commander-avatar-wrap {
  position: relative;
  width: 56px;
  height: 56px;
  flex-shrink: 0;
}

.avatar-ring {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  border-radius: 50%;
  border: 1.5px solid var(--color-accent);
  opacity: 0.5;
  animation: avatarPulse 3s ease-in-out infinite;
}

@keyframes avatarPulse {
  0%, 100% { transform: scale(1); opacity: 0.5; }
  50% { transform: scale(1.08); opacity: 0.8; }
}

.avatar-inner {
  position: absolute;
  top: 4px;
  right: 4px;
  bottom: 4px;
  left: 4px;
  border-radius: 50%;
  background: var(--color-accent-glow);
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--color-accent);
}

.avatar-inner svg {
  width: 24px;
  height: 24px;
}

.commander-info {
  flex: 1;
}

.info-label {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  color: var(--color-text-muted);
  letter-spacing: 0.1em;
}

.commander-name {
  font-size: 1.1rem;
  font-weight: 700;
  color: var(--color-text-primary);
  margin-top: 2px;
}

.command-detail {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  line-height: 1.7;
  margin-bottom: var(--space-sm);
}

.dept-detail-card {
  cursor: pointer;
}

.dept-header {
  display: flex;
  align-items: center;
  gap: var(--space-md);
  margin-bottom: var(--space-md);
}

.dept-icon-wrap {
  width: 36px;
  height: 36px;
  border-radius: 50%;
  border: 1px solid;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  opacity: 0.8;
  transition: all 0.3s ease;
}

.dept-detail-card:hover .dept-icon-wrap {
  opacity: 1;
  transform: scale(1.1);
}

.dept-icon-wrap svg {
  width: 18px;
  height: 18px;
}

.dept-detail-name {
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--color-text-primary);
}

.dept-detail-desc {
  font-size: 0.8rem;
  color: var(--color-text-secondary);
  line-height: 1.6;
}

/* Branches row */
.branches-row {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-2xl);
}

.branches-label {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  letter-spacing: 0.15em;
  color: var(--color-accent);
  margin-bottom: var(--space-lg);
  text-align: center;
}

.branches-grid {
  display: flex;
  justify-content: center;
  gap: var(--space-lg);
  flex-wrap: wrap;
}

.branch-pill {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-lg);
  border: 1px solid var(--color-border);
  border-radius: 100px;
  transition: all 0.3s var(--ease-out-expo);
  animation: pillIn 0.6s var(--ease-out-expo) var(--delay) both;
}

@keyframes pillIn {
  from {
    opacity: 0;
    transform: scale(0.9) translateY(8px);
  }
  to {
    opacity: 1;
    transform: scale(1) translateY(0);
  }
}

.branch-pill:hover {
  border-color: var(--color-accent);
  background: var(--color-accent-glow);
  transform: translateY(-2px);
  box-shadow: 0 4px 16px rgba(74, 232, 122, 0.1);
}

.branch-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--color-accent);
  animation: dotPulse 2s ease-in-out infinite;
}

@keyframes dotPulse {
  0%, 100% { opacity: 0.6; transform: scale(1); }
  50% { opacity: 1; transform: scale(1.2); }
}

.branch-name {
  font-family: var(--font-mono);
  font-size: 0.8rem;
  color: var(--color-text-secondary);
  letter-spacing: 0.05em;
}

.branch-pill:hover .branch-name {
  color: var(--color-accent);
}

@media (max-width: 900px) {
  .org-chart {
    overflow-x: auto;
    padding-bottom: var(--space-md);
  }

  .org-svg {
    min-width: 700px;
  }

  .structure-details {
    grid-template-columns: 1fr;
  }

  .commander-card {
    grid-column: auto;
  }
}

@media (max-width: 480px) {
  .branches-grid {
    gap: var(--space-sm);
  }

  .branch-pill {
    padding: var(--space-xs) var(--space-md);
  }
}
</style>
