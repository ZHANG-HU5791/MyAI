<script setup lang="ts">
import { getCurrentWindow } from "@tauri-apps/api/window";

const appWindow = getCurrentWindow();

function minimize() {
  appWindow.minimize();
}

function toggleMaximize() {
  appWindow.toggleMaximize();
}

function close() {
  appWindow.close();
}
</script>

<template>
  <div
    data-tauri-drag-region
    class="flex items-center justify-between h-9 bg-surface-900 border-b border-surface-800 px-3 shrink-0"
  >
    <!-- Left: app title -->
    <div class="flex items-center gap-2 pointer-events-none">
      <div class="w-2.5 h-2.5 rounded-full bg-blue-500"></div>
      <span class="text-xs font-medium text-surface-400">Omni-Creator Hub</span>
    </div>

    <!-- Right: window controls -->
    <div class="flex items-center gap-1">
      <button
        class="w-7 h-7 flex items-center justify-center rounded hover:bg-surface-700 text-surface-400 hover:text-surface-200 transition-colors"
        @click="minimize"
      >
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-width="2" d="M5 12h14" />
        </svg>
      </button>
      <button
        class="w-7 h-7 flex items-center justify-center rounded hover:bg-surface-700 text-surface-400 hover:text-surface-200 transition-colors"
        @click="toggleMaximize"
      >
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <rect x="4" y="4" width="16" height="16" rx="1" stroke-width="2" />
        </svg>
      </button>
      <button
        class="w-7 h-7 flex items-center justify-center rounded hover:bg-red-600 text-surface-400 hover:text-white transition-colors"
        @click="close"
      >
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-width="2" d="M6 6l12 12M6 18L18 6" />
        </svg>
      </button>
    </div>
  </div>
</template>
