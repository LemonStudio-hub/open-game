<script setup lang="ts">
import { useI18n } from 'vue-i18n'

const { t } = useI18n()
</script>

<template>
  <footer class="footer">
    <div class="container">
      <div class="footer-top">
        <div class="footer-brand">
          <div class="footer-logo">
            <svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M16 2L4 10v12l12 8 12-8V10L16 2z" stroke="currentColor" stroke-width="1.5" fill="none"/>
              <path d="M16 8l-6 4v8l6 4 6-4v-8l-6-4z" fill="currentColor" opacity="0.3"/>
              <path d="M16 12v8M12 14l4 2 4-2" stroke="currentColor" stroke-width="1.5"/>
            </svg>
            <span class="footer-gti">G.T.I.</span>
          </div>
          <p class="footer-tagline">{{ t('footer.tagline') }}</p>
          <p class="footer-motto">"{{ t('footer.motto') }}"</p>
        </div>

        <div class="footer-links">
          <div class="footer-column">
            <h4 class="footer-col-title">{{ t('footer.navTitle') }}</h4>
            <a href="#about">{{ t('footer.links.about') }}</a>
            <a href="#history">{{ t('footer.links.history') }}</a>
            <a href="#structure">{{ t('footer.links.structure') }}</a>
          </div>
          <div class="footer-column">
            <h4 class="footer-col-title">{{ t('footer.moreTitle') }}</h4>
            <a href="#operators">{{ t('footer.links.operators') }}</a>
            <a href="#operations">{{ t('footer.links.operations') }}</a>
            <a href="#relations">{{ t('footer.links.relations') }}</a>
          </div>
        </div>
      </div>

      <div class="footer-bottom">
        <p class="footer-copyright">
          {{ t('footer.copyright') }}
        </p>
        <p class="footer-disclaimer">
          {{ t('footer.disclaimer') }}
        </p>
      </div>
    </div>
  </footer>
</template>

<style scoped>
.footer {
  background: var(--color-bg-tertiary);
  border-top: 1px solid var(--color-border);
  padding: var(--space-4xl) 0 var(--space-2xl);
}

.footer-top {
  display: flex;
  justify-content: space-between;
  gap: var(--space-3xl);
  margin-bottom: var(--space-3xl);
}

.footer-brand {
  max-width: 300px;
}

.footer-logo {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  color: var(--color-accent);
  margin-bottom: var(--space-sm);
}

.footer-logo svg {
  width: 28px;
  height: 28px;
}

.footer-gti {
  font-family: var(--font-mono);
  font-size: 1rem;
  font-weight: 700;
  letter-spacing: 0.15em;
}

.footer-tagline {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  margin-bottom: var(--space-xs);
}

.footer-motto {
  font-size: 0.8rem;
  font-style: italic;
  color: var(--color-accent-dark);
}

.footer-links {
  display: flex;
  gap: var(--space-3xl);
  flex-wrap: wrap;
}

.footer-column {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.footer-col-title {
  font-family: var(--font-mono);
  font-size: 0.7rem;
  letter-spacing: 0.15em;
  color: var(--color-accent);
  margin-bottom: var(--space-sm);
}

.footer-column a {
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  transition: color var(--transition-fast);
}

.footer-column a:hover {
  color: var(--color-text-primary);
}

.footer-bottom {
  border-top: 1px solid var(--color-border);
  padding-top: var(--space-xl);
  text-align: center;
}

.footer-copyright {
  font-size: 0.8rem;
  color: var(--color-text-muted);
  margin-bottom: var(--space-xs);
}

.footer-disclaimer {
  font-size: 0.7rem;
  color: var(--color-text-muted);
  opacity: 0.6;
}

@media (max-width: 768px) {
  .footer-top {
    flex-direction: column;
    gap: var(--space-2xl);
  }

  .footer-links {
    gap: var(--space-2xl);
  }
}

@media (max-width: 480px) {
  .footer-links {
    gap: var(--space-xl);
  }

  .footer-column {
    min-width: 0;
    flex: 1;
  }
}
</style>
