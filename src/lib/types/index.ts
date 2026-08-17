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

export interface Message {
  id: string;
  conversation_id: string;
  role: MessageRole;
  content: string;
  screenshot_path: string | null;
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

export interface ChatChunkPayload {
  conversationId: string;
  content: string;
  isFinish: boolean;
  usage: TokenUsage | null;
  timestamp: string;
}

export interface ChatErrorPayload {
  code: string;
  message: string;
}

export type ChatStreamEvent =
  | { event_type: "message:chunk"; payload: ChatChunkPayload }
  | { event_type: "message:error"; payload: ChatErrorPayload };

export interface AudioCaptureConfig {
  capture_system_audio: boolean;
  system_audio_device_id: string | null;
  capture_microphone: boolean;
  microphone_device_id: string | null;
}
