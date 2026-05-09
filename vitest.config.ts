// vitest.config.ts
import { defineConfig } from 'vitest/config'

export default defineConfig({
  test: {
    // Biar gak perlu import { describe, it, expect } di tiap file
    globals: true,
    environment: 'node', // Ganti ke 'jsdom' atau 'happy-dom' kalau buat web/React
  },
})
