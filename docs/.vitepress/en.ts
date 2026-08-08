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
      navCommunity,
      navDevelopment,
    ],

    sidebar: {
      '/get-started/': sidebarGetStarted,
      '/api-reference/': sidebarAPIReferences,
      '/concepts/': sidebarConcepts,
    },
  },
});

export const enUsSearch = {
  translations: {
    placeholder: 'Search docs',
    button: {
      buttonText: 'Search',
      buttonAriaLabel: 'Search',
    },
    modal: {
      searchBox: {
        resetButtonText: 'Clear the query',
        resetButtonAriaLabel: 'Clear the query',
        cancelButtonText: 'Cancel',
        cancelButtonAriaLabel: 'Cancel',
      },
      startScreen: {
        recentSearchesTitle: 'Recent',
        noRecentSearchesText: 'No recent searches',
        saveRecentSearchButtonTitle: 'Save to recent searches',
        removeRecentSearchButtonTitle: 'Remove to recent searches',
        favoriteSearchesTitle: 'Favorite',
        removeFavoriteSearchButtonTitle: 'Remove from favorites',
      },
      errorScreen: {
        titleText: 'Unable to fetch results',
        helpText: 'You might want to check your network connection',
      },
      footer: {
        selectText: 'to select',
        navigateText: 'to navigate',
        closeText: 'to close',
        searchByText: 'Search by',
      },
      noResultsScreen: {
        noResultsText: 'No results for',
        suggestedQueryText: 'Try searching for',
        reportMissingResultsText: 'Believe this query should return results?',
        reportMissingResultsLinkText: 'Let us know',
      },
    },
  },
};
