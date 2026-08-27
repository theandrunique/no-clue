export type MessageRole = "user" | "assistant" | "system";

export type FinishReason = { type: "done" } | { type: "cancelled" } | { type: "error"; payload: { message: string } };

export interface Message {
  id: string;
  conversation_id: string;
  role: MessageRole;
  content: string;
  screenshot_path: string | null;
  finish_reason: FinishReason | null;
  created_at: string;
}

export type ChatStreamEvent =
  | { type: "start"; payload: { conversation_id: string; message_id: string } }
  | { type: "chunk"; payload: { conversation_id: string; message_id: string; delta: string } }
  | {
      type: "finish";
      payload: {
        conversation_id: string;
        message_id: string;
        finish_reason: FinishReason;
        created_at: string;
        usage: TokenUsage | null;
      };
    };

export interface TokenUsage {
  prompt_tokens: number;
  completion_tokens: number;
  total_tokens: number;
}
