export interface LlmRequest {
  model: string;
  system_prompt: string;
  messages: LlmMessage[];
  tools?: Record<string, unknown>[];
  max_tokens?: number;
  temperature?: number;
}

export interface LlmMessage {
  role: "user" | "assistant" | "system" | "tool";
  content: string;
  tool_call_id?: string;
}

export interface LlmResponse {
  content: string;
  model: string;
  usage: TokenUsage;
  tool_calls?: ToolCall[];
}

export interface TokenUsage {
  prompt_tokens: number;
  completion_tokens: number;
  total_tokens: number;
  cache_hit: boolean;
}

export interface ToolCall {
  id: string;
  function_name: string;
  arguments: Record<string, unknown>;
}
