import { defineStore } from "pinia";
import { ref } from "vue";
import type { McpServerStatus, McpTool } from "@/types/mcp";
import { useIpc } from "@/composables/useIpc";

export const useMcpStore = defineStore("mcp", () => {
  const servers = ref<McpServerStatus[]>([]);
  const tools = ref<McpTool[]>([]);
  const { call } = useIpc();

  async function loadServers() {
    servers.value = await call<McpServerStatus[]>("list_mcp_servers");
  }

  async function loadTools() {
    tools.value = await call<McpTool[]>("list_mcp_tools");
  }

  async function startServer(name: string) {
    await call("start_mcp_server", { name });
    await loadServers();
  }

  async function stopServer(name: string) {
    await call("stop_mcp_server", { name });
    await loadServers();
  }

  async function reloadConfig() {
    await call("reload_mcp_config");
    await loadServers();
    await loadTools();
  }

  function updateServerStatus(name: string, status: "running" | "stopped" | "error") {
    const idx = servers.value.findIndex((s) => s.name === name);
    if (idx >= 0) {
      servers.value[idx].status = status;
    }
  }

  return {
    servers,
    tools,
    loadServers,
    loadTools,
    startServer,
    stopServer,
    reloadConfig,
    updateServerStatus,
  };
});
