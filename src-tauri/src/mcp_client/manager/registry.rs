use crate::error::{AppError, AppResult};
use crate::event_bus::{AppEvent, EventBus};
use crate::mcp_client::manager::process::{ManagedProcess, ServerStatus};
use crate::mcp_client::manager::scanner::McpServerConfig;
use crate::mcp_client::protocol::types::{ToolCallResult, ToolDefinition};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Central registry that manages the lifecycle of all MCP server processes.
/// Shared across the application via `Arc<Mutex<...>>`.
pub struct McpRegistry {
    processes: Arc<Mutex<HashMap<String, ManagedProcess>>>,
    config_path: PathBuf,
    event_bus: EventBus,
}

impl McpRegistry {
    pub fn new(config_path: PathBuf, event_bus: EventBus) -> Self {
        Self {
            processes: Arc::new(Mutex::new(HashMap::new())),
            config_path,
            event_bus,
        }
    }

    /// Load config and spawn all enabled servers.
    pub async fn load_config(&self) -> AppResult<()> {
        let configs = super::scanner::scan_config(&self.config_path)?;
        let mut processes = self.processes.lock().await;

        for config in configs {
            let name = config.name.clone();
            match ManagedProcess::spawn(config) {
                Ok(proc) => {
                    log::info!("MCP server '{}' started", name);
                    let _ = self.event_bus.publish(AppEvent::McpServerStatus {
                        server_name: name.clone(),
                        status: "running".to_string(),
                        error_message: None,
                    });
                    processes.insert(name, proc);
                }
                Err(e) => {
                    log::error!("Failed to start MCP server '{name}': {e}");
                    let _ = self.event_bus.publish(AppEvent::McpServerStatus {
                        server_name: name,
                        status: "error".to_string(),
                        error_message: Some(e.to_string()),
                    });
                }
            }
        }

        Ok(())
    }

    /// Start a specific server by name.
    pub async fn start_server(&self, name: &str) -> AppResult<()> {
        let configs = super::scanner::scan_config(&self.config_path)?;
        let config = configs
            .into_iter()
            .find(|c| c.name == name)
            .ok_or_else(|| AppError::Mcp(format!("Server '{name}' not found in config")))?;

        let proc = ManagedProcess::spawn(config)?;
        let mut processes = self.processes.lock().await;
        processes.insert(name.to_string(), proc);

        let _ = self.event_bus.publish(AppEvent::McpServerStatus {
            server_name: name.to_string(),
            status: "running".to_string(),
            error_message: None,
        });

        Ok(())
    }

    /// Stop a specific server by name.
    pub async fn stop_server(&self, name: &str) -> AppResult<()> {
        let mut processes = self.processes.lock().await;
        if let Some(proc) = processes.get_mut(name) {
            proc.kill()?;
            let _ = self.event_bus.publish(AppEvent::McpServerStatus {
                server_name: name.to_string(),
                status: "stopped".to_string(),
                error_message: None,
            });
        }
        Ok(())
    }

    /// Aggregate tools from all running servers.
    pub async fn list_all_tools(&self) -> AppResult<Vec<ToolDefinition>> {
        let mut processes = self.processes.lock().await;
        let mut all_tools = Vec::new();

        for (name, proc) in processes.iter_mut() {
            if proc.is_alive() {
                match proc.client.list_tools() {
                    Ok(tools) => all_tools.extend(tools),
                    Err(e) => log::error!("Failed to list tools from '{name}': {e}"),
                }
            }
        }

        Ok(all_tools)
    }

    /// Call a tool on a specific server.
    pub async fn call_tool(
        &self,
        server_name: &str,
        tool_name: &str,
        args: serde_json::Value,
    ) -> AppResult<ToolCallResult> {
        let mut processes = self.processes.lock().await;
        let proc = processes
            .get_mut(server_name)
            .ok_or_else(|| AppError::Mcp(format!("Server '{server_name}' not found")))?;

        if !proc.is_alive() {
            return Err(AppError::Mcp(format!("Server '{server_name}' is not running")));
        }

        proc.client.call_tool(tool_name, args)
    }

    /// Check health of all servers, restart dead ones.
    pub async fn health_check(&self) -> AppResult<()> {
        let mut processes = self.processes.lock().await;
        let mut dead_servers = Vec::new();

        for (name, proc) in processes.iter_mut() {
            if !proc.is_alive() {
                log::warn!("MCP server '{name}' died, will restart");
                dead_servers.push(name.clone());
            }
        }

        // Restart dead servers
        drop(processes);
        for name in dead_servers {
            self.start_server(&name).await?;
        }

        Ok(())
    }

    /// Gracefully shutdown all managed processes. Called during app exit.
    pub async fn shutdown_all(&self) {
        let mut processes = self.processes.lock().await;
        for (name, proc) in processes.iter_mut() {
            if proc.is_alive() {
                if let Err(e) = proc.kill() {
                    log::error!("Failed to kill MCP server '{name}': {e}");
                } else {
                    log::info!("MCP server '{name}' shut down");
                }
            }
        }
        processes.clear();
    }
}
