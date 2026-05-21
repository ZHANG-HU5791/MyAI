use super::provider_trait::*;
use crate::error::{AppError, AppResult};
use async_trait::async_trait;
use futures::Stream;
use reqwest::Client;

/// DeepSeek uses an OpenAI-compatible API, so the request/response format is identical.
pub struct DeepSeekProvider {
    api_key: String,
    client: Client,
    base_url: String,
}

impl DeepSeekProvider {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
            base_url: "https://api.deepseek.com/v1".to_string(),
        }
    }
}

#[async_trait]
impl LlmProvider for DeepSeekProvider {
    fn name(&self) -> &str {
        "deepseek"
    }

    fn supports_streaming(&self) -> bool {
        true
    }

    async fn complete(&self, request: LlmRequest) -> AppResult<LlmResponse> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = super::openai::build_openai_body_compat(&request, false);

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
            return Err(AppError::Llm(format!("DeepSeek API error ({status}): {text}")));
        }

        super::openai::parse_openai_response_compat(&text)
    }

    async fn complete_stream(
        &self,
        request: LlmRequest,
    ) -> AppResult<Box<dyn Stream<Item = AppResult<LlmStreamChunk>> + Unpin>> {
        let url = format!("{}/chat/completions", self.base_url);
        let body = super::openai::build_openai_body_compat(&request, true);

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
            return Err(AppError::Llm(format!("DeepSeek stream error ({status}): {text}")));
        }

        let stream = super::streaming::create_sse_stream(resp);
        Ok(Box::new(stream))
    }

    fn available_models(&self) -> Vec<String> {
        vec![
            "deepseek-chat".to_string(),
            "deepseek-coder".to_string(),
            "deepseek-reasoner".to_string(),
        ]
    }
}
