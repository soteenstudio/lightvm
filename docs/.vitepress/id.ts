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
      { text: 'Memulai', link: '/id/getStarted/installation' },
      { text: 'Referensi API', link: '/id/api-references' },
      {
        text: 'Tentang',
        items: [
          { text: 'Lisensi', link: 'https://github.com/soteenstudio/lightvm/blob/main/LICENSE' },
          { text: 'Kode Etik', link: 'https://github.com/soteenstudio/lightvm/blob/main/.github/CODE_OF_CONDUCT.md' },
          { text: 'Kebijakan Keamanan', link: 'https://github.com/soteenstudio/lightvm/blob/main/.github/SECURITY.md' },
          { text: 'Rilis', link: 'https://github.com/soteenstudio/lightvm/releases' }
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
          { text: 'Instalasi', link: '/id/getStarted/installation' },
          { text: 'Penggunaan Cepat', link: '/id/getStarted/quick-usage' }
        ]
      },
      {
        text: 'Fungsi Metode',
        collapsed: false,
        items: [
          { text: 'Metode Run', link: '/id/methodFunctions/run-method' },
          { text: 'Metode Provide', link: '/id/methodFunctions/provide-method' },
          { text: 'Metode Inspect', link: '/id/methodFunctions/inspect-method' },
          { text: 'Metode Halt', link: '/id/methodFunctions/halt-method' },
          { text: 'Metode On', link: '/id/methodFunctions/on-method' },
          { text: 'Metode Export', link: '/id/methodFunctions/export-method' },
          {
            text: 'Metode Tools',
            collapsed: true,
            items: [
              { text: 'Metode Optimize Bytecode', link: '/id/methodFunctions/optimize-bytecode' },
              { text: 'Metode Stringify', link: '/id/methodFunctions/optimize-bytecode' }
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