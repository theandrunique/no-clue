import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import type { ProviderDescriptor, SttProviderSettings } from "@/types/providers";

const SELECTED_STT_PROVIDER_KEY = "selected_stt_provider";

export const useSttProvidersStore = defineStore("sttProviders", () => {
  const providers = ref<ProviderDescriptor[]>([]);
  const selectedProviderId = ref<string | null>(null);
  const selectedProviderSettings = ref<SttProviderSettings | null>(null);
  const loading = ref(false);

  async function loadProviders() {
    loading.value = true;
    try {
      providers.value = await invoke<ProviderDescriptor[]>("get_stt_providers");
    } catch (e) {
      console.error("[SttProvidersStore] Failed to load providers:", e);
    } finally {
      loading.value = false;
    }
  }

  async function loadProviderSettings() {
    if (!selectedProviderId.value) return;
    try {
      selectedProviderSettings.value = await invoke<SttProviderSettings>("get_stt_provider_settings", { provider: selectedProviderId.value });
    } catch (e) {
      console.error("[SttProvidersStore] Failed to load provider settings:", e);
      selectedProviderSettings.value = null;
    }
  }

  async function saveProviderSettings(settings: SttProviderSettings) {
    if (!selectedProviderId.value) return;
    try {
      await invoke("save_stt_provider_settings", { provider: selectedProviderId.value, settings });
      selectedProviderSettings.value = settings;
    } catch (e) {
      console.error("[SttProvidersStore] Failed to save provider settings:", e);
      throw e;
    }
  }

  function setSelectedProvider(providerId: string) {
    selectedProviderId.value = providerId;
    localStorage.setItem(SELECTED_STT_PROVIDER_KEY, providerId);
  }

  function loadSelectedProvider() {
    const saved = localStorage.getItem(SELECTED_STT_PROVIDER_KEY);
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
