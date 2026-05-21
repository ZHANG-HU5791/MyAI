<script setup lang="ts">
import { computed } from "vue";
import { renderMarkdown } from "@/services/markdown-stream-parser";

const props = defineProps<{ content: string }>();

const safeHtml = computed(() => renderMarkdown(props.content));
</script>

<template>
  <div class="p-4">
    <div
      class="prose prose-invert prose-sm max-w-none"
      v-html="safeHtml"
    ></div>
  </div>
</template>
