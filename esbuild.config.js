import esbuild from 'esbuild';
const builds = [
  {
    entryPoints: ['./ts/src/index.ts'],
    format: 'esm',
    outfile: './ts/dist/index.min.mjs',
  },
  {
    entryPoints: ['./ts/src/index.ts'],
    format: 'cjs',
    outfile: './ts/dist/index.min.cjs',
  },
];
for (const config of builds) {
  esbuild
    .build({
      bundle: true,
      minify: true,
      sourcemap: true,
      platform: 'node',
      ...config,
      external: ['chalk', '@lightvm/core-*'],
    })
    .catch(() => process.exit(1));
}
