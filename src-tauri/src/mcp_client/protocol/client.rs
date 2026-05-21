use crate::error::{AppError, AppResult};
use super::transport::StdioTransport;
use super::types::*;
use std::process::{ChildStdin, ChildStdout};

/// MCP client that communicates with a single MCP server over stdio.
pub struct McpClient {
    transport: StdioTransport,
    server_info: Option<ServerInfo>,
    tools: Vec<ToolDefinition>,
}

impl McpClient {
    pub fn new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        Self {
            transport: StdioTransport::new(stdin, stdout),
            server_info: None,
            tools: Vec::new(),
        }
    }

    /// Perform the MCP initialize handshake.
    pub fn initialize(&mut self) -> AppResult<()> {
        let params = InitializeParams {
            protocol_version: "2024-11-05".to_string(),
            capabilities: ClientCapabilities { tools: Some(serde_json::json!({})) },
            client_info: ClientInfo {
                name: "omni-creator-hub".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
        };

        let id = self.transport.send_request(
            "initialize",
            Some(serde_json::to_value(&params)?),
        )?;

        let response = self.transport.read_response()?;
        if response.id != Some(id) {
            return Err(AppError::Mcp("Response ID mismatch".to_string()));
        }

        if let Some(err) = response.error {
            return Err(AppError::Mcp(format!("Initialize failed: {}", err.message)));
        }

        if let Some(result) = response.result {
            let init: InitializeResult = serde_json::from_value(result)?;
            log::info!("MCP server connected: {} v{}", init.server_info.name, init.server_info.version);
            self.server_info = Some(init.server_info);
        }

        // Send initialized notification (no response expected)
        self.transport.send_request("notifications/initialized", None)?;

        Ok(())
    }

    /// List all tools available from this server.
    pub fn list_tools(&mut self) -> AppResult<Vec<ToolDefinition>> {
        let id = self.transport.send_request("tools/list", None)?;
        let response = self.transport.read_response()?;

        if response.id != Some(id) {
            return Err(AppError::Mcp("Response ID mismatch".to_string()));
        }

        if let Some(err) = response.error {
            return Err(AppError::Mcp(format!("tools/list failed: {}", err.message)));
        }

        if let Some(result) = response.result {
            let tools_value = result["tools"].clone();
            self.tools = serde_json::from_value(tools_value)?;
        }

        Ok(self.tools.clone())
    }

    /// Call a tool by name with the given arguments.
    pub fn call_tool(&mut self, name: &str, arguments: serde_json::Value) -> AppResult<ToolCallResult> {
        let params = serde_json::json!({
            "name": name,
            "arguments": arguments
        });

        let id = self.transport.send_request("tools/call", Some(params))?;
        let response = self.transport.read_response()?;

        if response.id != Some(id) {
            return Err(AppError::Mcp("Response ID mismatch".to_string()));
        }

        if let Some(err) = response.error {
            return Err(AppError::Mcp(format!("tools/call failed: {}", err.message)));
        }

        if let Some(result) = response.result {
            let result: ToolCallResult = serde_json::from_value(result)?;
            return Ok(result);
        }

        Err(AppError::Mcp("Empty response from tools/call".to_string()))
    }

    pub fn server_info(&self) -> Option<&ServerInfo> {
        self.server_info.as_ref()
    }

    pub fn tools(&self) -> &[ToolDefinition] {
        &self.tools
    }
}
