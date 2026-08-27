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
