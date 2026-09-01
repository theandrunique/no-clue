import type { Llm, LlmProviderSettings, LlmProvider } from "$lib/types/providers";
import { invoke } from "@tauri-apps/api/core";

export const llmProvidersIpc = {
  getProviders: () => invoke<LlmProvider[]>("get_llm_providers"),

  saveProviderSettings: (request: { provider_id: string; settings: LlmProviderSettings }) =>
    invoke("save_llm_provider_settings", request),

  removeProviderSettings: (providerId: string) => invoke("remove_llm_provider_settings", { providerId }),

  getAvailableModels: () => invoke<Llm[]>("get_available_llms")
};
