import { defineConfig } from 'vitepress'

// https://vitepress.dev/reference/site-config
export default defineConfig({
  title: "LightVM Documentation",
  description: "A capability-based virtual machine designed for secure, predictable, and optimized bytecode execution.",
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Get Started', link: '/getStarted/installation' },
      { text: 'API References', link: '/api-references' }
    ],

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
              { text: 'Optimize Bytecode', link: 'methodFunctions/optimize-bytecode' }
            ]
          },
        ]
      }
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/soteenstudio/lightvm' }
    ],
    
    footer: {
      message: "Released under the Apache-2.0 License.",
      copyright: "Copyright © 2026 SoTeen Studio"
    }
  }
})
