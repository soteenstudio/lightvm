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
