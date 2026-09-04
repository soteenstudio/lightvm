<template>
  <div class="partner-footer-container">
    <div class="partner-footer-content">
      <p class="collab-text">{{ collabText }}</p>

      <div class="logo-wrapper">
        <a
          v-for="(partner, index) in partnerData"
          :key="partner.name"
          :href="partner.website"
          :aria-describedby="`partner-tooltip-${index}`"
          target="_blank"
          rel="noopener noreferrer"
          class="brand-link"
        >
          <span class="brand-name">
            <span
              :id="`partner-tooltip-${index}`"
              role="tooltip"
              class="brand-desc"
            >
              {{ getLocaleDescription(partner, currentLocale) }}
            </span>
        
            <img
              class="brand-icon"
              :src="partner.logo"
              :alt="partner.name"
            >
            {{ partner.name }}
          </span>
        </a>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed, ref } from 'vue';
import { useData } from 'vitepress';
import partnerData from '../../../data/partner.json';

const { lang } = useData();
const activePartner = ref(null);

const translations = {
  en: { collabText: 'Official Documentation Partner' },
  id: { collabText: 'Mitra Dokumentasi Resmi' },
};

const currentLocale = computed(() => {
  const shortLang = lang.value?.split('-')[0] ?? 'en';
  return translations[shortLang] ? shortLang : 'en';
});

const collabText = computed(
  () => translations[currentLocale.value].collabText,
);

function getLocaleDescription(partner, locale) {
  return partner.description?.[locale] ?? partner.description?.en ?? '';
}
</script>

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
  position: relative;
  text-decoration: none;
}

.brand-name {
  position: relative;
  display: inline-flex;
  align-items: center;
  font-size: 0.95rem;
  font-weight: 600;
  color: var(--vp-c-text-1);
}

.brand-desc {
  position: absolute;
  z-index: 10;
  bottom: calc(100% + 12px);
  left: 50%;
  width: max-content;
  max-width: min(20rem, calc(100vw - 32px));
  padding: 10px 12px;
  border: 1px solid var(--vp-c-divider);
  border-radius: 8px;
  background: var(--vp-c-bg-soft);
  box-shadow: 0 8px 24px rgb(0 0 0 / 15%);
  color: var(--vp-c-text-2);
  font-size: 0.75rem;
  font-weight: 400;
  line-height: 1.5;
  text-align: center;
  opacity: 0;
  pointer-events: none;
  transform: translate(-50%, 4px);
  transition: opacity 0.2s ease, transform 0.2s ease;
}

.brand-desc::after {
  position: absolute;
  bottom: -6px;
  left: 50%;
  width: 10px;
  height: 10px;
  border-right: 1px solid var(--vp-c-divider);
  border-bottom: 1px solid var(--vp-c-divider);
  background: var(--vp-c-bg-soft);
  content: '';
  transform: translateX(-50%) rotate(45deg);
}

.brand-link:hover .brand-desc,
.brand-link:focus-visible .brand-desc {
  opacity: 1;
  transform: translate(-50%, 0);
}

.brand-link:focus-visible {
  outline: 2px solid var(--vp-c-brand-1);
  outline-offset: 4px;
  border-radius: 4px;
}

@media (prefers-reduced-motion: reduce) {
  .brand-desc {
    transition: none;
  }
}

.brand-icon {
  width: 24px;
  height: 24px;
  margin-right: 8px;
  vertical-align: middle;
}
</style>
