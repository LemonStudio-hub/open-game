<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)
</script>

<template>
  <section id="about" ref="sectionRef" class="section about">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('about.label') }}</span>
        <h2 class="section-title">{{ t('about.title') }}</h2>
        <div class="divider"></div>
        <p class="section-subtitle">
          {{ t('about.subtitle') }}
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
          <h3 class="card-title">{{ t('about.cards.global.title') }}</h3>
          <p class="card-text">
            {{ t('about.cards.global.text') }}
          </p>
        </div>

        <div class="about-card reveal">
          <div class="card-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z"/>
            </svg>
          </div>
          <h3 class="card-title">{{ t('about.cards.legal.title') }}</h3>
          <p class="card-text">
            {{ t('about.cards.legal.text') }}
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
          <h3 class="card-title">{{ t('about.cards.elite.title') }}</h3>
          <p class="card-text">
            {{ t('about.cards.elite.text') }}
          </p>
        </div>

        <div class="about-card reveal">
          <div class="card-icon">
            <svg viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
              <path d="M22 12h-4l-3 9L9 3l-3 9H2"/>
            </svg>
          </div>
          <h3 class="card-title">{{ t('about.cards.mission.title') }}</h3>
          <p class="card-text">
            {{ t('about.cards.mission.text') }}
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
  grid-template-columns: repeat(auto-fit, minmax(min(260px, 100%), 1fr));
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

@media (max-width: 480px) {
  .about-card {
    padding: var(--space-lg);
  }

  .card-icon {
    width: 40px;
    height: 40px;
    margin-bottom: var(--space-md);
  }
}
</style>
