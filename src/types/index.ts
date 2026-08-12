export interface Transcript {
  id: string;
  conversation_id: string;
  source: "microphone" | "system";
  text: string;
  confidence: number;
  timestamp: number;
}

export interface TranscriptionResult {
  id: string;
  conversation_id: string;
  source: "microphone" | "system";
  text: string;
  is_final: boolean;
  confidence: number;
  timestamp: number;
}

export interface Message {
  id: string;
  conversation_id: string;
  role: "user" | "assistant" | "system";
  content: string;
  screenshot_path?: string;
  timestamp: number;
}

export interface Conversation {
  id: string;
  title: string;
  created_at: number;
  updated_at: number;
}

export interface SystemPrompt {
  id: string;
  name: string;
  prompt: string;
  created_at: number;
  updated_at: number;
}
