<template>
  <footer class="footer">
    <div class="container">
      <div class="footer-content">
        <!-- Brand -->
        <div class="footer-brand">
          <a :href="localizedPath('/')" class="logo">
            <svg width="32" height="32" viewBox="0 0 32 32" fill="none">
              <circle cx="16" cy="16" r="14" stroke="currentColor" stroke-width="2" />
              <circle cx="16" cy="16" r="6" fill="currentColor" />
              <path d="M16 2 L16 8" stroke="currentColor" stroke-width="2" />
              <path d="M16 24 L16 30" stroke="currentColor" stroke-width="2" />
              <path d="M2 16 L8 16" stroke="currentColor" stroke-width="2" />
              <path d="M24 16 L30 16" stroke="currentColor" stroke-width="2" />
            </svg>
            <span>OpenOS</span>
          </a>
          <p>{{ t.footer.description }}</p>
        </div>

        <!-- Links -->
        <div class="footer-links">
          <!-- Product -->
          <div class="link-group">
            <h4>{{ t.footer.links.product.title }}</h4>
            <ul>
              <li v-for="(link, index) in t.footer.links.product.items" :key="index">
                <a :href="link.href">{{ link.label }}</a>
              </li>
            </ul>
          </div>

          <!-- Resources -->
          <div class="link-group">
            <h4>{{ t.footer.links.resources.title }}</h4>
            <ul>
              <li v-for="(link, index) in t.footer.links.resources.items" :key="index">
                <a :href="link.href">{{ link.label }}</a>
              </li>
            </ul>
          </div>

          <!-- Community -->
          <div class="link-group">
            <h4>{{ t.footer.links.community.title }}</h4>
            <ul>
              <li v-for="(link, index) in t.footer.links.community.items" :key="index">
                <a :href="link.href" target="_blank" rel="noopener noreferrer">
                  {{ link.label }}
                </a>
              </li>
            </ul>
          </div>
        </div>
      </div>

      <!-- Bottom -->
      <div class="footer-bottom">
        <p>{{ t.footer.copyright }}</p>
        <p>{{ t.footer.madeWith }}</p>
      </div>
    </div>
  </footer>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { useTranslations, getLocalizedPath, type Locale } from '../i18n';

const props = defineProps<{
  locale: Locale;
}>();

const t = computed(() => useTranslations(props.locale));

const localizedPath = (path: string) => getLocalizedPath(path, props.locale);
</script>

<style scoped>
.footer {
  background: var(--color-bg-secondary);
  border-top: 1px solid var(--color-border);
  padding: 4rem 0 2rem;
}

.footer-content {
  display: grid;
  grid-template-columns: 1fr 2fr;
  gap: 4rem;
  margin-bottom: 3rem;
}

.footer-brand .logo {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  color: var(--color-text-primary);
  font-weight: 700;
  font-size: 1.25rem;
  text-decoration: none;
  margin-bottom: 1rem;
}

.footer-brand p {
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  line-height: 1.6;
  max-width: 300px;
}

.footer-links {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 2rem;
}

.link-group h4 {
  color: var(--color-text-primary);
  font-size: 0.875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin-bottom: 1rem;
}

.link-group ul {
  list-style: none;
  padding: 0;
  margin: 0;
}

.link-group li {
  margin-bottom: 0.5rem;
}

.link-group a {
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  text-decoration: none;
  transition: color var(--transition-fast);
}

.link-group a:hover {
  color: var(--color-primary-light);
}

.footer-bottom {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding-top: 2rem;
  border-top: 1px solid var(--color-border);
}

.footer-bottom p {
  color: var(--color-text-muted);
  font-size: 0.75rem;
}

@media (max-width: 1024px) {
  .footer-content {
    grid-template-columns: 1fr;
    gap: 2rem;
  }
}

@media (max-width: 640px) {
  .footer-links {
    grid-template-columns: 1fr;
    gap: 1.5rem;
  }

  .footer-bottom {
    flex-direction: column;
    gap: 0.5rem;
    text-align: center;
  }
}
</style>
