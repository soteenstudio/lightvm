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
      { text: 'Get Started', link: '/getStarted/installation' },
      { text: 'API References', link: '/api-references' },
      {
        text: 'About',
        items: [
          { text: 'License', link: 'https://github.com/soteenstudio/lightvm/blob/main/LICENSE' },
          { text: 'Code of Conduct', link: 'https://github.com/soteenstudio/lightvm/blob/main/.github/CODE_OF_CONDUCT.md' },
          { text: 'Security Policy', link: 'https://github.com/soteenstudio/lightvm/blob/main/.github/SECURITY.md' },
          { text: 'Releases', link: 'https://github.com/soteenstudio/lightvm/releases' }
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

    sidebar: [
      {
        text: 'Get Started',
        collapsed: false,
        items: [
          { text: 'Installation', link: '/getStarted/installation' },
          { text: 'Quick Usage', link: '/getStarted/quick-usage' }
        ]
      },
      {
        text: 'Method Functions',
        collapsed: false,
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
              { text: 'Optimize Bytecode Method', link: '/methodFunctions/optimize-bytecode' },
              { text: 'Stringify Method', link: '/methodFunctions/optimize-bytecode' }
            ]
          },
        ]
      }
    ],

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