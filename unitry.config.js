export default {
  concurrency: 4,
  timeout: 5000,
  bail: false,
  esbuild: {
    external: ["node:*"],
  },
}