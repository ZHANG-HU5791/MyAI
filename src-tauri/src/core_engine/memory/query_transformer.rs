use serde::{Deserialize, Serialize};

/// Intent classification for routing to the appropriate LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentType {
    CodeGeneration,
    GeneralChat,
    DocumentCreation,
    ComplexReasoning,
    ToolUse,
}

/// The result of transforming and classifying a user query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformedQuery {
    pub original: String,
    pub optimized: String,
    pub intent: IntentType,
    pub token_estimate: u32,
}

/// Strip redundancy, normalize whitespace, classify intent, estimate tokens.
pub fn transform(input: &str, _context_json: &str) -> TransformedQuery {
    let optimized = input.trim().to_string();
    let intent = classify_intent(&optimized);
    let token_estimate = estimate_tokens(&optimized);

    TransformedQuery {
        original: input.to_string(),
        optimized,
        intent,
        token_estimate,
    }
}

fn classify_intent(text: &str) -> IntentType {
    let lower = text.to_lowercase();
    if lower.contains("write") || lower.contains("code") || lower.contains("implement")
        || lower.contains("function") || lower.contains("class") || lower.contains("struct")
    {
        IntentType::CodeGeneration
    } else if lower.contains("document") || lower.contains("write a") || lower.contains("draft")
    {
        IntentType::DocumentCreation
    } else if lower.contains("think") || lower.contains("analyze") || lower.contains("reason")
        || lower.contains("compare") || lower.contains("evaluate")
    {
        IntentType::ComplexReasoning
    } else if lower.contains("search") || lower.contains("find") || lower.contains("tool")
        || lower.contains("run") || lower.contains("execute")
    {
        IntentType::ToolUse
    } else {
        IntentType::GeneralChat
    }
}

fn estimate_tokens(text: &str) -> u32 {
    ((text.len() as f64) / 4.0).ceil() as u32
}
