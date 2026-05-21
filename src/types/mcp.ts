export interface McpServerConfig {
  name: string;
  command: string;
  args: string[];
  env: Record<string, string>;
  enabled: boolean;
}

export interface McpConfigFile {
  servers: McpServerConfig[];
  version: string;
}

export interface McpServerStatus {
  name: string;
  status: "running" | "stopped" | "error";
  tools_count: number;
}

export interface McpTool {
  name: string;
  description: string;
  input_schema: Record<string, unknown>;
}

export interface McpToolResult {
  content: McpToolContent[];
  is_error?: boolean;
}

export interface McpToolContent {
  type: string;
  text?: string;
}
