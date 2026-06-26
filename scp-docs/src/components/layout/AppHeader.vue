<script setup lang="ts">
import { useTheme } from '@/composables/useTheme'
import { useSearchStore } from '@/stores/search'
import { useRoute } from 'vue-router'
import { computed } from 'vue'

const { theme, toggle: toggleTheme } = useTheme()
const search = useSearchStore()
const route = useRoute()

const breadcrumbs = computed(() => {
  const name = route.meta.title as string
  return name || 'Home'
})
</script>

<template>
  <header class="header">
    <div class="header-left">
      <router-link to="/" class="logo-link">
        <div class="logo-icon">
          <svg viewBox="0 0 32 32" fill="none">
            <circle cx="16" cy="16" r="14" stroke="currentColor" stroke-width="2" />
            <circle cx="16" cy="16" r="4" fill="currentColor" />
            <line x1="16" y1="2" x2="16" y2="8" stroke="currentColor" stroke-width="2" />
            <line x1="16" y1="24" x2="16" y2="30" stroke="currentColor" stroke-width="2" />
            <line x1="2" y1="16" x2="8" y2="16" stroke="currentColor" stroke-width="2" />
            <line x1="24" y1="16" x2="30" y2="16" stroke="currentColor" stroke-width="2" />
          </svg>
        </div>
        <div class="logo-text">
          <span class="logo-title">SCP Foundation</span>
          <span class="logo-subtitle">Latom Node</span>
        </div>
      </router-link>
    </div>

    <div class="header-center">
      <span class="breadcrumb">{{ breadcrumbs }}</span>
    </div>

    <div class="header-right">
      <button class="search-btn" @click="search.open" title="Search (Ctrl+K)">
        <svg width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="11" cy="11" r="8" />
          <line x1="21" y1="21" x2="16.65" y2="16.65" />
        </svg>
        <span class="search-label">Search</span>
        <kbd>⌘K</kbd>
      </button>

      <button class="icon-btn" @click="toggleTheme" :title="theme === 'dark' ? 'Light mode' : 'Dark mode'">
        <svg v-if="theme === 'dark'" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <circle cx="12" cy="12" r="5" />
          <line x1="12" y1="1" x2="12" y2="3" /><line x1="12" y1="21" x2="12" y2="23" />
          <line x1="4.22" y1="4.22" x2="5.64" y2="5.64" /><line x1="18.36" y1="18.36" x2="19.78" y2="19.78" />
          <line x1="1" y1="12" x2="3" y2="12" /><line x1="21" y1="12" x2="23" y2="12" />
          <line x1="4.22" y1="19.78" x2="5.64" y2="18.36" /><line x1="18.36" y1="5.64" x2="19.78" y2="4.22" />
        </svg>
        <svg v-else width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2">
          <path d="M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" />
        </svg>
      </button>
    </div>
  </header>
</template>

<style scoped>
.header {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  height: var(--header-height);
  background: var(--bg-surface);
  border-bottom: 1px solid var(--border-subtle);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 var(--space-lg);
  z-index: var(--z-header);
  backdrop-filter: blur(12px);
  background: color-mix(in srgb, var(--bg-surface) 85%, transparent);
}

.header-left {
  flex: 1;
}

.logo-link {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  color: var(--text-primary);
  text-decoration: none;
}

.logo-link:hover {
  color: var(--text-primary);
}

.logo-icon {
  width: 32px;
  height: 32px;
  color: var(--color-primary);
}

.logo-text {
  display: flex;
  flex-direction: column;
  line-height: 1.1;
}

.logo-title {
  font-size: var(--text-sm);
  font-weight: 700;
  letter-spacing: 0.08em;
  text-transform: uppercase;
  color: var(--text-primary);
}

.logo-subtitle {
  font-size: var(--text-xs);
  color: var(--color-primary);
  font-weight: 500;
  letter-spacing: 0.12em;
  text-transform: uppercase;
}

.header-center {
  flex: 1;
  text-align: center;
}

.breadcrumb {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  font-weight: 500;
}

.header-right {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: var(--space-sm);
}

.search-btn {
  display: flex;
  align-items: center;
  gap: var(--space-sm);
  padding: 6px 12px;
  border-radius: var(--radius-md);
  background: var(--bg-elevated);
  border: 1px solid var(--border-subtle);
  color: var(--text-secondary);
  font-size: var(--text-sm);
  transition: all var(--transition-fast);
}

.search-btn:hover {
  border-color: var(--border-default);
  color: var(--text-primary);
}

.search-label {
  display: none;
}

@media (min-width: 640px) {
  .search-label {
    display: inline;
  }
}

kbd {
  font-family: var(--font-mono);
  font-size: var(--text-xs);
  padding: 2px 6px;
  border-radius: 4px;
  background: var(--bg-primary);
  border: 1px solid var(--border-subtle);
  color: var(--text-tertiary);
  display: none;
}

@media (min-width: 640px) {
  kbd {
    display: inline;
  }
}

.icon-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: var(--radius-md);
  color: var(--text-secondary);
  transition: all var(--transition-fast);
}

.icon-btn:hover {
  background: var(--bg-elevated);
  color: var(--text-primary);
}
</style>
