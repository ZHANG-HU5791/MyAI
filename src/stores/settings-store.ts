import { defineStore } from "pinia";
import { ref } from "vue";
import { useIpc } from "@/composables/useIpc";

export interface AppSettings {
  default_model: string;
  theme: string;
  mcp_config_path: string;
}

export const useSettingsStore = defineStore("settings", () => {
  const defaultModel = ref("gemini-2.5-flash");
  const theme = ref<"dark" | "light">("dark");
  const mcpConfigPath = ref("");
  const { call } = useIpc();

  async function loadSettings() {
    const settings = await call<AppSettings>("get_app_settings");
    defaultModel.value = settings.default_model;
    theme.value = settings.theme as "dark" | "light";
    mcpConfigPath.value = settings.mcp_config_path;
  }

  async function saveSettings() {
    await call("update_app_settings", {
      settings: {
        default_model: defaultModel.value,
        theme: theme.value,
        mcp_config_path: mcpConfigPath.value,
      },
    });
  }

  async function setApiKey(provider: string, key: string) {
    await call("set_api_key", { provider, key });
  }

  async function deleteApiKey(provider: string) {
    await call("delete_api_key", { provider });
  }

  return {
    defaultModel,
    theme,
    mcpConfigPath,
    loadSettings,
    saveSettings,
    setApiKey,
    deleteApiKey,
  };
});
