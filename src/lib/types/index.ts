export interface SystemPrompt {
  id: string;
  name: string;
  prompt: string;
  created_at: string;
  updated_at: string;
}

export interface AudioDevice {
  id: string;
  name: string;
  is_default: boolean;
}

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

export interface Conversation {
  id: string;
  title: string;
  created_at: string;
  updated_at: string;
}

export type AudioSource = "system" | "microphone";

export interface Transcript {
  id: string;
  conversation_id: string;
  source: AudioSource;
  text: string;
  confidence: number;
  created_at: string;
}

export interface TranscriptResult {
  id: string;
  conversation_id: string;
  source: AudioSource;
  text: string;
  is_final: boolean;
  confidence: number;
  created_at: string;
}

export interface TokenUsage {
  prompt_tokens: number;
  completion_tokens: number;
  total_tokens: number;
}

export interface ChatStreamStartPayload {
  message_id: string;
  conversation_id: string;
}

export interface ChatStreamChunkPayload {
  message_id: string;
  conversation_id: string;
  delta: string;
}

export interface ChatStreamFinishPayload {
  message_id: string;
  conversation_id: string;
  finish_reason: FinishReason;
  created_at: string;
  usage: TokenUsage | null;
}

export type ChatStreamEvent =
  | { type: "start"; payload: ChatStreamStartPayload }
  | { type: "chunk"; payload: ChatStreamChunkPayload }
  | { type: "finish"; payload: ChatStreamFinishPayload };

export interface AudioCaptureConfig {
  capture_system_audio: boolean;
  system_audio_device_id: string | null;
  capture_microphone: boolean;
  microphone_device_id: string | null;
}
