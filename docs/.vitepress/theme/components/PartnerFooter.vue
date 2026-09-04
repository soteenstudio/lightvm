<template>
  <div class="partner-footer-container">
    <div class="partner-footer-content">
      <p class="collab-text">{{ collabText }}</p>
      <div class="logo-wrapper">
        <!-- Logo Library (NoodleCSS) -->
        <a href="{{ partnerData[0].website }}" target="_blank" rel="noopener" class="brand-link">
          <span class="brand-name"><img class="brand-icon" :src="partnerData[0].logo" :alt="partnerData[0].name"> {{ partnerData[0].name }}</span>
        </a>

        <!-- Logo Toko Partner (Kopi Koding) -->
        <a href="{{ partnerData[1].website }}" target="_blank" rel="noopener" class="brand-link">
          <span class="brand-name"><img class="brand-icon" :src="partnerData[1].logo" :alt="partnerData[1].name"> {{ partnerData[1].name }}</span>
        </a>
      </div>
    </div>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import { useData } from 'vitepress';
import partnerData from '../../../data/partner.json';

const { lang } = useData();

const translations = {
  en: {
    collabText: 'Official Documentation Partner'
  },
  id: {
    collabText: 'Mitra Dokumentasi Resmi'
  }
};

const currentLocale = computed(() => {
  const shortLang = lang.value ? lang.value.split('-')[0] : 'en';
  return translations[shortLang] ? shortLang : 'en';
});

const collabText = computed(() => {
  return translations[currentLocale.value]?.collabText || translations.en.collabText;
});
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
  text-decoration: none;
  transition: opacity 0.25s;
}

.brand-link:hover {
  opacity: 0.7;
}

.brand-name {
  display: flex;
  font-weight: 600;
  font-size: 0.95rem;
  color: var(--vp-c-text-1);
}

.brand-icon {
  width: 24px;
  height: 24px;
  margin-right: 8px;
  vertical-align: middle;
}
</style>
