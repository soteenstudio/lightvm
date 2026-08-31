<script setup>
// Nggak butuh logic rumit, cukup markup HTML & CSS styling
</script>

<template>
  <div class="partner-footer-container">
    <div class="partner-footer-content">
      <p class="collab-text">Official Documentation Partner</p>
      <div class="logo-wrapper">
        <!-- Logo Library (NoodleCSS) -->
        <a href="https://github.com/fictional/noodlecss" target="_blank" rel="noopener" class="brand-link">
          <span class="brand-name">🍜 NoodleCSS</span>
        </a>

        <!-- Logo Toko Partner (Kopi Koding) -->
        <a href="https://kopikoding.fictional" target="_blank" rel="noopener" class="brand-link">
          <span class="brand-name">☕ Kopi Koding</span>
        </a>
      </div>
    </div>
  </div>
</template>

<style scoped>
.partner-footer-container {
  background-color: var(--vp-c-bg);
  transition: background-color 0.5s, border-color 0.5s;
}

.partner-footer-content {
  max-width: var(--vp-layout-max-width, 1152px);
  margin: 0 auto;
  padding: 32px 24px;
  text-align: center;
}

@media (min-width: 640px) {
  .partner-footer-content {
    padding: 32px 32px;
  }
}

@media (min-width: 960px) {
  .partner-footer-content {
    padding: 32px 64px;
  }
}

.collab-text {
  font-size: 0.75rem;
  font-weight: 600;
  color: var(--vp-c-text-3);
  margin-bottom: 12px;
  text-transform: uppercase;
  letter-spacing: 1.5px;
}

.logo-wrapper {
  display: flex;
  justify-content: center;
  align-items: center;
  gap: 24px;
}

.brand-link {
  text-decoration: none;
  transition: opacity 0.25s;
}

.brand-link:hover {
  opacity: 0.7;
}

.brand-name {
  font-weight: 600;
  font-size: 0.95rem;
  color: var(--vp-c-text-1);
}
</style>
