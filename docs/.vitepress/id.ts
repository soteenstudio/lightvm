import { defineConfig } from 'vitepress';
import {
  baseConfig,
  navAbout,
  navSupport,
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
        link: '/id/api-references/method-functions/run-method',
      },
      { text: 'Konsep', link: '/id/concepts/what-is' },
      navAbout,
      navSupport,
    ],

    sidebar: {
      '/id/get-started/': sidebarGetStarted,
      '/id/api-references/': sidebarAPIReferences,
      '/id/concepts/': sidebarConcepts,
    },
  },
});
