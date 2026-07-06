<script setup lang="ts">
import { ref } from 'vue'
import { useScrollReveal } from '../composables/useScrollReveal'

const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)
</script>

<template>
  <section id="about" ref="sectionRef" class="section about">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">// OVERVIEW</span>
        <h2 class="section-title">关于 G.T.I.</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          全球反恐特勤组 — 独立于联合国维和体系之外的全球性反恐机动力量
        </p>
      </div>

      <div class="about-grid">
        <div class="about-card reveal">
          <div class="card-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <circle cx="12" cy="12" r="10"/>
              <path d="M2 12h20M12 2a15.3 15.3 0 014 10 15.3 15.3 0 01-4 10 15.3 15.3 0 01-4-10 15.3 15.3 0 014-10z"/>
            </svg>
          </div>
          <h3 class="card-title">全球部署</h3>
          <p class="card-text">
            在全球设有多处分部，包括北美分部、亚洲分部、大洋洲分部、非洲分部等，干员来自各成员国，可快速响应全球任意区域的恐怖主义威胁。
          </p>
        </div>

        <div class="about-card reveal">
          <div class="card-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
            </svg>
          </div>
          <h3 class="card-title">合法授权</h3>
          <p class="card-text">
            经联合国安理会决议正式成立，在人事、预算与行动指挥上保有高度自主权，拥有采取一切必要手段维护地区稳定的合法授权。
          </p>
        </div>

        <div class="about-card reveal">
          <div class="card-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M17 21v-2a4 4 0 00-4-4H5a4 4 0 00-4 4v2"/>
              <circle cx="9" cy="7" r="4"/>
              <path d="M23 21v-2a4 4 0 00-3-3.87M16 3.13a4 4 0 010 7.75"/>
            </svg>
          </div>
          <h3 class="card-title">精英干员</h3>
          <p class="card-text">
            精锐武装核心最初抽调自美军三角洲特种部队，随后通过各成员国派遣制度，逐步吸纳来自全球的特战精英，组成世界顶尖特战力量。
          </p>
        </div>

        <div class="about-card reveal">
          <div class="card-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M22 12h-4l-3 9L9 3l-3 9H2"/>
            </svg>
          </div>
          <h3 class="card-title">核心使命</h3>
          <p class="card-text">
            遏制哈夫克集团对全球安全的威胁，争夺并控制曼德尔砖，阻止哈夫克秘密实验，破坏"巴别塔计划"，瓦解其太空军事布局。
          </p>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.about {
  background: var(--color-bg-secondary);
}

.about-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(260px, 1fr));
  gap: var(--space-xl);
}

.about-card {
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-2xl);
  transition: all var(--transition-base);
  position: relative;
  overflow: hidden;
}

.about-card::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  height: 2px;
  background: linear-gradient(90deg, transparent, var(--color-accent), transparent);
  opacity: 0;
  transition: opacity var(--transition-base);
}

.about-card:hover {
  border-color: var(--color-border-hover);
  background: var(--color-bg-card-hover);
  transform: translateY(-4px);
  box-shadow: var(--shadow-accent);
}

.about-card:hover::before {
  opacity: 1;
}

.card-icon {
  width: 48px;
  height: 48px;
  color: var(--color-accent);
  margin-bottom: var(--space-lg);
}

.card-icon svg {
  width: 100%;
  height: 100%;
}

.card-title {
  font-size: 1.15rem;
  font-weight: 600;
  margin-bottom: var(--space-sm);
  color: var(--color-text-primary);
}

.card-text {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  line-height: 1.7;
}
</style>
