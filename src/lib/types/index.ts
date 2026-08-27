export type { SystemPrompt } from "./systemPrompts";
export type { Conversation } from "./conversation";
export type { Message, MessageRole, TokenUsage, FinishReason, ChatStreamEvent } from "./chat";
export type {
  Transcript,
  TranscriptResult,
  TranscriptionStatus,
  AudioSource,
  TranscriptionStreamEvent
} from "./transcription";

export interface AudioDevice {
  id: string;
  name: string;
  is_default: boolean;
}

export interface AudioCaptureConfig {
  capture_system_audio: boolean;
  system_audio_device_id: string | null;
  capture_microphone: boolean;
  microphone_device_id: string | null;
}
