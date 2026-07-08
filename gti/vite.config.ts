import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig({
  plugins: [vue()],
  server: {
    port: 8091,
    host: true,
  },
  build: {
    target: ['es2018', 'chrome63', 'firefox57', 'safari11', 'edge79'],
    cssTarget: ['chrome63', 'firefox57', 'safari11', 'edge79'],
  },
  css: {
    devSourcemap: true,
  },
})
