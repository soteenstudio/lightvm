import { defineConfig } from 'vitepress'
import { enUs } from './en.js'
import { idId } from './id.js';

// https://vitepress.dev/reference/site-config
export default defineConfig({
  rewrites: {
    'en/:rest*': ':rest*'
  },
  
  title: "LightVM",
  titleTemplate: ":title | LightVM Docs",
  description: "A capability-based virtual machine designed for secure, predictable, and optimized bytecode execution.",

  head: [
    // Tambahkan di dalam head di config.js
    ['script', { type: 'application/ld+json' }, JSON.stringify({
      "@context": "https://schema.org",
      "@type": "SoftwareSourceCode",
      "name": "LightVM",
      "description": "A capability-based virtual machine designed for secure, predictable, and optimized bytecode execution.",
      "codeRepository": "https://github.com/soteenstudio/lightvm",
      "programmingLanguage": "Rust", // Atau bahasa utama VM-mu
      "author": {
        "@type": "Person",
        "name": "Claycuy"
      }
    })]
  ],

  sitemap: {
    hostname: 'https://lightvm.vercel.app'
  },

  transformPageData(pageData) {
    // 1. Tambahkan safety check: kalau content gak ada, langsung return
    if (!pageData.content) {
      return;
    }

    // 2. Tetap cek apakah ini halaman home
    if (pageData.relativePath === 'index.md' || pageData.frontmatter.layout === 'home') {
      return;
    }

    // 3. Sekarang split aman dilakukan karena kita udah tau pageData.content itu ada
    const lines = pageData.content.split('\n').filter(line => line.trim() !== '');

    if (lines.length >= 2 && !pageData.frontmatter.description) {
      const desc = lines[1].replace(/[#*`]/g, '').trim(); 
      pageData.frontmatter.description = desc;
    }
  },

  lastUpdated: true,
  locales: {
    root: {
      label: 'English',
      lang: 'en-US',
      ...enUs
    },
    id: {
      label: 'Bahasa Indonesia',
      lang: 'id-ID',
      link: '/id/',
      dir: 'id',
      ...idId
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
