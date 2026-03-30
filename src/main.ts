/**
 * main.ts — Vue application entry point.
 *
 * Bootstraps the Vue 3 application and mounts it to the `#app` DOM node
 * defined in `index.html`.
 *
 * Import order follows the conventional pattern:
 *   1. Global CSS (loaded first so base styles are available before any
 *      component's `<style scoped>` block is injected).
 *   2. Vue runtime.
 *   3. Root component.
 */

// ── Global styles ─────────────────────────────────────────────────────────────

import '@/assets/styles/main.css'

// ── Vue runtime ───────────────────────────────────────────────────────────────

import { createApp } from 'vue'

// ── Root component ────────────────────────────────────────────────────────────

import App from '@/App.vue'

// ── Mount ─────────────────────────────────────────────────────────────────────

createApp(App).mount('#app')
