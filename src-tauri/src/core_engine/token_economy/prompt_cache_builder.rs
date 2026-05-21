use crate::core_engine::memory::context_assembler::AssembledContext;
use serde::Serialize;

/// A cacheable prompt split into static (cacheable) and dynamic (per-request) parts.
/// Structuring the prompt this way enables API-level prompt caching.
#[derive(Debug, Clone, Serialize)]
pub struct CacheablePrompt {
    pub static_prefix: String,
    pub dynamic_suffix: String,
}

impl CacheablePrompt {
    pub fn full_prompt(&self) -> String {
        format!("{}\n{}", self.static_prefix, self.dynamic_suffix)
    }
}

/// Build a deterministic, cacheable system prompt.
/// Static prefix = global constraints + tool definitions (identical across requests).
/// Dynamic suffix = workspace state + recent messages + query (varies per request).
pub fn build(
    system_prompt: &str,
    tool_defs: &[serde_json::Value],
    context: &AssembledContext,
    user_query: &str,
) -> CacheablePrompt {
    let static_prefix = format!(
        "{}\n\n# Available Tools\n{}\n",
        system_prompt,
        serde_json::to_string_pretty(tool_defs).unwrap_or_default()
    );

    let conversation = context
        .context_messages
        .iter()
        .map(|m| format!("{}: {}", m.role, m.content))
        .collect::<Vec<_>>()
        .join("\n");

    let dynamic_suffix = format!(
        "# Conversation\n{}\n\n# User Query\n{}\n",
        conversation, user_query
    );

    CacheablePrompt {
        static_prefix,
        dynamic_suffix,
    }
}
