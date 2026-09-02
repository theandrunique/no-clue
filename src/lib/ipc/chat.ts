import type { LlmSettings } from "$lib/types/providers";
import type { Message } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

export const chatIpc = {
  sendMessage: (request: {
    model: LlmSettings;
    conversationId: string;
    userMessage: string;
    captureScreenshot: boolean;
    systemPromptId?: string;
  }) => invoke<Message>("send_message", request),

  retryGeneration: (request: {
    model: LlmSettings;
    conversationId: string;
    userMessageId: string;
    captureScreenshot: boolean;
    systemPromptId?: string;
  }) => invoke("retry_generation", request),

  stopMessageStream: (conversationId: string) => invoke("stop_generation", { conversationId }),

  getMessages: (conversationId: string) => invoke<Message[]>("get_messages", { conversationId })
};
