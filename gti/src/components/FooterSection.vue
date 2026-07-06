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
          <p class="footer-tagline">全球反恐特勤组</p>
          <p class="footer-motto">"不让任何一个人掉队"</p>
        </div>

        <div class="footer-links">
          <div class="footer-column">
            <h4 class="footer-col-title">导航</h4>
            <a href="#about">关于 GTI</a>
            <a href="#history">历史沿革</a>
            <a href="#structure">组织架构</a>
          </div>
          <div class="footer-column">
            <h4 class="footer-col-title">更多</h4>
            <a href="#operators">精英干员</a>
            <a href="#operations">主要行动</a>
            <a href="#relations">势力关系</a>
          </div>
        </div>
      </div>

      <div class="footer-bottom">
        <p class="footer-copyright">
          本档案为粉丝站整理编制，仅供爱好者参考使用
        </p>
        <p class="footer-disclaimer">
          G.T.I. 及《三角洲行动》相关设定版权归属游戏官方
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
}
</style>
