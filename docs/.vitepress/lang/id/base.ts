export const baseConfig = {
  notFound: {
    title: 'HALAMAN TIDAK DITEMUKAN',
    quote: `Namun jika Anda tidak mengubah arah, dan jika Anda terus mencari, Anda mungkin akan berakhir di tempat yang Anda tuju.`,
    linkText: 'Kembali ke beranda',
    linkLabel: 'kembali ke beranda',
  },

  darkModeSwitchLabel: 'Tema',
  returnToTopLabel: 'Kembali ke Atas',

  outline: {
    label: 'Pada halaman ini',
  },

  editLink: {
    pattern: 'https://github.com/soteenstudio/lightvm/edit/main/docs/:path',
    text: 'Edit halaman ini di GitHub',
  },

  docFooter: {
    prev: 'Halaman sebelumnya',
    next: 'Halaman berikutnya',
  },

  lastUpdated: {
    text: 'Terakhir diperbarui',
    formatOptions: {
      dateStyle: 'short',
      timeStyle: 'short',
    },
  } as const,

  footer: {
    message: 'Dirilis di bawah <a href="https://www.apache.org/licenses/LICENSE-2.0" target="_blank">Lisensi Apache-2.0</a>.',
    copyright: 'Hak cipta © 2025-2026 <a href="https://github.com/soteenstudio">SoTeen Studio</a>',
  },
};
