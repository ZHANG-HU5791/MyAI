<script setup lang="ts">
import { computed } from "vue";
import type { Message } from "@/stores/chat-store";
import { renderMarkdown } from "@/services/markdown-stream-parser";

const props = defineProps<{ message: Message }>();

const isUser = computed(() => props.message.role === "user");
const renderedContent = computed(() =>
  isUser.value ? props.message.content : renderMarkdown(props.message.content)
);
</script>

<template>
  <div
    class="flex gap-3"
    :class="{ 'justify-end': isUser }"
  >
    <!-- Avatar -->
    <div
      v-if="!isUser"
      class="w-7 h-7 rounded-full bg-blue-600 flex items-center justify-center text-[10px] font-bold text-white shrink-0"
    >
      AI
    </div>

    <!-- Content -->
    <div
      class="max-w-[80%] rounded-lg px-3 py-2 text-sm"
      :class="isUser ? 'bg-blue-600 text-white' : 'bg-surface-800 text-surface-200'"
    >
      <div
        v-if="isUser"
        class="whitespace-pre-wrap"
      >{{ message.content }}</div>
      <div
        v-else
        class="prose prose-invert prose-sm max-w-none"
        v-html="renderedContent"
      ></div>
    </div>

    <!-- User avatar -->
    <div
      v-if="isUser"
      class="w-7 h-7 rounded-full bg-surface-700 flex items-center justify-center text-[10px] font-bold text-surface-300 shrink-0"
    >
      U
    </div>
  </div>
</template>
