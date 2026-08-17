const STORAGE_KEY = "no-clue-provider-settings";

export interface ProviderSettings {
  llmProviderId: string;
  sttProviderId: string;
}

const defaultProviderSettings: ProviderSettings = {
  llmProviderId: "fake",
  sttProviderId: "fake"
};

function loadProviderSettings(): ProviderSettings {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return { ...defaultProviderSettings, ...JSON.parse(stored) };
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
    get llmProviderId() {
      return settings.llmProviderId;
    },
    set llmProviderId(value: string) {
      settings.llmProviderId = value;
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
