use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;

/// Discriminated union of all application events.
/// Each variant carries its own payload and is serialized to the frontend
/// as a tagged JSON object so the Vue event dispatcher can match on `type`.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum AppEvent {
    /// A chunk of streamed LLM text for the chat panel.
    ChatStreamChunk {
        session_id: String,
        chunk: String,
    },

    /// The full LLM response has finished streaming.
    ChatComplete {
        session_id: String,
        message_id: String,
    },

    /// The intent classifier routed the query to a specific model.
    IntentRouted {
        query_type: String,
        selected_model: String,
        reason: String,
    },

    /// A semantic cache hit was found for the query.
    CacheHit {
        query_hash: String,
        cached_response: String,
    },

    /// Semantic cache was updated with a new entry.
    CacheUpdated {
        entry_count: usize,
    },

    /// An MCP server changed health state.
    McpServerStatus {
        server_name: String,
        status: String, // "running", "stopped", "error"
        error_message: Option<String>,
    },

    /// The canvas content was updated (code, markdown, config table, diff).
    CanvasUpdated {
        session_id: String,
        content_type: String,
    },

    /// The background refiner worker updated the Master Spec.
    MasterSpecUpdated {
        session_id: String,
    },

    /// Generic memory state invalidation — frontend should refetch.
    MemoryStateInvalidated {
        reason: String,
    },

    /// Token usage report from a completed LLM call.
    TokenUsageReport {
        session_id: String,
        prompt_tokens: u32,
        completion_tokens: u32,
        total_tokens: u32,
        model: String,
        cache_hit: bool,
    },

    /// A generic error occurred.
    Error {
        message: String,
        source: String,
    },
}

/// Thread-safe event bus backed by a tokio broadcast channel.
/// Supports multiple subscribers — every active listener receives every event.
#[derive(Clone)]
pub struct EventBus {
    sender: Arc<broadcast::Sender<AppEvent>>,
}

impl EventBus {
    /// Create a new bus with a capacity of 1024 events before lagging.
    pub fn new() -> Self {
        let (sender, _) = broadcast::channel(1024);
        Self {
            sender: Arc::new(sender),
        }
    }

    /// Publish an event to all subscribers. Returns Err only if there are zero receivers.
    pub fn publish(&self, event: AppEvent) -> Result<(), broadcast::error::SendError<AppEvent>> {
        self.sender.send(event)?;
        Ok(())
    }

    /// Create a new subscriber handle.
    pub fn subscribe(&self) -> broadcast::Receiver<AppEvent> {
        self.sender.subscribe()
    }

    /// Spawn a tokio task that forwards every bus event to the Tauri frontend
    /// via `app_handle.emit("app-event", &event)`. This is the single bridge
    /// between Rust-side events and the Vue webview.
    pub fn setup_event_forwarding(&self, app_handle: AppHandle) {
        let mut receiver = self.subscribe();
        let handle = app_handle.clone();

        tauri::async_runtime::spawn(async move {
            loop {
                match receiver.recv().await {
                    Ok(event) => {
                        if let Err(e) = handle.emit("app-event", &event) {
                            log::error!("Failed to forward event to frontend: {e}");
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(n)) => {
                        log::warn!("Event bus lagged, {n} events were dropped");
                    }
                    Err(broadcast::error::RecvError::Closed) => {
                        log::info!("Event bus closed, stopping forwarder");
                        break;
                    }
                }
            }
        });
    }
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}
