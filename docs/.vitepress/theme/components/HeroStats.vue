<template>
  <div class="hero-stats">
    <p>{{ displayStats }}</p>
  </div>
</template>

<script setup>
import { computed } from 'vue';
import { useData } from 'vitepress';
import stats from '../../../data/stats.json';

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

const currentLocale = computed(() => {
  const shortLang = lang.value ? lang.value.split('-')[0] : 'en';
  return translations[shortLang] ? shortLang : 'en';
});

const t = (key) => {
  return translations[currentLocale.value][key] || translations['en'][key];
};

const displayStats = computed(() => {
  return `${stats.weekly}k ${t('weekly')} | ${stats.allTime}k ${t('allTime')}`;
});
</script>

<style scoped>
.hero-stats {
  margin-top: 24px;
  text-align: center;
  color: var(--vp-c-text-2);
  font-size: 0.9rem;
}
</style>
