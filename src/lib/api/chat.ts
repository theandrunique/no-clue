import type { Message } from "$lib/types";
import type { ProviderDescriptor } from "$lib/types/providers";
import { invoke } from "@tauri-apps/api/core";

export const chatApi = {
  sendMessage: (request: {
    provider: string;
    conversationId: string;
    userMessage: string;
    captureScreenshot: boolean;
    systemPromptId?: string;
  }) => invoke<Message>("send_message", request),

  retryGeneration: (request: {
    provider: string;
    conversationId: string;
    userMessageId: string;
    captureScreenshot: boolean;
    systemPromptId?: string;
  }) => invoke("retry_generation", request),

  stopMessageStream: (conversationId: string) => invoke("stop_generation", { conversationId }),

  getMessages: (conversationId: string) => invoke<Message[]>("get_messages", { conversationId }),

  getProviders: () => invoke<ProviderDescriptor[]>("get_llm_providers")
};
