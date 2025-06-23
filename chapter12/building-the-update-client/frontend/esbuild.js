const esbuild = require('esbuild');
const cssModulesPlugin = require('esbuild-css-modules-plugin');

esbuild.build({
    plugins: [cssModulesPlugin()],
    entryPoints: ['src/index.tsx'],
    bundle: true,
    outfile: 'public/bundle.js',
    format: 'esm',
    define: {
      'process.env.NODE_ENV': '"production"',
    },
    minify: true,
    sourcemap: true,
    loader: {
      '.js': 'jsx',
      '.tsx': 'tsx',
      '.ts': 'ts',
      '.wasm': 'binary',
      '.css': 'css'
    },
  }).catch(() => process.exit(1));
  