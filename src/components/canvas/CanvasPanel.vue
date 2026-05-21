<script setup lang="ts">
import { computed } from "vue";
import { useCanvasStore } from "@/stores/canvas-store";
import CanvasToolbar from "./CanvasToolbar.vue";
import MarkdownRenderer from "./MarkdownRenderer.vue";
import CodeBlockRenderer from "./CodeBlockRenderer.vue";

const canvasStore = useCanvasStore();

const isEmpty = computed(() => canvasStore.contentType === "empty");
</script>

<template>
  <div class="flex flex-col h-full bg-surface-950">
    <CanvasToolbar />

    <div class="flex-1 overflow-y-auto">
      <!-- Empty state -->
      <div
        v-if="isEmpty"
        class="flex items-center justify-center h-full text-surface-600 text-sm"
      >
        <div class="text-center">
          <svg class="w-12 h-12 mx-auto mb-3 text-surface-700" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-width="1.5" d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
          </svg>
          <p>Canvas will display generated content here</p>
          <p class="text-xs text-surface-700 mt-1">Code, markdown, configs, diffs...</p>
        </div>
      </div>

      <!-- Markdown renderer -->
      <MarkdownRenderer
        v-else-if="canvasStore.contentType === 'markdown'"
        :content="canvasStore.content as string"
      />

      <!-- Code renderer -->
      <CodeBlockRenderer
        v-else-if="canvasStore.contentType === 'code'"
        :code="canvasStore.content as string"
        :language="canvasStore.language"
      />

      <!-- Fallback -->
      <div v-else class="p-4 text-sm text-surface-300">
        <pre class="whitespace-pre-wrap">{{ canvasStore.content }}</pre>
      </div>
    </div>
  </div>
</template>
