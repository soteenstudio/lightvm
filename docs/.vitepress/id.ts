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

export const idIdSearch = {
  placeholder: 'Cari docs',
  translations: {
    button: {
      buttonText: 'Cari',
      buttonAriaLabel: 'Cari',
    },
    modal: {
      searchBox: {
        resetButtonText: 'Bersihkan kueri',
        resetButtonAriaLabel: 'Bersihkan kueri',
        cancelButtonText: 'Batal',
        cancelButtonAriaLabel: 'Batal',
      },
      startScreen: {
        recentSearchesTitle: 'Terbaru',
        noRecentSearchesText: 'Tidak ada pencarian terbaru',
        saveRecentSearchButtonTitle: 'Simpan ke pencarian terbaru',
        removeRecentSearchButtonTitle: 'Hapus dari pencarian terbaru',
        favoriteSearchesTitle: 'Favorit',
        removeFavoriteSearchButtonTitle: 'Hapus dari favorit',
      },
      errorScreen: {
        titleText: 'Tidak dapat mengambil hasil',
        helpText: 'Anda mungkin perlu memeriksa koneksi jaringan Anda',
      },
      footer: {
        selectText: 'untuk memilih',
        navigateText: 'untuk navigasi',
        closeText: 'untuk menutup',
        searchByText: 'Pencarian oleh',
      },
      noResultsScreen: {
        noResultsText: 'Tidak ada hasil untuk',
        suggestedQueryText: 'Coba cari',
        reportMissingResultsText:
          'Apakah Anda yakin kueri ini akan menghasilkan hasil?',
        reportMissingResultsLinkText: 'Beri tahu kami',
      },
    },
  },
};
