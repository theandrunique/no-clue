import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ProviderDescriptor, AiProviderSettings } from "@/types/providers";

const SELECTED_AI_PROVIDER_KEY = "selected_ai_provider";

export const useAiProvidersStore = defineStore("aiProviders", () => {
  const providers = ref<ProviderDescriptor[]>([]);
  const selectedProviderId = ref<string | null>(null);
  const selectedProviderSettings = ref<AiProviderSettings | null>(null);
  const loading = ref(false);

  async function loadProviders() {
    loading.value = true;
    try {
      providers.value = await invoke<ProviderDescriptor[]>("get_ai_providers");
    } catch (e) {
      console.error("[AiProvidersStore] Failed to load providers:", e);
    } finally {
      loading.value = false;
    }
  }

  async function loadProviderSettings(providerId: string) {
    if (!selectedProviderId.value) return;
    try {
      selectedProviderSettings.value = await invoke<AiProviderSettings>("get_ai_provider_settings", { provider: providerId });
    } catch (e) {
      console.error("[AiProvidersStore] Failed to load provider settings:", e);
      selectedProviderSettings.value = null;
    }
  }

  async function saveProviderSettings(providerId: string, settings: AiProviderSettings) {
    if (!selectedProviderId.value) return;
    try {
      await invoke("save_ai_provider_settings", { provider: providerId, settings });
      selectedProviderSettings.value = settings;
    } catch (e) {
      console.error("[AiProvidersStore] Failed to save provider settings:", e);
      throw e;
    }
  }

  function setSelectedProvider(providerId: string) {
    selectedProviderId.value = providerId;
    localStorage.setItem(SELECTED_AI_PROVIDER_KEY, providerId);
  }

  function loadSelectedProvider() {
    const saved = localStorage.getItem(SELECTED_AI_PROVIDER_KEY);
    if (saved) {
      selectedProviderId.value = saved;
    }
  }

  function getSelectedProviderId(): string | null {
    return selectedProviderId.value;
  }

  return {
    providers,
    selectedProviderId,
    selectedProviderSettings,
    loading,
    loadProviders,
    loadProviderSettings,
    saveProviderSettings,
    setSelectedProvider,
    loadSelectedProvider,
    getSelectedProviderId,
  };
});
