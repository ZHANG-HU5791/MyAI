use crate::error::{AppError, AppResult};
use super::types::{JsonRpcRequest, JsonRpcResponse};
use std::io::{BufRead, BufReader, Read, Write};
use std::process::{ChildStdin, ChildStdout};

/// MCP transport over stdio using LSP-style message framing.
/// Messages are framed as: `Content-Length: N\r\n\r\n{json_payload}`
pub struct StdioTransport {
    stdin: ChildStdin,
    stdout: BufReader<ChildStdout>,
    next_id: u64,
}

impl StdioTransport {
    pub fn new(stdin: ChildStdin, stdout: ChildStdout) -> Self {
        Self {
            stdin,
            stdout: BufReader::new(stdout),
            next_id: 1,
        }
    }

    /// Send a JSON-RPC request with LSP framing.
    pub fn send_request(
        &mut self,
        method: &str,
        params: Option<serde_json::Value>,
    ) -> AppResult<u64> {
        let id = self.next_id;
        self.next_id += 1;

        let request = JsonRpcRequest::new(id, method, params);
        let payload = serde_json::to_string(&request)?;
        let header = format!("Content-Length: {}\r\n\r\n", payload.len());

        self.stdin.write_all(header.as_bytes())?;
        self.stdin.write_all(payload.as_bytes())?;
        self.stdin.flush()?;

        Ok(id)
    }

    /// Read a JSON-RPC response with LSP framing.
    /// Reads headers until blank line, extracts Content-Length, reads exact bytes.
    pub fn read_response(&mut self) -> AppResult<JsonRpcResponse> {
        // Read headers
        let mut content_length: Option<usize> = None;
        loop {
            let mut line = String::new();
            let bytes_read = self.stdout.read_line(&mut line)?;
            if bytes_read == 0 {
                return Err(AppError::Mcp("EOF from MCP server".to_string()));
            }

            let line = line.trim();
            if line.is_empty() {
                break; // End of headers
            }

            if let Some(value) = line.strip_prefix("Content-Length:") {
                content_length = Some(value.trim().parse::<usize>().map_err(|e| {
                    AppError::Mcp(format!("Invalid Content-Length: {e}"))
                })?);
            }
        }

        let length = content_length
            .ok_or_else(|| AppError::Mcp("Missing Content-Length header".to_string()))?;

        // Read exact number of bytes
        let mut buffer = vec![0u8; length];
        self.stdout.read_exact(&mut buffer)?;

        let response: JsonRpcResponse = serde_json::from_slice(&buffer)?;
        Ok(response)
    }
}
