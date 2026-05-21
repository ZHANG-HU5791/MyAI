use super::super::memory::query_transformer::IntentType;

/// Routing decision: which provider and model to use for this query.
#[derive(Debug, Clone)]
pub struct RoutingDecision {
    pub provider: String,
    pub model: String,
    pub reason: String,
}

/// Rule-based intent classifier that routes queries to the optimal provider.
pub fn classify(intent: &IntentType, available_providers: &[String]) -> RoutingDecision {
    match intent {
        IntentType::CodeGeneration => {
            let (provider, model) = pick(
                available_providers,
                &[("deepseek", "deepseek-coder"), ("openai", "gpt-4o"), ("gemini", "gemini-2.5-flash")],
            );
            RoutingDecision {
                provider,
                model,
                reason: "Code generation routed to best code model".to_string(),
            }
        }
        IntentType::GeneralChat => {
            let (provider, model) = pick(
                available_providers,
                &[("gemini", "gemini-2.5-flash"), ("openai", "gpt-4o-mini")],
            );
            RoutingDecision {
                provider,
                model,
                reason: "General chat routed to fast/cheap model".to_string(),
            }
        }
        IntentType::DocumentCreation => {
            let (provider, model) = pick(
                available_providers,
                &[("gemini", "gemini-2.5-pro"), ("openai", "gpt-4o")],
            );
            RoutingDecision {
                provider,
                model,
                reason: "Document creation routed to long-form capable model".to_string(),
            }
        }
        IntentType::ComplexReasoning => {
            let (provider, model) = pick(
                available_providers,
                &[("gemini", "gemini-2.5-pro"), ("openai", "gpt-4o"), ("deepseek", "deepseek-reasoner")],
            );
            RoutingDecision {
                provider,
                model,
                reason: "Complex reasoning routed to most capable model".to_string(),
            }
        }
        IntentType::ToolUse => {
            let (provider, model) = pick(
                available_providers,
                &[("openai", "gpt-4o"), ("gemini", "gemini-2.5-flash")],
            );
            RoutingDecision {
                provider,
                model,
                reason: "Tool use routed to function-calling capable model".to_string(),
            }
        }
    }
}

/// Pick the first available provider from the preference list.
fn pick(available: &[String], preferences: &[(&str, &str)]) -> (String, String) {
    for (prov, model) in preferences {
        if available.iter().any(|a| a == prov) {
            return (prov.to_string(), model.to_string());
        }
    }
    // Fallback: first available provider with its default model
    if let Some(first) = available.first() {
        (first.clone(), "default".to_string())
    } else {
        ("gemini".to_string(), "gemini-2.5-flash".to_string())
    }
}
