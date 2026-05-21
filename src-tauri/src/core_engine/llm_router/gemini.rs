use super::provider_trait::*;
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;

pub struct GeminiProvider {
    api_key: String,
    client: Client,
    base_url: String,
}

impl GeminiProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url: "https://generativelanguage.googleapis.com/v1beta".to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for GeminiProvider {
    fn name(&self) -> &str {
        "gemini"
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    async fn complete(&self, request: LlmRequest) -> AppResult<LlmResponse> {
        let url = format!(
            "{}/models/{}:generateContent?key={}",
            self.base_url, request.model, self.api_key
        );

        let body = build_gemini_body(&request, false);
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        let status = resp.status();
        let text = resp.text().await?;
        if !status.is_success() {
            return Err(AppError::Llm(format!("Gemini API error ({status}): {text}")));
        }

        parse_gemini_response(&text, &request.model)
    }

    async fn complete_stream(
        &self,
        request: LlmRequest,
    ) -> AppResult<Box<dyn Stream<Item = AppResult<LlmStreamChunk>> + Unpin>> {
        let url = format!(
            "{}/models/{}:streamGenerateContent?alt=sse&key={}",
            self.base_url, request.model, self.api_key
        );

        let body = build_gemini_body(&request, false);
        let resp = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await?;

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await?;
            return Err(AppError::Llm(format!("Gemini stream error ({status}): {text}")));
        }

        let stream = super::streaming::create_sse_stream(resp);
        Ok(Box::new(stream))
    }

    fn available_models(&self) -> Vec<String> {
        vec![
            "gemini-2.5-pro".to_string(),
            "gemini-2.5-flash".to_string(),
            "gemini-2.0-flash".to_string(),
        ]
    }
}

fn build_gemini_body(request: &LlmRequest, _stream: bool) -> serde_json::Value {
    let contents: Vec<serde_json::Value> = request
        .messages
        .iter()
        .map(|m| {
            serde_json::json!({
                "role": if m.role == "assistant" { "model" } else { "user" },
                "parts": [{ "text": m.content }]
            })
        })
        .collect();

    let mut body = serde_json::json!({ "contents": contents });

    if let Some(ref tools) = request.tools {
        body["tools"] = serde_json::json!([{ "function_declarations": tools }]);
    }

    if let Some(temp) = request.temperature {
        body["generationConfig"] = serde_json::json!({ "temperature": temp });
    }

    body
}

fn parse_gemini_response(text: &str, model: &str) -> AppResult<LlmResponse> {
    let v: serde_json::Value = serde_json::from_str(text)?;

    let content = v["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();

    let usage = TokenUsage {
        prompt_tokens: v["usageMetadata"]["promptTokenCount"]
            .as_u64()
            .unwrap_or(0) as u32,
        completion_tokens: v["usageMetadata"]["candidatesTokenCount"]
            .as_u64()
            .unwrap_or(0) as u32,
        total_tokens: v["usageMetadata"]["totalTokenCount"]
            .as_u64()
            .unwrap_or(0) as u32,
        cache_hit: false,
    };

    Ok(LlmResponse {
        content,
        model: model.to_string(),
        usage,
        tool_calls: None,
    })
}
