<script setup lang="ts">
import { ref, watch } from "vue";
import { useChatStore } from "@/stores/chat-store";
import { MarkdownStreamParser } from "@/services/markdown-stream-parser";

const chatStore = useChatStore();
const parser = new MarkdownStreamParser();
const renderedHtml = ref("");

watch(
  () => chatStore.streamingContent,
  (content) => {
    renderedHtml.value = parser.append("");
    // Reset and re-render from scratch for safety
    parser.reset();
    renderedHtml.value = parser.append(content);
  },
  { immediate: true }
);
</script>

<template>
  <div class="flex gap-3">
    <div class="w-7 h-7 rounded-full bg-blue-600 flex items-center justify-center text-[10px] font-bold text-white shrink-0">
      AI
    </div>
    <div class="max-w-[80%] rounded-lg px-3 py-2 text-sm bg-surface-800 text-surface-200">
      <div
        class="prose prose-invert prose-sm max-w-none"
        v-html="renderedHtml"
      ></div>
      <span class="inline-block w-1.5 h-4 bg-surface-400 animate-pulse ml-0.5 align-middle"></span>
    </div>
  </div>
</template>
