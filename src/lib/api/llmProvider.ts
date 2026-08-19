import type { ProviderDescriptor } from "$lib/types/providers";
import type { SendMessageResult } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

export const llmProviderApi = {
  sendMessage: (request: {
    provider: string;
    conversationId: string;
    userMessage: string;
    captureScreenshot: boolean;
    systemPromptId?: string;
  }) => invoke<SendMessageResult>("send_message", request),

  retryGeneration: (request: {
    provider: string;
    conversationId: string;
    userMessageId: string;
    captureScreenshot: boolean;
    systemPromptId?: string;
  }) => invoke<SendMessageResult>("retry_generation", request),

  stopMessageStream: () => invoke("stop_stream"),

  getProviders: () => invoke<ProviderDescriptor[]>("get_ai_providers")
};
