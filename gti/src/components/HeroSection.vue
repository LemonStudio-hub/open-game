<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t } = useI18n()
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
        <span class="badge-text">{{ t('hero.badge') }}</span>
      </div>

      <h1 class="hero-title reveal">
        <span class="hero-title-line">
          <span class="title-letter" style="--delay: 0">G</span><span class="title-dot">.</span><span class="title-letter" style="--delay: 1">T</span><span class="title-dot">.</span><span class="title-letter" style="--delay: 2">I</span><span class="title-dot">.</span>
        </span>
        <span class="hero-title-sub">{{ t('hero.subtitle') }}</span>
      </h1>

      <p class="hero-motto reveal">
        "{{ t('hero.motto') }}"
      </p>

      <p class="hero-description reveal">
        <span class="desc-line">{{ t('hero.description[0]') }}</span>
        <span class="desc-line">{{ t('hero.description[1]') }}</span>
      </p>

      <div class="hero-stats reveal">
        <div class="stat">
          <span class="stat-value">2018</span>
          <span class="stat-label">{{ t('hero.stats.year') }}</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat">
          <span class="stat-value">4</span>
          <span class="stat-label">{{ t('hero.stats.classes') }}</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat">
          <span class="stat-value">14+</span>
          <span class="stat-label">{{ t('hero.stats.operators') }}</span>
        </div>
        <div class="stat-divider"></div>
        <div class="stat">
          <span class="stat-value">{{ t('hero.stats.scopeValue') }}</span>
          <span class="stat-label">{{ t('hero.stats.scope') }}</span>
        </div>
      </div>

      <div class="hero-cta reveal">
        <a href="#about" class="btn-primary" @click.prevent="scrollToAbout">
          <span>{{ t('hero.cta') }}</span>
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
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  z-index: 0;
}

.hero-grid {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  background-image:
    linear-gradient(rgba(74, 232, 122, 0.03) 1px, transparent 1px),
    linear-gradient(90deg, rgba(74, 232, 122, 0.03) 1px, transparent 1px);
  background-size: 60px 60px;
  -webkit-mask-image: radial-gradient(ellipse at center, black 30%, transparent 70%);
  mask-image: radial-gradient(ellipse at center, black 30%, transparent 70%);
}

.hero-gradient {
  position: absolute;
  top: 0;
  right: 0;
  bottom: 0;
  left: 0;
  background:
    radial-gradient(ellipse 80% 50% at 50% -20%, rgba(74, 232, 122, 0.08), transparent),
    radial-gradient(ellipse 60% 40% at 50% 100%, rgba(74, 232, 122, 0.05), transparent);
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

.title-letter {
  display: inline-block;
  opacity: 0;
  transform: translateY(30px);
  animation: titleReveal 0.8s var(--ease-out-expo) forwards;
  animation-delay: calc(var(--delay) * 0.1s + 0.3s);
}

@keyframes titleReveal {
  from {
    opacity: 0;
    transform: translateY(30px) scale(0.9);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
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
  font-size: clamp(0.85rem, 2vw, 1rem);
  color: var(--color-text-secondary);
  line-height: 1.8;
  margin-bottom: var(--space-3xl);
  padding: 0 var(--space-md);
}

.desc-line {
  display: block;
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
  box-shadow: 0 0 30px rgba(74, 232, 122, 0.3);
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
  .hero {
    min-height: 100vh;
    min-height: 100dvh;
  }

  .hero-content {
    padding-top: calc(var(--nav-height) + var(--space-lg));
    padding-bottom: var(--space-3xl);
  }

  .hero-badge {
    font-size: 0.6rem;
    margin-bottom: var(--space-lg);
  }

  .hero-stats {
    gap: var(--space-md);
  }

  .stat-divider {
    display: none;
  }

  .hero-cta {
    margin-bottom: var(--space-xl);
  }

  .hero-scroll-indicator {
    display: none;
  }
}

@media (max-width: 480px) {
  .hero-stats {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: var(--space-lg);
    text-align: center;
  }

  .hero-title-sub {
    letter-spacing: 0.2em;
  }
}

@media (max-height: 500px) and (orientation: landscape) {
  .hero {
    min-height: auto;
    padding: var(--space-xl) 0;
  }

  .hero-content {
    padding-top: var(--space-lg);
  }

  .hero-badge {
    margin-bottom: var(--space-md);
  }

  .hero-title {
    margin-bottom: var(--space-sm);
  }

  .hero-motto {
    margin-bottom: var(--space-md);
  }

  .hero-description {
    margin-bottom: var(--space-lg);
  }

  .hero-stats {
    margin-bottom: var(--space-lg);
  }

  .hero-cta {
    margin-bottom: 0;
  }

  .hero-scroll-indicator {
    display: none;
  }
}
</style>
