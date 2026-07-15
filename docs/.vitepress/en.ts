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
        text: 'API Reference',
        link: '/api-reference/method-functions/run-method',
      },
      { text: 'Concepts', link: '/concepts/what-is' },
      navAbout,
      navSupport,
    ],

    sidebar: {
      '/get-started/': sidebarGetStarted,
      '/api-reference/': sidebarAPIReferences,
      '/concepts/': sidebarConcepts,
    },
  },
});
