import { defineConfig } from 'vitepress';
import { enUs } from './en.js';
import { idId } from './id.js';
import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

// Mendapatkan direktori file saat ini
const __dirname = path.dirname(fileURLToPath(import.meta.url));

// Membaca file SVG
const donationSvg = fs.readFileSync(path.resolve(__dirname, '../public/assets/donation-box.svg'), 'utf-8');

// https://vitepress.dev/reference/site-config
export default defineConfig({
  rewrites: {
    'en/:rest*': ':rest*',
  },

  title: 'LightVM',
  titleTemplate: ':title | LightVM Docs',
  description:
    'A capability-based virtual machine designed for secure, predictable, and optimized bytecode execution.',

  cleanUrls: true,

  head: [
    [
      'meta',
      {
        name: 'theme-color',
        content: '#ffffff',
        media: '(prefers-color-scheme: light)',
      },
    ],
    [
      'meta',
      {
        name: 'theme-color',
        content: '#16141C',
        media: '(prefers-color-scheme: dark)',
      },
    ],
    [
      'script',
      { type: 'application/ld+json' },
      JSON.stringify({
        '@context': 'https://schema.org',
        '@type': 'SoftwareSourceCode',
        name: 'LightVM',
        description:
          'A capability-based virtual machine designed for secure, predictable, and optimized bytecode execution.',
        codeRepository: 'https://github.com/soteenstudio/lightvm',
        programmingLanguage: 'Rust',
        author: {
          '@type': 'Person',
          name: 'Claycuy',
        },
      }),
    ],
  ],

  sitemap: {
    hostname: 'https://lightvm.vercel.app',
  },

  transformPageData(pageData: any) {
    if (!pageData.content) {
      return;
    }

    if (
      pageData.relativePath === 'index.md' ||
      pageData.frontmatter.layout === 'home'
    ) {
      return;
    }

    const lines = (pageData.content as string)
      .split('\n')
      .filter((line: string) => line.trim() !== '');

    if (lines.length >= 2 && !pageData.frontmatter.description) {
      const desc = lines[1].replace(/[#*`]/g, '').trim();
      pageData.frontmatter.description = desc;
    }
  },

  lastUpdated: true,
  locales: {
    root: {
      label: 'English (US)',
      lang: 'en-US',
      ...enUs,
    },
    id: {
      label: 'Bahasa Indonesia',
      lang: 'id-ID',
      link: '/id/',
      dir: 'id',
      ...idId,
    },
  },
  themeConfig: {
    // https://vitepress.dev/reference/default-theme-config
    search: {
      provider: 'local',
    },

    socialLinks: [
      { icon: 'github', link: 'https://github.com/soteenstudio/lightvm' },
      { icon: 'instagram', link: 'https://instagram.com/soteenstudio' },
{
  icon: {
    svg: donationSvg,
  },
  link: 'https://trakteer.id/soteen_studio/tip?quantity=1',
  ariaLabel: 'trakteer'
}

    ],
  },
});
