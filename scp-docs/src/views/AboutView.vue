<template>
  <div class="about">
    <div class="page-header">
      <h1>About the Foundation</h1>
      <p class="page-desc">Understanding the SCP Foundation and the Latom Node documentation system.</p>
    </div>

    <div class="about-content">
      <section class="section">
        <div class="section-icon">◈</div>
        <div class="section-body">
          <h2>The SCP Foundation</h2>
          <p>
            The SCP Foundation is a clandestine organization operating under the authority of the United Nations
            and various world governments. Its mission is to secure, contain, and protect anomalous objects,
            entities, and phenomena that pose a threat to normalcy and human civilization.
          </p>
          <p>
            Founded in the early 19th century, the Foundation has grown into a global network of secure
            facilities, research laboratories, and mobile task forces dedicated to understanding and
            neutralizing anomalous threats.
          </p>
        </div>
      </section>

      <section class="section">
        <div class="section-icon">◫</div>
        <div class="section-body">
          <h2>The Latom Node</h2>
          <p>
            The Latom Node is a documentation terminal within the Foundation's information network. It serves
            as an archival and retrieval system for SCP entries, containment procedures, research documents,
            and operational protocols.
          </p>
          <p>
            Designated as Node LATOM-7, this terminal operates under Level 4 clearance and maintains
            synchronization with the central Foundation database. All information presented is current
            as of the last synchronization cycle.
          </p>
        </div>
      </section>

      <section class="section">
        <div class="section-icon">◎</div>
        <div class="section-body">
          <h2>Object Classification System</h2>
          <div class="class-grid">
            <div class="class-card safe">
              <h3>Safe</h3>
              <p>Objects that are easily and safely contained. Minimal risk when proper procedures are followed.</p>
            </div>
            <div class="class-card euclid">
              <h3>Euclid</h3>
              <p>Objects requiring more extensive containment procedures. Behavior is not fully understood or predictable.</p>
            </div>
            <div class="class-card keter">
              <h3>Keter</h3>
              <p>Objects that are exceedingly difficult to contain consistently or pose significant threat if breached.</p>
            </div>
            <div class="class-card thaumiel">
              <h3>Thaumiel</h3>
              <p>Objects used by the Foundation to contain or counteract other anomalous entities or phenomena.</p>
            </div>
            <div class="class-card apollyon">
              <h3>Apollyon</h3>
              <p>Objects that cannot be contained, are expected to breach imminently, and pose a catastrophic threat.</p>
            </div>
            <div class="class-card neutralized">
              <h3>Neutralized</h3>
              <p>Objects that are no longer anomalous, have been destroyed, or have lost their anomalous properties.</p>
            </div>
          </div>
        </div>
      </section>

      <section class="section">
        <div class="section-icon">▣</div>
        <div class="section-body">
          <h2>System Information</h2>
          <div class="sys-info">
            <div class="sys-row">
              <span class="sys-label">Terminal</span>
              <span class="sys-value">LATOM-7</span>
            </div>
            <div class="sys-row">
              <span class="sys-label">Version</span>
              <span class="sys-value">7.2.1</span>
            </div>
            <div class="sys-row">
              <span class="sys-label">Clearance</span>
              <span class="sys-value accent">Level 4</span>
            </div>
            <div class="sys-row">
              <span class="sys-label">Status</span>
              <span class="sys-value success">Operational</span>
            </div>
            <div class="sys-row">
              <span class="sys-label">Last Sync</span>
              <span class="sys-value">{{ new Date().toISOString().split('T')[0] }}</span>
            </div>
          </div>
        </div>
      </section>
    </div>
  </div>
</template>

<style scoped>
.about {
  max-width: var(--max-content);
  margin: 0 auto;
}

.page-header {
  margin-bottom: var(--space-2xl);
}

.page-header h1 {
  margin-bottom: var(--space-sm);
}

.page-desc {
  color: var(--text-secondary);
  font-size: var(--text-lg);
}

.about-content {
  display: flex;
  flex-direction: column;
  gap: var(--space-2xl);
}

.section {
  display: flex;
  gap: var(--space-lg);
}

.section-icon {
  font-size: var(--text-2xl);
  color: var(--color-primary);
  flex-shrink: 0;
  margin-top: 4px;
}

.section-body {
  flex: 1;
}

.section-body h2 {
  font-size: var(--text-xl);
  margin-bottom: var(--space-md);
}

.section-body p {
  color: var(--text-secondary);
  line-height: var(--leading-relaxed);
  margin-bottom: var(--space-md);
}

.class-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(240px, 1fr));
  gap: var(--space-md);
  margin-top: var(--space-md);
}

.class-card {
  padding: var(--space-md);
  border-radius: var(--radius-md);
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
}

.class-card h3 {
  font-size: var(--text-base);
  margin-bottom: var(--space-xs);
}

.class-card p {
  font-size: var(--text-sm);
  color: var(--text-tertiary);
  margin-bottom: 0;
}

.class-card.safe h3 { color: var(--class-safe); }
.class-card.euclid h3 { color: var(--class-euclid); }
.class-card.keter h3 { color: var(--class-keter); }
.class-card.thaumiel h3 { color: var(--class-thaumiel); }
.class-card.apollyon h3 { color: var(--text-primary); }
.class-card.neutralized h3 { color: var(--class-neutralized); }

.sys-info {
  background: var(--bg-surface);
  border: 1px solid var(--border-subtle);
  border-radius: var(--radius-md);
  overflow: hidden;
}

.sys-row {
  display: flex;
  justify-content: space-between;
  padding: var(--space-sm) var(--space-md);
  font-family: var(--font-mono);
  font-size: var(--text-sm);
}

.sys-row:not(:last-child) {
  border-bottom: 1px solid var(--border-subtle);
}

.sys-label {
  color: var(--text-tertiary);
}

.sys-value {
  color: var(--text-secondary);
}

.sys-value.accent {
  color: var(--color-primary);
}

.sys-value.success {
  color: var(--color-success);
}

@media (max-width: 640px) {
  .section {
    flex-direction: column;
    gap: var(--space-sm);
  }
}
</style>
