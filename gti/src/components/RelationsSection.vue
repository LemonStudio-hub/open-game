<script setup lang="ts">
import { ref } from 'vue'
import { useScrollReveal } from '../composables/useScrollReveal'

const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const factions = [
  {
    name: '哈夫克集团',
    nameEn: 'HAVOC',
    relation: '全面对抗',
    description: '全球市值最高的科技企业，兼具恐怖活动与军事扩张属性，GTI核心打击目标。秘密研发曼德尔砖等高价值军事技术，其"巴别塔计划"对全球安全构成根本性威胁。',
    color: 'var(--color-havoc)',
    icon: '⚔',
  },
  {
    name: '阿萨拉卫队',
    nameEn: 'ASSARA',
    relation: '三方混战',
    description: '阿萨拉本土武装力量，既反抗哈夫克集团的侵略与控制，又坚决抗拒外部势力介入其领土主权。与GTI形成复杂的三方博弈格局。',
    color: 'var(--color-assara)',
    icon: '⛊',
  },
  {
    name: '联合国',
    nameEn: 'UNITED NATIONS',
    relation: '名义隶属',
    description: '安理会授权成立并提供法理依据，名义上负责预算框架管理。实际指挥独立运作，形成独特的"委托-代理"治理结构。',
    color: 'var(--color-text-secondary)',
    icon: '◈',
  },
]
</script>

<template>
  <section id="relations" ref="sectionRef" class="section relations">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">// RELATIONS</span>
        <h2 class="section-title">势力关系</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          错综复杂的国际博弈格局
        </p>
      </div>

      <div class="relations-grid stagger-children">
        <div
          v-for="faction in factions"
          :key="faction.name"
          class="faction-card"
          :style="{ '--faction-color': faction.color }"
        >
          <div class="faction-header">
            <div class="faction-icon">{{ faction.icon }}</div>
            <div class="faction-identity">
              <h3 class="faction-name">{{ faction.name }}</h3>
              <span class="faction-name-en">{{ faction.nameEn }}</span>
            </div>
            <span class="faction-relation">{{ faction.relation }}</span>
          </div>
          <p class="faction-desc">{{ faction.description }}</p>
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
  grid-template-columns: repeat(auto-fit, minmax(320px, 1fr));
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
}
</style>
