import { defineConfig } from 'vitepress';
import {
  baseConfig,
  navAbout,
  navSupport,
  navCommunity,
  navDevelopment,
  sidebarGetStarted,
  sidebarAPIReferences,
  sidebarConcepts,
} from './lang/id/index.js';

export const idId = defineConfig({
  themeConfig: {
    ...baseConfig,

    nav: [
      { text: 'Beranda', link: '/id/' },
      { text: 'Memulai', link: '/id/get-started/installation' },
      {
        text: 'Referensi API',
        link: '/id/api-reference/method-functions/run-method',
      },
      { text: 'Konsep', link: '/id/concepts/what-is' },
      navAbout,
      navSupport,
      navCommunity,
      navDevelopment,
    ],

    sidebar: {
      '/id/get-started/': sidebarGetStarted,
      '/id/api-reference/': sidebarAPIReferences,
      '/id/concepts/': sidebarConcepts,
    },
  },
});
