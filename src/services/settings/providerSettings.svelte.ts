import type { LlmSettings } from "$lib/types/providers";

const STORAGE_KEY = "no-clue-provider-settings";

export interface ProviderSettings {
  llm: LlmSettings | null;
  sttProviderId: string;
}

const defaultProviderSettings: ProviderSettings = {
  llm: null,
  sttProviderId: "fake"
};

function loadProviderSettings(): ProviderSettings {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      const parsed = JSON.parse(stored);
      return {
        llm: parsed?.llm ?? null,
        sttProviderId: parsed?.sttProviderId ?? defaultProviderSettings.sttProviderId
      };
    }
    // eslint-disable-next-line no-empty
  } catch {}

  return defaultProviderSettings;
}

function createProviderSettingsStore() {
  const settings = $state(loadProviderSettings());
  save();

  function save() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(settings));
  }

  return {
    get llm() {
      return settings.llm;
    },
    set llm(value: LlmSettings | null) {
      settings.llm = value;
      save();
    },
    get sttProviderId() {
      return settings.sttProviderId;
    },
    set sttProviderId(value: string) {
      settings.sttProviderId = value;
      save();
    }
  };
}

export const providerSettingsStore = createProviderSettingsStore();
