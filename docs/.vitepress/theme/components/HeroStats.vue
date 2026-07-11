<template>
  <div class="hero-stats">
    <p> 
      {{ stats.weekly }}k {{ t('weekly') }} | 
      {{ stats.allTime }}k {{ t('allTime') }}
    </p>
  </div>
</template>

<script setup>
import { useData } from 'vitepress';
import stats from '../../../public/stats.json';

const { lang } = useData();

const translations = {
  en: {
    weekly: 'weekly downloads',
    allTime: 'all-time downloads'
  },
  id: {
    weekly: 'unduhan mingguan',
    allTime: 'unduhan total'
  }
};

const t = (key) => {
  const currentLang = lang.value || 'en';
  return translations[currentLang][key] || translations['en'][key];
};
</script>

<style scoped>
.hero-stats {
  margin-top: 24px;
  text-align: center;
  color: var(--vp-c-text-2);
  font-size: 0.9rem;
}
</style>
