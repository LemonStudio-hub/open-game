<script setup lang="ts">
import { ref } from 'vue'
import { useI18n } from 'vue-i18n'
import { useScrollReveal } from '../composables/useScrollReveal'

const { t, tm } = useI18n()
const sectionRef = ref<HTMLElement | null>(null)
useScrollReveal(sectionRef)
</script>

<template>
  <section id="resolution" ref="sectionRef" class="section resolution">
    <div class="container">
      <div class="section-header reveal">
        <span class="section-label">{{ t('resolution.label') }}</span>
        <h2 class="section-title">{{ t('resolution.title') }}</h2>
        <div class="divider"></div>
      </div>

      <div class="resolution-doc reveal-scale">
        <div class="doc-watermark">UNITED NATIONS</div>

        <div class="doc-header">
          <div class="un-emblem">
            <svg viewBox="0 0 64 64" fill="none" xmlns="http://www.w3.org/2000/svg">
              <circle cx="32" cy="32" r="28" stroke="currentColor" stroke-width="1"/>
              <circle cx="32" cy="32" r="20" stroke="currentColor" stroke-width="0.5" opacity="0.5"/>
              <path d="M32 4v56M4 32h56" stroke="currentColor" stroke-width="0.5" opacity="0.3"/>
              <ellipse cx="32" cy="32" rx="14" ry="28" stroke="currentColor" stroke-width="0.5" opacity="0.4"/>
              <circle cx="32" cy="32" r="3" fill="currentColor" opacity="0.6"/>
              <path d="M22 18l5 4-2 5 5-3 4 5-1-6 5-2-6-1 2-5-4 4-5-4 3 5z" fill="currentColor" opacity="0.3"/>
            </svg>
          </div>
          <div class="doc-org">
            <span class="org-name">{{ t('resolution.orgName') }}</span>
            <span class="org-name-en">{{ t('resolution.orgNameEn') }}</span>
          </div>
        </div>

        <div class="doc-meta">
          <div class="meta-row">
            <span class="meta-label">{{ t('resolution.meta.council') }}</span>
            <span class="meta-value">{{ t('resolution.meta.councilEn') }}</span>
          </div>
          <div class="meta-row">
            <span class="meta-label">{{ t('resolution.meta.number') }}</span>
            <span class="meta-value highlight">S/RES/2984 (2032)</span>
          </div>
          <div class="meta-row">
            <span class="meta-label">{{ t('resolution.meta.date') }}</span>
            <span class="meta-value">{{ t('resolution.meta.dateValue') }}</span>
          </div>
        </div>

        <div class="doc-title-block">
          <h3 class="doc-title" v-html="t('resolution.docTitle').replace(/\n/g, '<br />')"></h3>
        </div>

        <div class="doc-body">
          <div class="preamble">
            <p class="pre-para">
              <span class="pre-verb">{{ t('resolution.preamble.recalls') }}</span>{{ t('resolution.preamble.recallsText') }}
            </p>
            <p class="pre-para">
              <span class="pre-verb">{{ t('resolution.preamble.reaffirms') }}</span>{{ t('resolution.preamble.reaffirmsText') }}
            </p>
            <p class="pre-para">
              <span class="pre-verb">{{ t('resolution.preamble.concerned') }}</span>{{ t('resolution.preamble.concernedText') }}
            </p>
            <p class="pre-para">
              <span class="pre-verb">{{ t('resolution.preamble.noting') }}</span>{{ t('resolution.preamble.notingText') }}
            </p>
            <p class="pre-para">
              <span class="pre-verb">{{ t('resolution.preamble.condemning') }}</span>{{ t('resolution.preamble.condemningText') }}
            </p>
            <p class="pre-para">
              <span class="pre-verb">{{ t('resolution.preamble.emphasizing') }}</span>{{ t('resolution.preamble.emphasizingText') }}
            </p>
            <p class="pre-para">
              <span class="pre-verb">{{ t('resolution.preamble.determining') }}</span>{{ t('resolution.preamble.determiningText') }}
            </p>
            <p class="pre-para last">
              <span class="pre-verb">{{ t('resolution.preamble.acting') }}</span>{{ t('resolution.preamble.actingText') }}
            </p>
          </div>

          <div class="operative">
            <p class="op-heading">{{ t('resolution.operative.heading') }}</p>

            <p v-for="(item, idx) in (tm('resolution.operative.items') as any[])" :key="idx" class="op-para" :class="{ last: idx === 7 }">
              <span class="op-num">{{ (tm('resolution.operative.nums') as string[])[idx] }}</span>
              <span class="op-text">
                <span class="op-verb">{{ item.verb }}</span>
                <span>{{ item.text }}</span>
              </span>
            </p>
          </div>
        </div>

        <div class="doc-footer">
          <div class="vote-record">
            <span class="vote-label">{{ t('resolution.vote.label') }}</span>
            <div class="vote-result">
              <span class="vote-yes">{{ t('resolution.vote.yes') }}</span>
              <span class="vote-no">{{ t('resolution.vote.no') }}</span>
              <span class="vote-abstain">{{ t('resolution.vote.abstain') }}</span>
            </div>
            <span class="vote-note">{{ t('resolution.vote.note') }}</span>
          </div>

          <div class="doc-stamp">
            <div class="stamp-border">
              <span class="stamp-text">SECURITY COUNCIL</span>
              <span class="stamp-text">UNITED NATIONS</span>
              <span class="stamp-year">2032</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  </section>
</template>

<style scoped>
.resolution {
  background: var(--color-bg-secondary);
}

.resolution-doc {
  max-width: 800px;
  margin: 0 auto;
  background: linear-gradient(135deg, rgba(21, 24, 32, 0.95), rgba(17, 19, 24, 0.98));
  border: 1px solid var(--color-border);
  border-radius: var(--radius-lg);
  padding: var(--space-3xl) var(--space-2xl);
  position: relative;
  overflow: hidden;
}

.doc-watermark {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%) rotate(-30deg);
  font-family: var(--font-mono);
  font-size: 5rem;
  font-weight: 800;
  color: rgba(74, 232, 122, 0.03);
  letter-spacing: 0.3em;
  white-space: nowrap;
  pointer-events: none;
  user-select: none;
}

.doc-header {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: var(--space-md);
  margin-bottom: var(--space-2xl);
  padding-bottom: var(--space-lg);
  border-bottom: 1px solid var(--color-border);
}

.un-emblem {
  width: 48px;
  height: 48px;
  color: var(--color-accent);
  opacity: 0.7;
}

.un-emblem svg {
  width: 100%;
  height: 100%;
}

.doc-org {
  display: flex;
  flex-direction: column;
  text-align: center;
}

.org-name {
  font-size: 1.4rem;
  font-weight: 700;
  letter-spacing: 0.3em;
  color: var(--color-text-primary);
}

.org-name-en {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  letter-spacing: 0.25em;
  color: var(--color-text-muted);
  margin-top: 2px;
}

.doc-meta {
  display: flex;
  flex-direction: column;
  gap: var(--space-xs);
  margin-bottom: var(--space-2xl);
  padding: var(--space-lg);
  background: rgba(74, 232, 122, 0.03);
  border: 1px solid var(--color-border);
  border-radius: var(--radius-md);
}

.meta-row {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  gap: var(--space-md);
}

.meta-label {
  font-size: 0.8rem;
  color: var(--color-text-muted);
  flex-shrink: 0;
}

.meta-value {
  font-family: var(--font-mono);
  font-size: 0.85rem;
  color: var(--color-text-secondary);
  text-align: right;
}

.meta-value.highlight {
  color: var(--color-accent);
  font-weight: 600;
}

.doc-title-block {
  text-align: center;
  margin-bottom: var(--space-2xl);
  padding-bottom: var(--space-xl);
  border-bottom: 2px solid var(--color-border);
}

.doc-title {
  font-size: 1.15rem;
  font-weight: 600;
  line-height: 1.8;
  color: var(--color-text-primary);
  letter-spacing: 0.05em;
}

.doc-body {
  position: relative;
  z-index: 1;
}

.preamble {
  margin-bottom: var(--space-2xl);
  padding-left: var(--space-lg);
  border-left: 2px solid rgba(74, 232, 122, 0.15);
}

.pre-para {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  line-height: 1.9;
  margin-bottom: var(--space-sm);
  padding-left: var(--space-md);
}

.pre-para.last {
  margin-bottom: 0;
}

.pre-verb {
  font-weight: 600;
  color: var(--color-text-primary);
}

.operative {
  padding-left: 0;
}

.op-heading {
  font-size: 0.9rem;
  font-weight: 600;
  color: var(--color-text-primary);
  margin-bottom: var(--space-lg);
  text-align: center;
  letter-spacing: 0.1em;
}

.op-para {
  font-size: 0.9rem;
  color: var(--color-text-secondary);
  line-height: 1.9;
  margin-bottom: var(--space-md);
  display: flex;
  gap: var(--space-sm);
  padding-left: var(--space-md);
}

.op-para.last {
  margin-bottom: 0;
}

.op-num {
  font-family: var(--font-mono);
  font-size: 0.85rem;
  color: var(--color-accent);
  flex-shrink: 0;
  min-width: 2em;
  font-weight: 600;
}

.op-text {
  flex: 1;
}

.op-verb {
  font-weight: 600;
  color: var(--color-text-primary);
}

.doc-footer {
  margin-top: var(--space-3xl);
  padding-top: var(--space-xl);
  border-top: 1px solid var(--color-border);
  display: flex;
  align-items: flex-end;
  justify-content: space-between;
  gap: var(--space-xl);
}

.vote-record {
  display: flex;
  flex-direction: column;
  gap: var(--space-sm);
}

.vote-label {
  font-family: var(--font-mono);
  font-size: 0.65rem;
  letter-spacing: 0.15em;
  color: var(--color-text-muted);
  text-transform: uppercase;
}

.vote-result {
  display: flex;
  gap: var(--space-md);
}

.vote-yes {
  font-family: var(--font-mono);
  font-size: 0.8rem;
  color: #4ae87a;
}

.vote-no {
  font-family: var(--font-mono);
  font-size: 0.8rem;
  color: #e8534a;
}

.vote-abstain {
  font-family: var(--font-mono);
  font-size: 0.8rem;
  color: var(--color-text-muted);
}

.vote-note {
  font-size: 0.75rem;
  color: var(--color-text-muted);
  font-style: italic;
}

.doc-stamp {
  flex-shrink: 0;
}

.stamp-border {
  width: 96px;
  height: 96px;
  border: 2px solid rgba(74, 232, 122, 0.2);
  border-radius: 50%;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 2px;
  transform: rotate(-12deg);
}

.stamp-text {
  font-family: var(--font-mono);
  font-size: 0.45rem;
  letter-spacing: 0.1em;
  color: var(--color-accent);
  opacity: 0.5;
}

.stamp-year {
  font-family: var(--font-mono);
  font-size: 0.9rem;
  font-weight: 700;
  color: var(--color-accent);
  opacity: 0.4;
}

@media (max-width: 768px) {
  .resolution-doc {
    padding: var(--space-2xl) var(--space-lg);
  }

  .doc-watermark {
    font-size: 3rem;
  }

  .doc-title {
    font-size: 1rem;
  }

  .doc-footer {
    flex-direction: column;
    align-items: flex-start;
  }

  .doc-stamp {
    align-self: flex-end;
  }

  .meta-row {
    flex-direction: column;
    gap: 2px;
  }

  .meta-value {
    text-align: left;
  }
}

@media (max-width: 480px) {
  .resolution-doc {
    padding: var(--space-lg) var(--space-md);
  }

  .doc-header {
    gap: var(--space-sm);
  }

  .un-emblem {
    width: 36px;
    height: 36px;
  }

  .org-name {
    font-size: 1.1rem;
    letter-spacing: 0.2em;
  }

  .preamble {
    padding-left: var(--space-md);
  }

  .pre-para,
  .op-para {
    font-size: 0.85rem;
  }

  .vote-result {
    flex-wrap: wrap;
    gap: var(--space-sm);
  }

  .stamp-border {
    width: 72px;
    height: 72px;
  }
}
</style>
