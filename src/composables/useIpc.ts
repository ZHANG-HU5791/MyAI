import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { useMemoryStore } from "@/stores/memory-store";
import { useTokenEconomyStore } from "@/stores/token-economy-store";
import { useMcpStore } from "@/stores/mcp-store";
import type { AppEvent } from "@/types/events";
import { onUnmounted } from "vue";

/**
 * Central IPC composable.
 * Wraps Tauri invoke + listen with a global event dispatcher that routes
 * AppEvent types to the correct Pinia store actions.
 */
export function useIpc() {
  const unlisteners: UnlistenFn[] = [];

  /** Call a Tauri IPC command. */
  async function call<T>(command: string, args?: Record<string, unknown>): Promise<T> {
    return invoke<T>(command, args);
  }

  /**
   * Listen for the unified "app-event" channel and dispatch to stores.
   * This is the single bridge between Rust EventBus and Vue reactivity.
   */
  function startGlobalListener() {
    listen<AppEvent>("app-event", (event) => {
      const { type, payload } = event.payload;

      switch (type) {
        case "MasterSpecUpdated":
        case "MemoryStateInvalidated": {
          const memoryStore = useMemoryStore();
          memoryStore.refetch();
          break;
        }

        case "CacheUpdated":
        case "TokenUsageReport": {
          const tokenStore = useTokenEconomyStore();
          if (type === "TokenUsageReport" && "total_tokens" in payload) {
            tokenStore.recordUsage(
              payload.total_tokens,
              payload.model,
              payload.cache_hit
            );
          }
          break;
        }

        case "McpServerStatus": {
          const mcpStore = useMcpStore();
          mcpStore.updateServerStatus(
            payload.server_name,
            payload.status as "running" | "stopped" | "error"
          );
          break;
        }

        case "CacheHit": {
          const tokenStore = useTokenEconomyStore();
          tokenStore.incrementCacheHits();
          break;
        }

        default:
          break;
      }
    }).then((unlisten) => {
      unlisteners.push(unlisten);
    });
  }

  /** Listen for a specific event type with a typed callback. */
  function onEvent<T extends AppEvent>(
    type: T["type"],
    callback: (payload: T["payload"]) => void
  ) {
    listen<AppEvent>("app-event", (event) => {
      if (event.payload.type === type) {
        callback(event.payload.payload as T["payload"]);
      }
    }).then((unlisten) => {
      unlisteners.push(unlisten);
    });
  }

  /** Cleanup all listeners. */
  function cleanup() {
    unlisteners.forEach((fn) => fn());
    unlisteners.length = 0;
  }

  // Auto-cleanup on component unmount
  onUnmounted(cleanup);

  return { call, onEvent, startGlobalListener, cleanup };
}
