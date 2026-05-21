use crate::error::{AppError, AppResult};
use super::provider_trait::{LlmStreamChunk, ToolCall};
use futures::stream::Stream;
use reqwest::Response;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;

/// Throttle interval for emitting chunks to the frontend.
/// Targeting ~30 FPS to prevent flooding the webview.
const EMIT_INTERVAL: Duration = Duration::from_millis(33);
/// Buffer size threshold that forces an immediate emit.
const BUFFER_THRESHOLD: usize = 256;

/// Create a throttled SSE stream from a reqwest response.
/// Parses `data: {...}` lines and emits LlmStreamChunk at a controlled rate.
pub fn create_sse_stream(response: Response) -> ThrottledSseStream {
    let (tx, rx) = mpsc::channel(64);

    tokio::spawn(async move {
        use futures::StreamExt;

        let mut byte_stream = response.bytes_stream();
        let mut line_buffer = String::new();
        let mut emit_buffer = String::new();
        let mut last_emit = Instant::now();

        while let Some(chunk_result) = byte_stream.next().await {
            let chunk = match chunk_result {
                Ok(c) => c,
                Err(e) => {
                    let _ = tx.send(Err(AppError::Llm(format!("Stream read error: {e}")))).await;
                    return;
                }
            };

            let text = String::from_utf8_lossy(&chunk);
            line_buffer.push_str(&text);

            // Process complete lines
            while let Some(newline_pos) = line_buffer.find('\n') {
                let line = line_buffer[..newline_pos].trim().to_string();
                line_buffer = line_buffer[newline_pos + 1..].to_string();

                if line.is_empty() {
                    continue;
                }

                if let Some(data) = line.strip_prefix("data: ") {
                    let data = data.trim();
                    if data == "[DONE]" {
                        let _ = tx
                            .send(Ok(LlmStreamChunk {
                                delta: String::new(),
                                finish_reason: Some("stop".to_string()),
                            }))
                            .await;
                        return;
                    }

                    if let Ok(v) = serde_json::from_str::<serde_json::Value>(data) {
                        let delta = extract_delta_text(&v);
                        if !delta.is_empty() {
                            emit_buffer.push_str(&delta);

                            // Throttle: emit if enough time passed or buffer is full
                            let elapsed = last_emit.elapsed();
                            if elapsed >= EMIT_INTERVAL || emit_buffer.len() >= BUFFER_THRESHOLD {
                                let _ = tx
                                    .send(Ok(LlmStreamChunk {
                                        delta: emit_buffer.clone(),
                                        finish_reason: None,
                                    }))
                                    .await;
                                emit_buffer.clear();
                                last_emit = Instant::now();
                            }
                        }
                    }
                }
            }
        }

        // Flush remaining buffer
        if !emit_buffer.is_empty() {
            let _ = tx
                .send(Ok(LlmStreamChunk {
                    delta: emit_buffer,
                    finish_reason: None,
                }))
                .await;
        }
    });

    ThrottledSseStream { receiver: rx }
}

/// Extract the delta text from an SSE JSON payload.
/// Handles both Gemini and OpenAI response formats.
fn extract_delta_text(v: &serde_json::Value) -> String {
    // OpenAI format: choices[0].delta.content
    if let Some(content) = v["choices"][0]["delta"]["content"].as_str() {
        return content.to_string();
    }
    // Gemini format: candidates[0].content.parts[0].text
    if let Some(text) = v["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        return text.to_string();
    }
    String::new()
}

/// A throttled stream of SSE chunks.
pub struct ThrottledSseStream {
    receiver: mpsc::Receiver<AppResult<LlmStreamChunk>>,
}

impl Stream for ThrottledSseStream {
    type Item = AppResult<LlmStreamChunk>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.receiver.poll_recv(cx)
    }
}

/// Collect a full stream into a single string + optional tool calls.
pub async fn collect_stream(
    mut stream: Box<dyn Stream<Item = AppResult<LlmStreamChunk>> + Unpin>,
) -> AppResult<(String, Option<Vec<ToolCall>>)> {
    use futures::StreamExt;

    let mut full_content = String::new();
    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        full_content.push_str(&chunk.delta);
        if chunk.finish_reason.as_deref() == Some("stop") {
            break;
        }
    }
    Ok((full_content, None))
}
