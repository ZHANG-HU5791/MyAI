<script setup lang="ts">
import { useTokenEconomy } from "@/composables/useTokenEconomy";
import { useMcpStore } from "@/stores/mcp-store";
import { useChatStore } from "@/stores/chat-store";
import { computed } from "vue";

const { totalTokens, cacheHitRate, totalRequests } = useTokenEconomy();
const mcpStore = useMcpStore();
const chatStore = useChatStore();

const runningServers = computed(() =>
  mcpStore.servers.filter((s) => s.status === "running").length
);

const currentModel = computed(() => chatStore.intentRouting?.model ?? "—");
</script>

<template>
  <div
    class="flex items-center justify-between h-6 bg-surface-900 border-t border-surface-800 px-3 text-[10px] text-surface-500 shrink-0"
  >
    <div class="flex items-center gap-4">
      <span>Tokens: {{ totalTokens.toLocaleString() }}</span>
      <span>Cache: {{ cacheHitRate.toFixed(0) }}%</span>
      <span>Requests: {{ totalRequests }}</span>
    </div>
    <div class="flex items-center gap-4">
      <span>MCP: {{ runningServers }}/{{ mcpStore.servers.length }}</span>
      <span>Model: {{ currentModel }}</span>
    </div>
  </div>
</template>
