# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

Mobile lock screen UI prototype — a Vue 3 SPA simulating an iOS-style lock screen with swipe-to-unlock, clock, status bar, and a minimal home screen. This directory is part of a larger monorepo but is self-contained as a standalone Vite project.

## Commands

| Task | Command |
|------|---------|
| Dev server | `npm run dev` |
| Production build | `npm run build` (runs `vue-tsc -b` then `vite build`) |
| Preview build | `npm run preview` |

No test runner or linter is configured. TypeScript strict flags (`noUnusedLocals`, `noUnusedParameters`) serve as a lightweight check.

## Architecture

**Stack:** Vue 3 Composition API (`<script setup>`), TypeScript, Vite 8, plain CSS with custom properties.

**Screen switching:** `App.vue` holds a `ref<'lock' | 'home'>` — no router, no state library. `LockScreen` emits `unlock` (swipe gesture, 80px threshold), `HomeScreen` emits `lock`. Vue `<Transition>` animates between them.

**Component tree:**
- `App.vue` → `LockScreen.vue` | `HomeScreen.vue`
  - `StatusBar.vue` — iOS-style (Dynamic Island, wifi, battery)
  - `TimeDisplay.vue` — clock + date, polls `new Date()` every second
  - `BottomActions.vue` — flashlight toggle, camera button (local state only)
  - `SwipeIndicator.vue` — animated bottom bar

**Styling:** Global CSS custom properties in `src/style.css` (warm-white palette, glassmorphism tokens, shadows). Components use `<style scoped>`. Responsive breakpoints: `≤380px` (small phones), `≥768px` (tablets). iOS aesthetic: SF Pro font stack, blur effects, Dynamic Island.

**Assets:** SVG icons in `public/icons.svg` (sprite sheet), `public/favicon.svg`.
