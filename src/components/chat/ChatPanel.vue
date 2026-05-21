<script setup lang="ts">
import { ref } from "vue";
import ChatHistory from "./ChatHistory.vue";
import ChatMessage from "./ChatMessage.vue";
import ChatMessageStream from "./ChatMessageStream.vue";
import ChatInput from "./ChatInput.vue";
import IntentRouterIndicator from "./IntentRouterIndicator.vue";
import { useChatStore } from "@/stores/chat-store";

const chatStore = useChatStore();
const showHistory = ref(false);

async function handleSend(content: string) {
  if (!chatStore.activeSessionId) {
    await chatStore.createSession("New Chat");
  }
  await chatStore.sendMessage(content);
}
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- Header -->
    <div class="flex items-center justify-between px-3 py-2 border-b border-surface-800 shrink-0">
      <div class="flex items-center gap-2">
        <button
          class="button-ghost text-xs"
          @click="showHistory = !showHistory"
        >
          <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-width="2" d="M4 6h16M4 12h16M4 18h16" />
          </svg>
        </button>
        <span class="text-sm font-medium text-surface-300">Chat</span>
      </div>
      <button
        class="button-ghost text-xs"
        @click="chatStore.createSession('New Chat')"
      >
        + New
      </button>
    </div>

    <div class="flex flex-1 overflow-hidden">
      <!-- History sidebar -->
      <ChatHistory
        v-if="showHistory"
        class="w-48 border-r border-surface-800 shrink-0"
        @close="showHistory = false"
      />

      <!-- Messages + Input -->
      <div class="flex flex-col flex-1 overflow-hidden">
        <IntentRouterIndicator v-if="chatStore.intentRouting" />

        <!-- Message list -->
        <div class="flex-1 overflow-y-auto p-4 space-y-4">
          <ChatMessage
            v-for="msg in chatStore.activeMessages"
            :key="msg.id"
            :message="msg"
          />
          <ChatMessageStream v-if="chatStore.isStreaming" />
        </div>

        <!-- Input -->
        <ChatInput @send="handleSend" />
      </div>
    </div>
  </div>
</template>
