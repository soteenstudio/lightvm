import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "LightVM",
  description: "A capability-based virtual machine designed for secure, predictable, and optimized bytecode execution.",
  locales: {
    root: {
      label: 'English',
      lang: 'en',
      themeConfig: {
        nav: [
          { text: 'Home', link: '/' },
          { text: 'Get Started', link: '/getStarted/installation' },
          { text: 'API References', link: '/api-references' }
        ],

        darkModeSwitchLabel: 'Appearance',

        outline: {
          label: 'On this page'
        },

        editLink: {
          pattern: 'https://github.com/soteenstudio/lightvm/edit/main/docs/:path',
          text: 'Edit this page on GitHub'
        },

        docFooter: {
          prev: 'Previous page',
          next: 'Next page'
        },

        sidebar: [
          {
            text: 'Get Started',
            items: [
              { text: 'Installation', link: '/getStarted/installation' },
              { text: 'Quick Usage', link: '/getStarted/quick-usage' }
            ]
          },
          {
            text: 'Method Functions',
            items: [
              { text: 'Run Method', link: '/methodFunctions/run-method' },
              { text: 'Provide Method', link: '/methodFunctions/provide-method' },
              { text: 'Inspect Method', link: '/methodFunctions/inspect-method' },
              { text: 'Halt Method', link: '/methodFunctions/halt-method' },
              { text: 'On Method', link: '/methodFunctions/on-method' },
              { text: 'Export Method', link: '/methodFunctions/export-method' },
              {
                text: 'Tools Method',
                collapsed: true,
                items: [
                  { text: 'Optimize Bytecode', link: 'methodFunctions/optimize-bytecode' },
                  { text: 'Stringify <Badge type="warning" text="Nightly" />', link: 'methodFunctions/optimize-bytecode' }
                ]
              },
            ]
          }
        ],

        footer: {
          message: "Released under the Apache-2.0 License.",
          copyright: "Copyright © 2026 SoTeen Studio"
        }
      }
    },
    id: {
      label: 'Bahasa Indonesia',
      lang: 'id',
      link: '/id/',
      themeConfig: {
        nav: [
          { text: 'Beranda', link: '/id/' },
          { text: 'Memulai', link: '/id/getStarted/installation' },
          { text: 'Referensi API', link: '/id/api-references' }
        ],

        darkModeSwitchLabel: 'Tema',

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
            items: [
              { text: 'Instalasi', link: '/id/getStarted/installation' },
              { text: 'Penggunaan Cepat', link: '/id/getStarted/quick-usage' }
            ]
          },
          {
            text: 'Fungsi Metode',
            items: [
              { text: 'Metode Run', link: '/methodFunctions/run-method' },
              { text: 'Metode Provide', link: '/methodFunctions/provide-method' },
              { text: 'Metode Inspect', link: '/methodFunctions/inspect-method' },
              { text: 'Metode Halt', link: '/methodFunctions/halt-method' },
              { text: 'Metode On', link: '/methodFunctions/on-method' },
              { text: 'Metode Export', link: '/methodFunctions/export-method' },
              {
                text: 'Metode Tools',
                collapsed: true,
                items: [
                  { text: 'Optimize Bytecode', link: 'methodFunctions/optimize-bytecode' },
                  { text: 'Stringify <Badge type="warning" text="Nightly" />', link: 'methodFunctions/optimize-bytecode' }
                ]
              },
            ]
          }
        ],

        footer: {
          message: "Dirilis di bawah Lisensi Apache-2.0.",
          copyright: "Hak cipta © 2026 SoTeen Studio"
        }
      }
    }
  },
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    search: {
      provider: 'local'
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/soteenstudio/lightvm' }
    ],
  }
})
