<template>
  <div
    v-if="!hasSidebar"
    class="partner-footer-container"
  >
    <div class="partner-footer-content">
      <p class="collab-text">{{ displayCollabText }}</p>

      <div class="logo-wrapper">
        <template
          v-for="(item, index) in items"
          :key="item.name"
        >
          <a
            v-if="item.website && item.website.trim() !== ''"
            :href="item.website"
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
                <span class="brand-title">{{ item.name }}</span>
                {{ getLocaleDescription(item, currentLocale) }}
              </span>
          
              <img
                v-if="item.logo && item.logo.trim() !== ''"
                class="brand-icon"
                :src="item.logo"
                :alt="item.name"
              >
              <span v-else class="brand-icon-fallback">
                {{ item.name.charAt(0).toUpperCase() }}
              </span>
            </span>
          </a>

          <div
            v-else
            :aria-describedby="`partner-tooltip-${index}`"
            class="brand-link"
            @click="triggerFunEasterEgg(item)"
          >
            <span class="brand-name">
              <span
                :id="`partner-tooltip-${index}`"
                role="tooltip"
                class="brand-desc"
              >
                <span class="brand-title">{{ item.name }}</span>
                {{ getLocaleDescription(item, currentLocale) }}
              </span>
          
              <img
                v-if="item.logo && item.logo.trim() !== ''"
                class="brand-icon"
                :src="item.logo"
                :alt="item.name"
              >
              <span v-else class="brand-icon-fallback">
                {{ item.name.charAt(0).toUpperCase() }}
              </span>
            </span>
          </div>
        </template>
      </div>
    </div>
  </div>

  <transition name="fade">
    <div v-if="alertMessage" class="custom-alert-overlay" @click="closeAlert">
      <div class="custom-alert-box" @click.stop>
        <p class="custom-alert-text">{{ alertMessage }}</p>
        <button class="custom-alert-btn" @click="closeAlert">OK</button>
      </div>
    </div>
  </transition>
</template>

<script setup>
import { computed, ref } from 'vue';
import { useData } from 'vitepress';
import { useSidebar } from 'vitepress/theme';

const props = defineProps({
  items: {
    type: Array,
    required: true,
  },
  translations: {
    type: Object,
    required: true,
  },
});

const { lang } = useData();
const { hasSidebar } = useSidebar();

const alertMessage = ref('');

const fallbackDescriptions = {
  en: 'Description not available',
  id: 'Deskripsi belum tersedia',
};

const currentLocale = computed(() => {
  const shortLang = lang.value?.split('-')[0] ?? 'en';
  return props.translations[shortLang] ? shortLang : 'en';
});

const displayCollabText = computed(() => {
  const t = props.translations[currentLocale.value];
  return typeof t === 'object' ? t.collabText ?? t.en ?? '' : t;
});

function getLocaleDescription(item, locale) {
  const desc = item.description?.[locale] ?? item.description?.en;
  
  if (!desc || desc.trim() === '') {
    return fallbackDescriptions[locale] ?? fallbackDescriptions.en;
  }
  
  return desc;
}

const easterEggMessages = {
  en: (name) => `${name} is currently offline or under maintenance. Please check back later.`,
  id: (name) => `${name} saat ini sedang dalam pemeliharaan atau belum tersedia. Silakan kunjungi kembali nanti.`,
};

function triggerFunEasterEgg(item) {
  const msgFn = easterEggMessages[currentLocale.value] || easterEggMessages.en;
  alertMessage.value = msgFn(item.name);
}

function closeAlert() {
  alertMessage.value = '';
}
</script>
