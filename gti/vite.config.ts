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
    rollupOptions: {
      output: {
        manualChunks(id) {
          if (id.includes('node_modules')) {
            if (id.includes('three')) {
              return 'vendor-three';
            }
            if (id.includes('d3')) {
              return 'vendor-d3';
            }
            if (id.includes('gsap')) {
              return 'vendor-gsap';
            }
            if (id.includes('vue')) {
              return 'vendor-vue';
            }
            return 'vendor';
          }
        },
      },
    },
    chunkSizeWarningLimit: 500,
    cssCodeSplit: true,
    sourcemap: false,
  },
  css: {
    devSourcemap: true,
    modules: {
      localsConvention: 'camelCaseOnly',
    },
  },
})
