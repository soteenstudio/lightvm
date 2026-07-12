import { defineConfig } from 'vitepress';

export const enUs = defineConfig({
  themeConfig: {
    notFound: {
      title: 'PAGE NOT FOUND',
      quote: `But if you don't change your direction, and if you keep looking, you may end up where you are heading.`,
      linkText: 'Take me home',
      linkLabel: 'go to home'
    },
    
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Get Started', link: '/get-started/installation' },
      { text: 'API References', link: '/api-references/method-functions/run-method' },
      {
        text: 'About',
        items: [
          { text: 'License', link: 'https://github.com/soteenstudio/lightvm/blob/main/LICENSE' },
          { text: 'Code of Conduct', link: 'https://github.com/soteenstudio/lightvm/blob/main/.github/CODE_OF_CONDUCT.md' },
          { text: 'Security Policy', link: 'https://github.com/soteenstudio/lightvm/blob/main/.github/SECURITY.md' },
          { text: 'Releases', link: 'https://github.com/soteenstudio/lightvm/releases' }
        ]
      },
      {
        text: 'Support',
        items: [
          { text: 'Issues', link: 'https://github.com/soteenstudio/lightvm/issues' },
          { text: 'Discussions', link: 'https://github.com/soteenstudio/lightvm/discussions' }
        ]
      }
    ],

    darkModeSwitchLabel: 'Appearance',
    returnToTopLabel: 'Return to Top',

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

    sidebar: {
      '/get-started/': [
        {
          text: 'Get Started',
          collapsed: false,
          items: [
            { text: 'Installation', link: '/get-started/installation' },
            { text: 'Quick Usage', link: '/get-started/quick-usage' }
          ]
        },
        { text: 'API References', link: '/api-references/method-functions/run-method' }
      ],
      '/api-references/': [
        {
          text: 'Method Functions',
          collapsed: false,
          items: [
            { text: 'Run Method', link: '/api-references/method-functions/run-method' },
            { text: 'Provide Method', link: '/api-references/method-functions/provide-method' },
            { text: 'Inspect Method', link: '/api-references/method-functions/inspect-method' },
            { text: 'Halt Method', link: '/api-references/method-functions/halt-method' },
            { text: 'On Method', link: '/api-references/method-functions/on-method' },
            { text: 'Export Method', link: '/api-references/method-functions/export-method' },
            {
              text: 'Tools Method',
              collapsed: true,
              items: [
                { text: 'Optimize Bytecode Method', link: '/api-references/method-functions/tools-method/optimize-bytecode-method' },
                { text: 'Stringify Method', link: '/api-references/method-functions/tools-method/stringify-method' },
                { text: 'Parse Method', link: '/api-references/method-functions/tools-method/parse-method' },
                { text: 'Parse Array Method', link: '/api-references/method-functions/tools-method/parse-array-method' }
              ]
            },
          ]
        }
      ],
    },

    lastUpdated: {
      text: 'Last updated',
      formatOptions: {
        dateStyle: 'short',
        timeStyle: 'short'
      }
    },

    footer: {
      message: "Released under the Apache-2.0 License.",
      copyright: "Copyright © 2026 SoTeen Studio"
    }
  }
});