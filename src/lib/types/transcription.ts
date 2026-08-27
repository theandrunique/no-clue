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

export type TranscriptionStatus = "idle" | "starting" | "running" | "stopping";

export type TranscriptionStreamEvent =
  | { type: "status"; payload: { status: TranscriptionStatus } }
  | { type: "result"; payload: { transcript: TranscriptResult } }
  | { type: "error"; payload: { error: string } };
