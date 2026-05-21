use super::master_spec::MasterSpec;
use super::query_transformer::TransformedQuery;
use crate::storage::message_repo::MessageRow;
use serde::{Deserialize, Serialize};

/// The assembled context ready to be sent to an LLM provider.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssembledContext {
    pub system_prompt: String,
    pub context_messages: Vec<ContextMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextMessage {
    pub role: String,
    pub content: String,
}

/// Synthesize the final high-density prompt from:
///   Global Constraints + Master Spec snapshot + Active Workspace
pub fn assemble(
    master_spec: &MasterSpec,
    recent_messages: &[MessageRow],
    transformed: &TransformedQuery,
) -> AssembledContext {
    let mut system_parts: Vec<String> = Vec::new();

    // 1. Global constraints
    if !master_spec.global_constraints.target_goals.is_empty() {
        system_parts.push(format!(
            "## Target Goals\n{}",
            master_spec
                .global_constraints
                .target_goals
                .iter()
                .map(|g| format!("- {g}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }

    if !master_spec
        .global_constraints
        .blacklisted_approaches
        .is_empty()
    {
        system_parts.push(format!(
            "## Blacklisted Approaches (DO NOT USE)\n{}",
            master_spec
                .global_constraints
                .blacklisted_approaches
                .iter()
                .map(|b| format!("- {b}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }

    // 2. Current state snapshot
    if !master_spec.current_state.project_description.is_empty() {
        system_parts.push(format!(
            "## Project State\n{}",
            master_spec.current_state.project_description
        ));
    }

    // 3. Active workspace (sliding window)
    let workspace_summary = master_spec.active_workspace.condensed();
    if !workspace_summary.is_empty() {
        system_parts.push(format!("## Recent Context\n{workspace_summary}"));
    }

    let system_prompt = system_parts.join("\n\n");

    // Build context messages from recent DB messages
    let context_messages: Vec<ContextMessage> = recent_messages
        .iter()
        .map(|m| ContextMessage {
            role: m.role.clone(),
            content: m.content.clone(),
        })
        .collect();

    AssembledContext {
        system_prompt,
        context_messages,
    }
}
