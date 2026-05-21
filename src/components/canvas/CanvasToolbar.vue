<script setup lang="ts">
import { useCanvasStore } from "@/stores/canvas-store";
import { useCanvasInteraction } from "@/composables/useCanvasInteraction";

const canvasStore = useCanvasStore();
const { copyToClipboard } = useCanvasInteraction();

async function copyContent() {
  const text = typeof canvasStore.content === "string"
    ? canvasStore.content
    : JSON.stringify(canvasStore.content, null, 2);
  await copyToClipboard(text);
}
</script>

<template>
  <div class="flex items-center justify-between px-3 py-2 border-b border-surface-800 shrink-0">
    <div class="flex items-center gap-2">
      <span class="text-xs font-medium text-surface-400">Canvas</span>
      <span
        v-if="canvasStore.contentType !== 'empty'"
        class="px-1.5 py-0.5 rounded text-[10px] bg-surface-800 text-surface-500"
      >
        {{ canvasStore.contentType }}
      </span>
    </div>
    <div class="flex items-center gap-1">
      <button
        v-if="canvasStore.contentType !== 'empty'"
        class="button-ghost text-xs"
        @click="copyContent"
      >
        Copy
      </button>
    </div>
  </div>
</template>
