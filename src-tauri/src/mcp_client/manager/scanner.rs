use crate::error::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for a single MCP server read from mcp_config.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
    #[serde(default = "default_enabled")]
    pub enabled: bool,
}

fn default_enabled() -> bool {
    true
}

/// The top-level config file structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfigFile {
    pub servers: Vec<McpServerConfig>,
    #[serde(default)]
    pub version: String,
}

/// Read and parse the MCP configuration file.
pub fn scan_config(path: &PathBuf) -> AppResult<Vec<McpServerConfig>> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| AppError::Config(format!("Failed to read mcp_config.json: {e}")))?;

    let config: McpConfigFile = serde_json::from_str(&content)?;
    Ok(config.servers.into_iter().filter(|s| s.enabled).collect())
}
