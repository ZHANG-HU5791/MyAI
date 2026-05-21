import { invoke } from "@tauri-apps/api/core";
import type { Session, Message } from "@/stores/chat-store";
import type { MasterSpec } from "@/types/master-spec";
import type { McpServerStatus, McpTool } from "@/types/mcp";

/** Typed wrappers for all Tauri IPC commands. */

// Chat
export const createSession = (title: string) =>
  invoke<string>("create_session", { title });

export const listSessions = () =>
  invoke<Session[]>("list_sessions");

export const deleteSession = (sessionId: string) =>
  invoke("delete_session", { sessionId });

export const getMessages = (sessionId: string) =>
  invoke<Message[]>("get_messages", { sessionId });

export const sendMessage = (sessionId: string, content: string) =>
  invoke("send_message", { sessionId, content });

// Memory
export const getMasterSpec = (sessionId?: string) =>
  invoke<MasterSpec>("get_master_spec", { sessionId: sessionId ?? null });

export const updateMasterSpec = (spec: MasterSpec, sessionId?: string) =>
  invoke("update_master_spec", { sessionId: sessionId ?? null, spec });

// MCP
export const listMcpServers = () =>
  invoke<McpServerStatus[]>("list_mcp_servers");

export const listMcpTools = () =>
  invoke<McpTool[]>("list_mcp_tools");

export const startMcpServer = (name: string) =>
  invoke("start_mcp_server", { name });

export const stopMcpServer = (name: string) =>
  invoke("stop_mcp_server", { name });

// Settings
export const getApiKeys = () =>
  invoke<Record<string, string | null>>("get_api_keys");

export const setApiKey = (provider: string, key: string) =>
  invoke("set_api_key", { provider, key });
