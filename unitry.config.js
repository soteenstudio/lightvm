export default {
  concurrency: 1,
  timeout: 5000,
  bail: false,
  esbuild: {
    external: ["node:*"],
  },
}