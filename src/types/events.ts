/** Discriminated union of all application events from the Rust backend. */
export type AppEvent =
  | ChatStreamChunkEvent
  | ChatCompleteEvent
  | IntentRoutedEvent
  | CacheHitEvent
  | CacheUpdatedEvent
  | McpServerStatusEvent
  | CanvasUpdatedEvent
  | MasterSpecUpdatedEvent
  | MemoryStateInvalidatedEvent
  | TokenUsageReportEvent
  | ErrorEvent;

export interface ChatStreamChunkEvent {
  type: "ChatStreamChunk";
  payload: { session_id: string; chunk: string };
}

export interface ChatCompleteEvent {
  type: "ChatComplete";
  payload: { session_id: string; message_id: string };
}

export interface IntentRoutedEvent {
  type: "IntentRouted";
  payload: { query_type: string; selected_model: string; reason: string };
}

export interface CacheHitEvent {
  type: "CacheHit";
  payload: { query_hash: string; cached_response: string };
}

export interface CacheUpdatedEvent {
  type: "CacheUpdated";
  payload: { entry_count: number };
}

export interface McpServerStatusEvent {
  type: "McpServerStatus";
  payload: {
    server_name: string;
    status: string;
    error_message: string | null;
  };
}

export interface CanvasUpdatedEvent {
  type: "CanvasUpdated";
  payload: { session_id: string; content_type: string };
}

export interface MasterSpecUpdatedEvent {
  type: "MasterSpecUpdated";
  payload: { session_id: string };
}

export interface MemoryStateInvalidatedEvent {
  type: "MemoryStateInvalidated";
  payload: { reason: string };
}

export interface TokenUsageReportEvent {
  type: "TokenUsageReport";
  payload: {
    session_id: string;
    prompt_tokens: number;
    completion_tokens: number;
    total_tokens: number;
    model: string;
    cache_hit: boolean;
  };
}

export interface ErrorEvent {
  type: "Error";
  payload: { message: string; source: string };
}
