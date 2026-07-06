<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'

const isScrolled = ref(false)
const isMobileMenuOpen = ref(false)

const navItems = [
  { id: 'about', label: '关于' },
  { id: 'history', label: '历史' },
  { id: 'structure', label: '架构' },
  { id: 'operators', label: '干员' },
  { id: 'operations', label: '行动' },
  { id: 'relations', label: '势力' },
]

const handleScroll = () => {
  isScrolled.value = window.scrollY > 20
}

const scrollTo = (id: string) => {
  const el = document.getElementById(id)
  if (el) {
    el.scrollIntoView({ behavior: 'smooth' })
    isMobileMenuOpen.value = false
  }
}

onMounted(() => {
  window.addEventListener('scroll', handleScroll, { passive: true })
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
          <span class="brand-sub">全球反恐特勤组</span>
        </span>
      </div>

      <div class="navbar-links" :class="{ open: isMobileMenuOpen }">
        <a
          v-for="item in navItems"
          :key="item.id"
          class="nav-link"
          @click.prevent="scrollTo(item.id)"
        >
          {{ item.label }}
        </a>
      </div>

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

.mobile-toggle {
  display: none;
  flex-direction: column;
  gap: 5px;
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
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
    background: rgba(10, 12, 16, 0.95);
    backdrop-filter: blur(20px);
    flex-direction: column;
    padding: var(--space-xl);
    gap: var(--space-md);
    border-bottom: 1px solid var(--color-border);
    transform: translateY(-100%);
    opacity: 0;
    pointer-events: none;
    transition: transform var(--transition-base), opacity var(--transition-base);
  }

  .navbar-links.open {
    transform: translateY(0);
    opacity: 1;
    pointer-events: all;
  }
}
</style>
