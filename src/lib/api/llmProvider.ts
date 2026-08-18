import type { ProviderDescriptor } from "$lib/types/providers";
import { invoke } from "@tauri-apps/api/core";

export const llmProviderApi = {
  sendMessage: (request: {
    provider: string;
    conversationId: string;
    userMessage: string;
    captureScreenshot: boolean;
    systemPromptId?: string;
  }) => invoke("send_message", request),

  stopMessageStream: () => invoke("stop_stream"),

  getProviders: () => invoke<ProviderDescriptor[]>("get_ai_providers")
};
