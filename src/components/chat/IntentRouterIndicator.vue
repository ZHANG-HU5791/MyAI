<script setup lang="ts">
import { useChatStore } from "@/stores/chat-store";

const chatStore = useChatStore();
</script>

<template>
  <div
    v-if="chatStore.intentRouting"
    class="flex items-center gap-2 px-4 py-1.5 bg-surface-900/50 border-b border-surface-800 text-[10px] text-surface-500"
  >
    <span class="inline-block w-1.5 h-1.5 rounded-full bg-green-500"></span>
    <span>Routed to <strong class="text-surface-300">{{ chatStore.intentRouting.model }}</strong></span>
    <span class="text-surface-600">·</span>
    <span>{{ chatStore.intentRouting.reason }}</span>
  </div>
</template>
