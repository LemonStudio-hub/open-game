<script setup lang="ts">
import { ref } from 'vue'
import { useScrollReveal } from '../composables/useScrollReveal'

const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)

const scrollToAbout = () => {
  document.getElementById('about')?.scrollIntoView({ behavior: 'smooth' })
}
</script>

<template>
  <section id="hero" ref="sectionRef" class="hero">
    <div class="hero-bg">
      <div class="hero-grid"></div>
      <div class="hero-gradient"></div>
    </div>

    <div class="hero-content container">
      <div class="hero-badge reveal">
        <span class="badge-dot"></span>
        <span>CLASSIFIED // LEVEL-7 CLEARANCE</span>
      </div>

      <h1 class="hero-title reveal">
        <span class="hero-title-line">G<span class="title-dot">.</span>T<span class="title-dot">.</span>I<span class="title-dot">.</span></span>
        <span class="hero-title-sub">全球反恐特勤组</span>
      </h1>

      <p class="hero-motto reveal">
        "不让任何一个人掉队"
      </p>

      <p class="hero-description reveal">
        独立于联合国现有维和体系之外的全球性反恐机动力量<br />
        应对全球性恐怖主义威胁，维护国际和平与区域稳定
      </p>

      <div class="hero-stats reveal">
        <div class="stat">
          <span class="stat-value">2018</span>
          <span class="stat-label">成立年份</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat">
          <span class="stat-value">4</span>
          <span class="stat-label">兵种类别</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat">
          <span class="stat-value">14+</span>
          <span class="stat-label">精英干员</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat">
          <span class="stat-value">全球</span>
          <span class="stat-label">部署范围</span>
        </div>
      </div>

      <div class="hero-cta reveal">
        <a href="#about" class="btn-primary" @click.prevent="scrollToAbout">
          <span>了解更多</span>
          <svg width="16" height="16" viewBox="0 0 16 16" fill="none">
            <path d="M8 3v10M3 8l5 5 5-5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
          </svg>
        </a>
      </div>
    </div>

    <div class="hero-scroll-indicator">
      <div class="scroll-line"></div>
    </div>
  </section>
</template>

<style scoped>
.hero {
  min-height: 100vh;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
  overflow: hidden;
}

.hero-bg {
  position: absolute;
  inset: 0;
  z-index: 0;
}

.hero-grid {
  position: absolute;
  inset: 0;
  background-image:
    linear-gradient(rgba(201, 168, 76, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(201, 168, 76, 0.03) 1px, transparent 1px);
  background-size: 60px 60px;
  mask-image: radial-gradient(ellipse at center, black 30%, transparent 70%);
}

.hero-gradient {
  position: absolute;
  inset: 0;
  background:
    radial-gradient(ellipse 80% 50% at 50% -20%, rgba(201, 168, 76, 0.08), transparent),
    radial-gradient(ellipse 60% 40% at 50% 100%, rgba(201, 168, 76, 0.05), transparent);
}

.hero-content {
  position: relative;
  z-index: 1;
  text-align: center;
  padding-top: var(--nav-height);
}

.hero-badge {
  display: inline-flex;
  align-items: center;
  gap: var(--space-sm);
  font-family: var(--font-mono);
  font-size: 0.7rem;
  letter-spacing: 0.2em;
  color: var(--color-accent);
  padding: var(--space-xs) var(--space-md);
  border: 1px solid var(--color-border);
  border-radius: 2px;
  margin-bottom: var(--space-2xl);
}

.badge-dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: var(--color-accent);
  animation: pulse-glow 2s ease-in-out infinite;
}

.hero-title {
  margin-bottom: var(--space-lg);
}

.hero-title-line {
  display: block;
  font-family: var(--font-mono);
  font-size: clamp(3.5rem, 10vw, 7rem);
  font-weight: 800;
  letter-spacing: 0.15em;
  color: var(--color-text-primary);
  line-height: 1;
}

.title-dot {
  color: var(--color-accent);
}

.hero-title-sub {
  display: block;
  font-family: var(--font-display);
  font-size: clamp(1rem, 2.5vw, 1.4rem);
  font-weight: 300;
  letter-spacing: 0.5em;
  color: var(--color-text-secondary);
  margin-top: var(--space-sm);
}

.hero-motto {
  font-size: clamp(1.1rem, 2vw, 1.4rem);
  font-weight: 300;
  font-style: italic;
  color: var(--color-accent);
  margin-bottom: var(--space-lg);
  letter-spacing: 0.05em;
}

.hero-description {
  font-size: 1rem;
  color: var(--color-text-secondary);
  line-height: 1.8;
  margin-bottom: var(--space-3xl);
}

.hero-stats {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-xl);
  margin-bottom: var(--space-3xl);
  flex-wrap: wrap;
}

.stat {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: var(--space-xs);
}

.stat-value {
  font-family: var(--font-mono);
  font-size: 1.5rem;
  font-weight: 700;
  color: var(--color-text-primary);
}

.stat-label {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  letter-spacing: 0.1em;
}

.stat-divider {
  width: 1px;
  height: 40px;
  background: var(--color-border);
}

.hero-cta {
  margin-bottom: var(--space-3xl);
}

.btn-primary {
  display: inline-flex;
  align-items: center;
  gap: var(--space-sm);
  padding: var(--space-sm) var(--space-xl);
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--color-bg-primary);
  background: var(--color-accent);
  border-radius: var(--radius-sm);
  transition: all var(--transition-base);
  letter-spacing: 0.05em;
}

.btn-primary:hover {
  background: var(--color-accent-light);
  color: var(--color-bg-primary);
  box-shadow: 0 0 30px rgba(201, 168, 76, 0.3);
}

.hero-scroll-indicator {
  position: absolute;
  bottom: var(--space-xl);
  left: 50%;
  transform: translateX(-50%);
  z-index: 1;
}

.scroll-line {
  width: 1px;
  height: 40px;
  background: linear-gradient(to bottom, var(--color-accent), transparent);
  animation: scan-line 2s ease-in-out infinite;
}

@media (max-width: 768px) {
  .hero-stats {
    gap: var(--space-md);
  }

  .stat-divider {
    display: none;
  }

  .hero-badge {
    font-size: 0.6rem;
  }
}
</style>
