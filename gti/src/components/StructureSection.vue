<script setup lang="ts">
import { ref } from 'vue'
import { useScrollReveal } from '../composables/useScrollReveal'

const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const departments = [
  { name: '常务委员会', desc: '由安理会指派委员组成，全权负责组织决策', icon: '◆' },
  { name: '军事参谋团', desc: '由各成员国派遣高级军官组成，负责作战计划制定与战术协调', icon: '◈' },
  { name: '独立监察办公室', desc: '名义上负责内部审计与合规监督，受常务委员会节制', icon: '◇' },
  { name: '特勤处', desc: '一线作战力量，由各兵种干员组成，即玩家所在作战单元', icon: '◉' },
]

const branches = ['北美分部', '亚洲分部', '大洋洲分部', '非洲分部']
</script>

<template>
  <section id="structure" ref="sectionRef" class="section structure">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">// STRUCTURE</span>
        <h2 class="section-title">组织架构</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          常务委员会集体领导制下的高效反恐作战体系
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
                <h3 class="command-title">最高指挥层</h3>
                <p class="command-subtitle">常务委员会集体领导制</p>
              </div>
            </div>
            <div class="command-detail">
              <p>
                由联合国安理会指派委员组成，全权负责组织所有重大决策。委员在法理上向安理会负责，但在实际操作中拥有高度独立的决策权限。
              </p>
              <div class="command-note">
                <span class="note-label">制度张力</span>
                <span>委员会名义上由安理会指派，但一旦任命后"全权负责"，实质上切断了安理会对具体行动的实时干预通道。</span>
              </div>
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
              <span class="commander-label">现任总指挥</span>
              <h4 class="commander-name">多米尼克·圣地亚哥 中将</h4>
            </div>
          </div>
        </div>

        <div class="structure-side">
          <div class="departments reveal">
            <h4 class="side-title">附属机构</h4>
            <div class="dept-list">
              <div v-for="dept in departments" :key="dept.name" class="dept-item">
                <span class="dept-icon">{{ dept.icon }}</span>
                <div>
                  <h5 class="dept-name">{{ dept.name }}</h5>
                  <p class="dept-desc">{{ dept.desc }}</p>
                </div>
              </div>
            </div>
          </div>

          <div class="branches reveal">
            <h4 class="side-title">全球分部</h4>
            <div class="branch-grid">
              <div v-for="branch in branches" :key="branch" class="branch-tag">
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

.command-note {
  background: rgba(201, 168, 76, 0.05);
  border-left: 2px solid var(--color-accent);
  padding: var(--space-md);
  border-radius: 0 var(--radius-sm) var(--radius-sm) 0;
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  line-height: 1.6;
}

.note-label {
  display: block;
  font-family: var(--font-mono);
  font-size: 0.7rem;
  color: var(--color-accent);
  letter-spacing: 0.1em;
  margin-bottom: var(--space-xs);
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
  font-size: 1.2rem;
  line-height: 1;
  margin-top: 2px;
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
}
</style>
