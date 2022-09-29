
import vue from '@vitejs/plugin-vue'
import { defineConfig } from 'vite'
import {resolve} from 'path'

export default defineConfig({
  plugins: [vue()],
  clearScreen: false,
  server: {
    strictPort: true,
    port: 5000
  },
  resolve:{
    alias:{
      '@': resolve(__dirname, "./src")
    }
  },
  envPrefix: ['VITE_', 'TAURI_'],
  build: {
    target: ['es2021', 'chrome100', 'safari13'],
    minify: !process.env.TAURI_DEBUG ? 'esbuild' : false,
    sourcemap: !!process.env.TAURI_DEBUG,
  },
})