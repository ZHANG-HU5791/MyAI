use super::master_spec::{ActiveWorkspace, WorkspaceTurn};
use chrono::Utc;

const MAX_TURNS: usize = 3;

impl ActiveWorkspace {
    /// Add a new turn, evicting the oldest if over the limit.
    pub fn add_turn(&mut self, role: &str, summary: &str) {
        self.recent_turns.push(WorkspaceTurn {
            role: role.to_string(),
            summary: summary.to_string(),
            timestamp: Utc::now().to_rfc3339(),
        });
        if self.recent_turns.len() > MAX_TURNS {
            self.recent_turns.remove(0);
        }
    }

    /// Return a condensed string of recent turns for prompt injection.
    pub fn condensed(&self) -> String {
        self.recent_turns
            .iter()
            .map(|t| format!("[{}] {}: {}", t.timestamp, t.role, t.summary))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
