use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The Master Spec is the single-source-of-truth state object that persists
/// across the entire conversation. It captures goals, constraints, and the
/// current project state so the LLM never hallucinates about what was decided.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterSpec {
    pub global_constraints: GlobalConstraints,
    pub current_state: CurrentState,
    pub active_workspace: ActiveWorkspace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalConstraints {
    pub target_goals: Vec<String>,
    pub blacklisted_approaches: Vec<String>,
    pub user_preferences: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrentState {
    pub project_description: String,
    pub tech_stack: Vec<String>,
    pub completed_milestones: Vec<String>,
    pub active_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveWorkspace {
    /// Sliding window of the last N turns (default 3).
    pub recent_turns: Vec<WorkspaceTurn>,
    pub pending_decisions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceTurn {
    pub role: String,
    pub summary: String,
    pub timestamp: String,
}

impl MasterSpec {
    pub fn new() -> Self {
        Self {
            global_constraints: GlobalConstraints {
                target_goals: Vec::new(),
                blacklisted_approaches: Vec::new(),
                user_preferences: HashMap::new(),
            },
            current_state: CurrentState {
                project_description: String::new(),
                tech_stack: Vec::new(),
                completed_milestones: Vec::new(),
                active_files: Vec::new(),
            },
            active_workspace: ActiveWorkspace {
                recent_turns: Vec::new(),
                pending_decisions: Vec::new(),
            },
        }
    }
}

impl Default for MasterSpec {
    fn default() -> Self {
        Self::new()
    }
}
