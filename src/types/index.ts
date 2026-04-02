export interface Transcript {
  id: string;
  conversationId: string;
  source: "microphone" | "system";
  text: string;
  confidence: number;
  timestamp: number;
}

export interface TranscriptionResult {
  id: string;
  conversationId: string;
  source: "microphone" | "system";
  text: string;
  isFinal: boolean;
  confidence: number;
  timestamp: number;
}

export interface Message {
  id: string;
  conversationId: string;
  role: "user" | "assistant" | "system";
  content: string;
  screenshotPath?: string;
  timestamp: number;
}

export interface Conversation {
  id: string;
  title: string;
  createdAt: number;
  updatedAt: number;
}
