import { defineConfig } from 'vitepress';
import {
  baseConfig,
  navAbout,
  navSupport,
  sidebarGetStarted,
  sidebarAPIReferences,
  sidebarConcepts,
} from './lang/en/index.js';

export const enUs = defineConfig({
  themeConfig: {
    ...baseConfig,

    nav: [
      { text: 'Home', link: '/' },
      { text: 'Get Started', link: '/get-started/installation' },
      {
        text: 'API References',
        link: '/api-references/method-functions/run-method',
      },
      { text: 'Concepts', link: '/concepts/what-is' },
      navAbout,
      navSupport,
    ],

    sidebar: {
      '/get-started/': sidebarGetStarted,
      '/api-references/': sidebarAPIReferences,
      '/concepts/': sidebarConcepts,
    },
  },
});
