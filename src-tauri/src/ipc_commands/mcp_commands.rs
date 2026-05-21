use crate::mcp_client::McpRegistry;
use crate::mcp_client::protocol::types::ToolDefinition;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerStatus {
    pub name: String,
    pub status: String,
    pub tools_count: usize,
}

#[tauri::command]
pub async fn list_mcp_servers(
    _registry: tauri::State<'_, McpRegistry>,
) -> Result<Vec<ServerStatus>, String> {
    // Phase 3.4: Full implementation
    Ok(Vec::new())
}

#[tauri::command]
pub async fn start_mcp_server(
    registry: tauri::State<'_, McpRegistry>,
    name: String,
) -> Result<(), String> {
    registry.start_server(&name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn stop_mcp_server(
    registry: tauri::State<'_, McpRegistry>,
    name: String,
) -> Result<(), String> {
    registry.stop_server(&name).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn reload_mcp_config(
    registry: tauri::State<'_, McpRegistry>,
) -> Result<(), String> {
    registry.load_config().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_mcp_tools(
    registry: tauri::State<'_, McpRegistry>,
) -> Result<Vec<ToolDefinition>, String> {
    registry.list_all_tools().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn call_mcp_tool(
    registry: tauri::State<'_, McpRegistry>,
    server: String,
    tool: String,
    args: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let result = registry
        .call_tool(&server, &tool, args)
        .await
        .map_err(|e| e.to_string())?;
    serde_json::to_value(result).map_err(|e| e.to_string())
}
