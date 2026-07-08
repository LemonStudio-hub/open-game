<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useI18n } from 'vue-i18n'

const { t, locale } = useI18n()

const isScrolled = ref(false)
const isMobileMenuOpen = ref(false)
const activeSection = ref('hero')

const navItemKeys = ['about', 'history', 'resolution', 'situation', 'structure', 'operators', 'operations', 'relations', 'wanted'] as const

const handleScroll = () => {
  isScrolled.value = window.scrollY > 20

  const sections = ['hero', ...navItemKeys]
  const scrollPos = window.scrollY + 100

  for (let i = sections.length - 1; i >= 0; i--) {
    const section = document.getElementById(sections[i])
    if (section && section.offsetTop <= scrollPos) {
      activeSection.value = sections[i]
      break
    }
  }
}

const scrollTo = (id: string) => {
  const el = document.getElementById(id)
  if (el) {
    el.scrollIntoView({ behavior: 'smooth' })
    isMobileMenuOpen.value = false
  }
}

const toggleLocale = () => {
  const next = locale.value === 'zh' ? 'en' : 'zh'
  locale.value = next
  localStorage.setItem('locale', next)
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll, { passive: true })
  handleScroll()
})

onUnmounted(() => {
  window.removeEventListener('scroll', handleScroll)
})
</script>

<template>
  <nav class="navbar" :class="{ scrolled: isScrolled }">
    <div class="navbar-inner">
      <div class="navbar-brand" @click="scrollTo('hero')">
        <div class="brand-icon">
          <svg viewBox="0 0 32 32" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M16 2L4 10v12l12 8 12-8V10L16 2z" stroke="currentColor" stroke-width="1.5" fill="none"/>
            <path d="M16 8l-6 4v8l6 4 6-4v-8l-6-4z" fill="currentColor" opacity="0.3"/>
            <path d="M16 12v8M12 14l4 2 4-2" stroke="currentColor" stroke-width="1.5"/>
          </svg>
        </div>
        <span class="brand-text">
          <span class="brand-gti">G.T.I.</span>
          <span class="brand-sub">{{ t('nav.brand') }}</span>
        </span>
      </div>

      <div class="navbar-links" :class="{ open: isMobileMenuOpen }">
        <a
          v-for="key in navItemKeys"
          :key="key"
          class="nav-link"
          :class="{ active: activeSection === key }"
          @click.prevent="scrollTo(key)"
        >
          {{ t(`nav.items.${key}`) }}
        </a>
      </div>

      <div class="navbar-actions">
        <button class="lang-switch" @click="toggleLocale" :aria-label="t('nav.langSwitch')">
          {{ t('nav.langSwitch') }}
        </button>

        <button
          class="mobile-toggle"
          :class="{ active: isMobileMenuOpen }"
          @click="isMobileMenuOpen = !isMobileMenuOpen"
          aria-label="Toggle menu"
        >
          <span></span>
          <span></span>
          <span></span>
        </button>
      </div>
    </div>
  </nav>
</template>

<style scoped>
.navbar {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  z-index: 1000;
  height: var(--nav-height);
  display: flex;
  align-items: center;
  transition: background-color var(--transition-base), backdrop-filter var(--transition-base);
}

.navbar.scrolled {
  background-color: rgba(10, 12, 16, 0.85);
  -webkit-backdrop-filter: blur(20px);
  backdrop-filter: blur(20px);
  border-bottom: 1px solid var(--color-border);
}

.navbar-inner {
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 0 var(--space-xl);
  width: 100%;
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.navbar-brand {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  cursor: pointer;
  transition: opacity var(--transition-fast);
}

.navbar-brand:hover {
  opacity: 0.8;
}

.brand-icon {
  width: 32px;
  height: 32px;
  color: var(--color-accent);
}

.brand-text {
  display: flex;
  flex-direction: column;
  line-height: 1;
}

.brand-gti {
  font-family: var(--font-mono);
  font-size: 1rem;
  font-weight: 700;
  letter-spacing: 0.15em;
  color: var(--color-accent);
}

.brand-sub {
  font-size: 0.6rem;
  color: var(--color-text-muted);
  letter-spacing: 0.05em;
  margin-top: 2px;
}

.navbar-links {
  display: flex;
  align-items: center;
  gap: var(--space-lg);
}

.nav-link {
  font-size: 0.85rem;
  font-weight: 500;
  color: var(--color-text-secondary);
  cursor: pointer;
  position: relative;
  padding: var(--space-xs) 0;
  transition: color var(--transition-fast);
}

.nav-link::after {
  content: '';
  position: absolute;
  bottom: -2px;
  left: 0;
  width: 0;
  height: 1px;
  background: var(--color-accent);
  transition: width var(--transition-base);
}

.nav-link:hover {
  color: var(--color-text-primary);
}

.nav-link:hover::after {
  width: 100%;
}

.nav-link.active {
  color: var(--color-accent);
}

.nav-link.active::after {
  width: 100%;
  background: var(--color-accent);
}

.navbar-actions {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
}

.lang-switch {
  font-family: var(--font-mono);
  font-size: 0.75rem;
  font-weight: 600;
  letter-spacing: 0.05em;
  color: var(--color-accent);
  background: transparent;
  border: 1px solid var(--color-border);
  border-radius: var(--radius-sm);
  padding: 4px 10px;
  cursor: pointer;
  transition: all var(--transition-fast);
  min-width: 36px;
  text-align: center;
}

.lang-switch:hover {
  border-color: var(--color-accent);
  background: rgba(74, 232, 122, 0.08);
}

.mobile-toggle {
  display: none;
  flex-direction: column;
  gap: 5px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 12px;
  min-width: 44px;
  min-height: 44px;
  align-items: center;
  justify-content: center;
}

.mobile-toggle span {
  width: 20px;
  height: 1.5px;
  background: var(--color-text-primary);
  transition: transform var(--transition-base), opacity var(--transition-base);
}

.mobile-toggle.active span:nth-child(1) {
  transform: rotate(45deg) translate(4px, 5px);
}

.mobile-toggle.active span:nth-child(2) {
  opacity: 0;
}

.mobile-toggle.active span:nth-child(3) {
  transform: rotate(-45deg) translate(4px, -5px);
}

@media (max-width: 768px) {
  .mobile-toggle {
    display: flex;
  }

  .navbar-links {
    position: fixed;
    top: var(--nav-height);
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(10, 12, 16, 0.95);
    -webkit-backdrop-filter: blur(20px);
    backdrop-filter: blur(20px);
    flex-direction: column;
    align-items: center;
    justify-content: center;
    padding: var(--space-xl);
    gap: var(--space-md);
    border-bottom: 1px solid var(--color-border);
    transform: translateY(-100%);
    opacity: 0;
    pointer-events: none;
    transition: transform var(--transition-base), opacity var(--transition-base);
    overflow-y: auto;
  }

  .navbar-links.open {
    transform: translateY(0);
    opacity: 1;
    pointer-events: all;
  }

  .nav-link {
    font-size: 1.1rem;
    padding: var(--space-sm) 0;
  }
}

@media (min-width: 769px) and (max-width: 1024px) {
  .navbar-links {
    gap: var(--space-md);
  }

  .nav-link {
    font-size: 0.8rem;
  }

  .brand-sub {
    display: none;
  }
}
</style>
