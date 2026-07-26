<template>
  <nav class="navbar">
    <div class="container">
      <div class="nav-content">
        <!-- Logo -->
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

        <!-- Desktop Navigation -->
        <div class="nav-links">
          <a :href="localizedPath('/#features')" class="nav-link">
            {{ t.nav.features }}
          </a>
          <a :href="localizedPath('/#architecture')" class="nav-link">
            {{ t.nav.architecture }}
          </a>
          <a :href="localizedPath('/#code')" class="nav-link">
            {{ t.nav.code }}
          </a>
          <a :href="localizedPath('/#docs')" class="nav-link">
            {{ t.nav.docs }}
          </a>
        </div>

        <!-- Actions -->
        <div class="nav-actions">
          <!-- Language Switcher -->
          <button
            class="lang-switcher"
            @click="toggleLanguage"
            :title="locale === 'en' ? 'Switch to Chinese' : 'Switch to English'"
          >
            <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <circle cx="12" cy="12" r="10" />
              <line x1="2" y1="12" x2="22" y2="12" />
              <path d="M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" />
            </svg>
            <span>{{ locale === 'en' ? '中文' : 'EN' }}</span>
          </button>

          <!-- GitHub Button -->
          <a
            href="https://github.com"
            target="_blank"
            rel="noopener noreferrer"
            class="github-btn"
          >
            <svg width="20" height="20" viewBox="0 0 24 24" fill="currentColor">
              <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0 0 24 12c0-6.63-5.37-12-12-12z" />
            </svg>
            <span>{{ t.nav.github }}</span>
          </a>

          <!-- Mobile Menu Toggle -->
          <button class="mobile-menu-btn" @click="toggleMobileMenu">
            <svg width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
              <line v-if="!mobileMenuOpen" x1="3" y1="6" x2="21" y2="6" />
              <line v-if="!mobileMenuOpen" x1="3" y1="12" x2="21" y2="12" />
              <line v-if="!mobileMenuOpen" x1="3" y1="18" x2="21" y2="18" />
              <line v-if="mobileMenuOpen" x1="6" y1="6" x2="18" y2="18" />
              <line v-if="mobileMenuOpen" x1="6" y1="18" x2="18" y2="6" />
            </svg>
          </button>
        </div>
      </div>

      <!-- Mobile Menu -->
      <div v-if="mobileMenuOpen" class="mobile-menu">
        <a :href="localizedPath('/#features')" class="mobile-link" @click="closeMobileMenu">
          {{ t.nav.features }}
        </a>
        <a :href="localizedPath('/#architecture')" class="mobile-link" @click="closeMobileMenu">
          {{ t.nav.architecture }}
        </a>
        <a :href="localizedPath('/#code')" class="mobile-link" @click="closeMobileMenu">
          {{ t.nav.code }}
        </a>
        <a :href="localizedPath('/#docs')" class="mobile-link" @click="closeMobileMenu">
          {{ t.nav.docs }}
        </a>
      </div>
    </div>
  </nav>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useTranslations, getLocalizedPath, type Locale } from '../i18n';

const props = defineProps<{
  locale: Locale;
}>();

const mobileMenuOpen = ref(false);

const t = computed(() => useTranslations(props.locale));

const localizedPath = (path: string) => getLocalizedPath(path, props.locale);

const toggleLanguage = () => {
  const newLocale = props.locale === 'en' ? 'zh' : 'en';
  const newPath = getLocalizedPath(window.location.pathname, newLocale);
  window.location.href = newPath;
};

const toggleMobileMenu = () => {
  mobileMenuOpen.value = !mobileMenuOpen.value;
};

const closeMobileMenu = () => {
  mobileMenuOpen.value = false;
};
</script>

<style scoped>
.navbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 100;
  background: rgba(10, 10, 15, 0.8);
  backdrop-filter: blur(12px);
  border-bottom: 1px solid var(--color-border);
}

.nav-content {
  display: flex;
  align-items: center;
  justify-content: space-between;
  height: 4rem;
}

.logo {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  color: var(--color-text-primary);
  font-weight: 700;
  font-size: 1.25rem;
  text-decoration: none;
}

.logo:hover {
  color: var(--color-primary-light);
}

.nav-links {
  display: flex;
  gap: 2rem;
}

.nav-link {
  color: var(--color-text-secondary);
  text-decoration: none;
  font-size: 0.875rem;
  font-weight: 500;
  transition: color var(--transition-fast);
}

.nav-link:hover {
  color: var(--color-text-primary);
}

.nav-actions {
  display: flex;
  align-items: center;
  gap: 1rem;
}

.lang-switcher {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 0.75rem;
  background: transparent;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.lang-switcher:hover {
  color: var(--color-text-primary);
  border-color: var(--color-border-hover);
  background: var(--color-bg-tertiary);
}

.github-btn {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem 1rem;
  background: var(--color-primary);
  border-radius: var(--radius-md);
  color: white;
  font-size: 0.875rem;
  font-weight: 500;
  text-decoration: none;
  transition: all var(--transition-fast);
}

.github-btn:hover {
  background: var(--color-primary-dark);
  color: white;
}

.mobile-menu-btn {
  display: none;
  background: transparent;
  border: none;
  color: var(--color-text-primary);
  cursor: pointer;
  padding: 0.5rem;
}

.mobile-menu {
  display: none;
  flex-direction: column;
  padding: 1rem 0 1.5rem;
  gap: 0.5rem;
}

.mobile-link {
  display: block;
  padding: 0.75rem 0;
  color: var(--color-text-secondary);
  text-decoration: none;
  font-size: 1rem;
  border-bottom: 1px solid var(--color-border);
}

.mobile-link:hover {
  color: var(--color-text-primary);
}

@media (max-width: 768px) {
  .nav-links {
    display: none;
  }

  .github-btn span {
    display: none;
  }

  .mobile-menu-btn {
    display: block;
  }

  .mobile-menu {
    display: flex;
  }
}
</style>
