import { createContext } from "svelte";

export interface LlmChatContext {
  readonly conversationId: string;
  readonly isStreaming: boolean;
  readonly error: string | null;
  readonly captureScreenshot: boolean;
  send(text: string): void;
  stop(): void;
  clearError(): void;
  toggleCaptureScreenshot(): void;
}

export const [getLlmChatContext, setLlmChatContext] = createContext<LlmChatContext>();
