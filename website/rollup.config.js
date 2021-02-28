import svelte from 'rollup-plugin-svelte';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';
import livereload from 'rollup-plugin-livereload';
import { terser } from 'rollup-plugin-terser';
import svelteSVG from 'rollup-plugin-svelte-svg';
import copy from 'rollup-plugin-copy';
import postcss from 'rollup-plugin-postcss';
import rust from '@wasm-tool/rollup-plugin-rust';

const production = !process.env.ROLLUP_WATCH;
const sourcemap = false;

export default {
  input: 'src/main.js',
  output: {
    sourcemap: sourcemap,
    format: 'iife',
    name: 'app',
    file: 'public/main.js',
  },
  plugins: [
    svelte({
      compilerOptions: {
        dev: !production,
      },
    }),
    postcss({
      extract: true,
      minimize: production,
    }),
    copy({
      targets: [{ src: ['static/*'], dest: 'public/' }],
      copyOnce: true,
    }),
    resolve({
      browser: true,
      dedupe: ['svelte'],
    }),
    commonjs(),
    svelteSVG(),
    rust({}),
    !production && serve(),
    !production && livereload('public'),
    production && terser(),
  ],
  watch: {
    clearScreen: false,
  },
};

function serve() {
  let server;

  function toExit() {
    if (server) server.kill(0);
  }

  return {
    writeBundle() {
      if (server) return;
      server = require('child_process').spawn(
        'npm',
        ['run', 'start', '--', '--dev'],
        {
          stdio: ['ignore', 'inherit', 'inherit'],
          shell: true,
        }
      );

      process.on('SIGTERM', toExit);
      process.on('exit', toExit);
    },
  };
}
