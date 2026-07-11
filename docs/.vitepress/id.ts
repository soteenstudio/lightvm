import { defineConfig } from 'vitepress';

export const idId = defineConfig({
  themeConfig: {
    notFound: {
      title: 'HALAMAN TIDAK DITEMUKAN',
      quote: `Namun jika Anda tidak mengubah arah, dan jika Anda terus mencari, Anda mungkin akan berakhir di tempat yang Anda tuju.`,
      linkText: 'Kembali ke beranda',
      linkLabel: 'kembali ke beranda'
    },
    
    nav: [
      { text: 'Beranda', link: '/id/' },
      { text: 'Memulai', link: '/id/get-started/installation' },
      { text: 'Referensi API', link: '/id/api-references' },
      {
        text: 'Tentang',
        items: [
          { text: 'Lisensi', link: 'https://github.com/soteenstudio/lightvm/blob/main/LICENSE' },
          { text: 'Kode Etik', link: 'https://github.com/soteenstudio/lightvm/blob/main/.github/CODE_OF_CONDUCT.md' },
          { text: 'Kebijakan Keamanan', link: 'https://github.com/soteenstudio/lightvm/blob/main/.github/SECURITY.md' },
          { text: 'Rilis', link: 'https://github.com/soteenstudio/lightvm/releases' }
        ]
      },
      {
        text: 'Dukungan',
        items: [
          { text: 'Masalah', link: 'https://github.com/soteenstudio/lightvm/issues' },
          { text: 'Diskusi', link: 'https://github.com/soteenstudio/lightvm/discussions' }
        ]
      }
    ],

    darkModeSwitchLabel: 'Tema',
    returnToTopLabel: 'Kembali ke Atas',

    outline: {
      label: 'Pada halaman ini'
    },

    editLink: {
      pattern: 'https://github.com/soteenstudio/lightvm/edit/main/docs/:path',
      text: 'Edit halaman ini di GitHub'
    },

    docFooter: {
      prev: 'Halaman sebelumnya',
      next: 'Halaman berikutnya'
    },

    sidebar: [
      {
        text: 'Memulai',
        collapsed: false,
        items: [
          { text: 'Instalasi', link: '/id/get-started/installation' },
          { text: 'Penggunaan Cepat', link: '/id/get-started/quick-usage' }
        ]
      },
      {
        text: 'Fungsi Metode',
        collapsed: false,
        items: [
          { text: 'Metode Run', link: '/id/method-functions/run-method' },
          { text: 'Metode Provide', link: '/id/method-functions/provide-method' },
          { text: 'Metode Inspect', link: '/id/method-functions/inspect-method' },
          { text: 'Metode Halt', link: '/id/method-functions/halt-method' },
          { text: 'Metode On', link: '/id/method-functions/on-method' },
          { text: 'Metode Export', link: '/id/method-functions/export-method' },
          {
            text: 'Metode Tools',
            collapsed: true,
            items: [
              { text: 'Metode Optimize Bytecode', link: '/id/method-functions/optimize-bytecode' },
              { text: 'Metode Stringify', link: '/id/method-functions/optimize-bytecode' }
            ]
          },
        ]
      }
    ],

    lastUpdated: {
      text: 'Terakhir diperbarui',
      formatOptions: {
        dateStyle: 'short',
        timeStyle: 'short'
      }
    },

    footer: {
      message: "Dirilis di bawah Lisensi Apache-2.0.",
      copyright: "Hak cipta © 2026 SoTeen Studio"
    }
  }
});