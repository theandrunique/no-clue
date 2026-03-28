import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ProviderDescriptor, ProviderSettings } from "@/types/providers";

const SELECTED_PROVIDER_KEY = "selected_provider";

export const useProvidersStore = defineStore("providers", () => {
  const providers = ref<ProviderDescriptor[]>([]);
  const selectedProviderId = ref<string | null>(null);
  const selectedProviderSettings = ref<ProviderSettings | null>(null);
  const loading = ref(false);

  async function loadProviders() {
    loading.value = true;
    try {
      providers.value = await invoke<ProviderDescriptor[]>("get_providers");
    } catch (e) {
      console.error("[ProvidersStore] Failed to load providers:", e);
    } finally {
      loading.value = false;
    }
  }

  async function loadProviderSettings(providerId: string) {
    try {
      selectedProviderSettings.value = await invoke<ProviderSettings>("get_provider_settings", { provider: providerId });
    } catch (e) {
      console.error("[ProvidersStore] Failed to load provider settings:", e);
      selectedProviderSettings.value = null;
    }
  }

  async function saveProviderSettings(providerId: string, settings: ProviderSettings) {
    try {
      await invoke("save_provider_settings", { provider: providerId, settings });
      selectedProviderSettings.value = settings;
    } catch (e) {
      console.error("[ProvidersStore] Failed to save provider settings:", e);
      throw e;
    }
  }

  function setSelectedProvider(providerId: string) {
    console.log("[ProvidersStore] setSelectedProvider called:", providerId);
    selectedProviderId.value = providerId;
    localStorage.setItem(SELECTED_PROVIDER_KEY, providerId);
    console.log("[ProvidersStore] selectedProviderId.value is now:", selectedProviderId.value);
  }

  function loadSelectedProvider() {
    const saved = localStorage.getItem(SELECTED_PROVIDER_KEY);
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
