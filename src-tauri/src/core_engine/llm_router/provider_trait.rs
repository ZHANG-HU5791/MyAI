use crate::error::AppResult;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use futures::Stream;

/// A unified request to any LLM provider.
#[derive(Debug, Clone)]
pub struct LlmRequest {
    pub model: String,
    pub system_prompt: String,
    pub messages: Vec<LlmMessage>,
    pub tools: Option<Vec<serde_json::Value>>,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone)]
pub struct LlmMessage {
    pub role: String,
    pub content: String,
    pub tool_call_id: Option<String>,
}

/// A completed LLM response.
#[derive(Debug, Clone)]
pub struct LlmResponse {
    pub content: String,
    pub model: String,
    pub usage: TokenUsage,
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// A single chunk of a streaming response.
#[derive(Debug, Clone)]
pub struct LlmStreamChunk {
    pub delta: String,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenUsage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
    pub cache_hit: bool,
}

#[derive(Debug, Clone)]
pub struct ToolCall {
    pub id: String,
    pub function_name: String,
    pub arguments: serde_json::Value,
}

/// Every LLM provider must implement this trait.
#[async_trait]
pub trait LlmProvider: Send + Sync {
    fn name(&self) -> &str;
    fn supports_streaming(&self) -> bool;
    async fn complete(&self, request: LlmRequest) -> AppResult<LlmResponse>;
    async fn complete_stream(
        &self,
        request: LlmRequest,
    ) -> AppResult<Box<dyn Stream<Item = AppResult<LlmStreamChunk>> + Unpin>>;
    fn available_models(&self) -> Vec<String>;
}
