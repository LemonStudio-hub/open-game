<template>
  <section id="code" class="section">
    <div class="container">
      <div class="section-header">
        <h2 class="gradient-text">{{ t.code.title }}</h2>
        <p>{{ t.code.subtitle }}</p>
      </div>

      <div class="code-tabs">
        <button
          v-for="(example, index) in t.code.examples"
          :key="index"
          class="tab-btn"
          :class="{ active: activeTab === index }"
          @click="activeTab = index"
        >
          {{ example.title }}
        </button>
      </div>

      <div class="code-content">
        <div
          v-for="(example, index) in t.code.examples"
          :key="index"
          v-show="activeTab === index"
          class="code-block"
        >
          <div class="code-header">
            <div class="code-dots">
              <span class="dot red"></span>
              <span class="dot yellow"></span>
              <span class="dot green"></span>
            </div>
            <span class="code-lang">{{ example.language }}</span>
          </div>
          <pre><code>{{ example.code }}</code></pre>
        </div>
      </div>

      <p class="code-description">{{ t.code.description }}</p>
    </div>
  </section>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue';
import { useTranslations, type Locale } from '../i18n';

const props = defineProps<{
  locale: Locale;
}>();

const t = computed(() => useTranslations(props.locale));

const activeTab = ref(0);
</script>

<style scoped>
.section-header {
  text-align: center;
  margin-bottom: 3rem;
}

.section-header h2 {
  font-size: 2.5rem;
  margin-bottom: 1rem;
}

.section-header p {
  font-size: 1.125rem;
  color: var(--color-text-secondary);
}

.code-tabs {
  display: flex;
  justify-content: center;
  gap: 1rem;
  margin-bottom: 2rem;
}

.tab-btn {
  padding: 0.75rem 1.5rem;
  background: var(--color-bg-card);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
  color: var(--color-text-secondary);
  font-size: 0.875rem;
  font-weight: 500;
  cursor: pointer;
  transition: all var(--transition-fast);
}

.tab-btn:hover {
  color: var(--color-text-primary);
  border-color: var(--color-border-hover);
}

.tab-btn.active {
  background: var(--color-primary);
  border-color: var(--color-primary);
  color: white;
}

.code-content {
  max-width: 800px;
  margin: 0 auto 2rem;
}

.code-block {
  background: var(--color-bg-secondary);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-xl);
  overflow: hidden;
}

.code-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0.75rem 1rem;
  background: var(--color-bg-tertiary);
  border-bottom: 1px solid var(--color-border);
}

.code-dots {
  display: flex;
  gap: 0.5rem;
}

.dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
}

.dot.red { background: #ef4444; }
.dot.yellow { background: #eab308; }
.dot.green { background: #22c55e; }

.code-lang {
  color: var(--color-text-muted);
  font-size: 0.75rem;
  font-family: var(--font-mono);
  text-transform: uppercase;
}

pre {
  padding: 1.5rem;
  margin: 0;
  overflow-x: auto;
  background: transparent;
  border: none;
}

code {
  font-family: var(--font-mono);
  font-size: 0.875rem;
  line-height: 1.75;
  color: var(--color-text-primary);
  background: none;
  padding: 0;
  border: none;
}

.code-description {
  text-align: center;
  font-size: 1.125rem;
  color: var(--color-text-secondary);
  max-width: 600px;
  margin: 0 auto;
}

@media (max-width: 640px) {
  .section-header h2 {
    font-size: 2rem;
  }

  .code-tabs {
    flex-direction: column;
    align-items: center;
  }

  .tab-btn {
    width: 100%;
    max-width: 300px;
  }
}
</style>
