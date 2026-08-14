import type { ProviderDescriptor } from "$lib/types/providers";
import { invoke } from "@tauri-apps/api/core";

export const providersApi = {
  getLlmProviders: () => invoke<ProviderDescriptor[]>("get_ai_providers"),
  getSttProviders: () => invoke<ProviderDescriptor[]>("get_stt_providers")
};
