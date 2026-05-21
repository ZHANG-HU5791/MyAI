<script setup lang="ts">
import { onMounted } from "vue";
import TitleBar from "./TitleBar.vue";
import StatusBar from "./StatusBar.vue";
import ChatPanel from "@/components/chat/ChatPanel.vue";
import CanvasPanel from "@/components/canvas/CanvasPanel.vue";
import { useIpc } from "@/composables/useIpc";
import { useStreamingChat } from "@/composables/useStreamingChat";
import { useChatStore } from "@/stores/chat-store";
import { useMemoryStore } from "@/stores/memory-store";
import { useSettingsStore } from "@/stores/settings-store";

const { startGlobalListener } = useIpc();
useStreamingChat();

const chatStore = useChatStore();
const memoryStore = useMemoryStore();
const settingsStore = useSettingsStore();

onMounted(async () => {
  startGlobalListener();
  await chatStore.loadSessions();
  await memoryStore.load();
  await settingsStore.loadSettings();
});
</script>

<template>
  <div class="h-screen flex flex-col bg-surface-950 text-surface-100 select-none">
    <TitleBar />
    <div class="flex flex-1 overflow-hidden">
      <ChatPanel class="w-[45%] min-w-[320px] border-r border-surface-800" />
      <CanvasPanel class="flex-1" />
    </div>
    <StatusBar />
  </div>
</template>
