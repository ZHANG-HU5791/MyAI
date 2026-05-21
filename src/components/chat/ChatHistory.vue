<script setup lang="ts">
import { useChatStore } from "@/stores/chat-store";

defineEmits<{ close: [] }>();

const chatStore = useChatStore();

async function selectSession(id: string) {
  chatStore.activeSessionId = id;
  await chatStore.loadMessages(id);
}
</script>

<template>
  <div class="flex flex-col bg-surface-900 h-full">
    <div class="flex items-center justify-between px-3 py-2 border-b border-surface-800">
      <span class="text-xs font-medium text-surface-400">Sessions</span>
      <button class="button-ghost text-xs" @click="$emit('close')">
        <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
        </svg>
      </button>
    </div>
    <div class="flex-1 overflow-y-auto">
      <button
        v-for="session in chatStore.sessions"
        :key="session.id"
        class="w-full text-left px-3 py-2 text-xs hover:bg-surface-800 transition-colors"
        :class="{ 'bg-surface-800 text-surface-100': session.id === chatStore.activeSessionId, 'text-surface-400': session.id !== chatStore.activeSessionId }"
        @click="selectSession(session.id)"
      >
        {{ session.title }}
      </button>
      <div v-if="chatStore.sessions.length === 0" class="px-3 py-4 text-xs text-surface-600 text-center">
        No sessions yet
      </div>
    </div>
  </div>
</template>
