use super::provider_trait::*;
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;

pub struct OpenAiProvider {
    api_key: String,
    client: Client,
    base_url: String,
}

impl OpenAiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url: "https://api.openai.com/v1".to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for OpenAiProvider {
    fn name(&self) -> &str {
        "openai"
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    async fn complete(&self, request: LlmRequest) -> AppResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = build_openai_body(&request, false);

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            return Err(AppError::Llm(format!("OpenAI API error ({status}): {text}")));
        }

        parse_openai_response(&text)
    }

    async fn complete_stream(
        &self,
        request: LlmRequest,
    ) -> AppResult<Box<dyn Stream<Item = AppResult<LlmStreamChunk>> + Unpin>> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = build_openai_body(&request, true);

        let resp = self
            .client
            .post(&url)
            .bearer_auth(&self.api_key)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await?;
            return Err(AppError::Llm(format!("OpenAI stream error ({status}): {text}")));
        }

        let stream = super::streaming::create_sse_stream(resp);
        Ok(Box::new(stream))
    }

    fn available_models(&self) -> Vec<String> {
        vec![
            "gpt-4o".to_string(),
            "gpt-4o-mini".to_string(),
            "gpt-4.1".to_string(),
            "gpt-4.1-mini".to_string(),
        ]
    }
}

/// Public alias for DeepSeek (OpenAI-compatible).
pub fn build_openai_body_compat(request: &LlmRequest, stream: bool) -> serde_json::Value {
    build_openai_body(request, stream)
}

fn build_openai_body(request: &LlmRequest, stream: bool) -> serde_json::Value {
    let messages: Vec<serde_json::Value> = request
        .messages
        .iter()
        .map(|m| {
            serde_json::json!({
                "role": m.role,
                "content": m.content
            })
        })
        .collect();

    let mut body = serde_json::json!({
        "model": request.model,
        "messages": messages,
        "stream": stream
    });

    if let Some(ref tools) = request.tools {
        body["tools"] = serde_json::json!(tools);
    }

    if let Some(temp) = request.temperature {
        body["temperature"] = serde_json::json!(temp);
    }

    if let Some(max) = request.max_tokens {
        body["max_tokens"] = serde_json::json!(max);
    }

    body
}

/// Public alias for DeepSeek (OpenAI-compatible).
pub fn parse_openai_response_compat(text: &str) -> AppResult<LlmResponse> {
    parse_openai_response(text)
}

fn parse_openai_response(text: &str) -> AppResult<LlmResponse> {
    let v: serde_json::Value = serde_json::from_str(text)?;

    let content = v["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let model = v["model"].as_str().unwrap_or("unknown").to_string();

    let usage = TokenUsage {
        prompt_tokens: v["usage"]["prompt_tokens"].as_u64().unwrap_or(0) as u32,
        completion_tokens: v["usage"]["completion_tokens"].as_u64().unwrap_or(0) as u32,
        total_tokens: v["usage"]["total_tokens"].as_u64().unwrap_or(0) as u32,
        cache_hit: false,
    };

    Ok(LlmResponse {
        content,
        model,
        usage,
        tool_calls: None,
    })
}
