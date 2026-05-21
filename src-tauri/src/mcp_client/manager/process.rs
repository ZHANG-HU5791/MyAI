use crate::error::{AppError, AppResult};
use crate::mcp_client::protocol::client::McpClient;
use crate::mcp_client::manager::scanner::McpServerConfig;
use std::process::{Child, Command, Stdio};

/// A managed MCP server child process.
pub struct ManagedProcess {
    pub child: Child,
    pub client: McpClient,
    pub config: McpServerConfig,
    pub status: ServerStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerStatus {
    Starting,
    Running,
    Stopped,
    Error(String),
}

impl ManagedProcess {
    /// Spawn an MCP server process and connect via stdio.
    pub fn spawn(config: McpServerConfig) -> AppResult<Self> {
        let mut cmd = Command::new(&config.command);
        cmd.args(&config.args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        for (key, value) in &config.env {
            cmd.env(key, value);
        }

        let mut child = cmd.spawn().map_err(|e| {
            AppError::Mcp(format!(
                "Failed to spawn MCP server '{}': {e}",
                config.name
            ))
        })?;

        let stdin = child.stdin.take().ok_or_else(|| {
            AppError::Mcp(format!("No stdin for MCP server '{}'", config.name))
        })?;
        let stdout = child.stdout.take().ok_or_else(|| {
            AppError::Mcp(format!("No stdout for MCP server '{}'", config.name))
        })?;

        let mut client = McpClient::new(stdin, stdout);
        client.initialize()?;

        Ok(Self {
            child,
            client,
            config,
            status: ServerStatus::Running,
        })
    }

    /// Check if the child process is still alive.
    pub fn is_alive(&mut self) -> bool {
        match self.child.try_wait() {
            Ok(Some(_)) => {
                self.status = ServerStatus::Stopped;
                false
            }
            Ok(None) => true,
            Err(_) => {
                self.status = ServerStatus::Error("Failed to check process status".to_string());
                false
            }
        }
    }

    /// Kill the child process.
    pub fn kill(&mut self) -> AppResult<()> {
        self.child.kill().map_err(|e| {
            AppError::Mcp(format!(
                "Failed to kill MCP server '{}': {e}",
                self.config.name
            ))
        })?;
        self.status = ServerStatus::Stopped;
        Ok(())
    }
}
