import { defineStore } from "pinia";
import { ref, watch } from "vue";

const STORAGE_KEY = "no-clue-settings";

export interface Settings {
  overlayOpacity: number;
  overlayWidth: number;
  overlayHeight: number;
}

const defaultSettings: Settings = {
  overlayOpacity: 80,
  overlayWidth: 600,
  overlayHeight: 500,
};

function loadSettings(): Settings {
  try {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      return { ...defaultSettings, ...JSON.parse(stored) };
    }
  } catch {
    // ignore
  }
  return defaultSettings;
}

export const useSettingsStore = defineStore("settings", () => {
  const settings = ref<Settings>(loadSettings());

  watch(
    settings,
    (newSettings) => {
      localStorage.setItem(STORAGE_KEY, JSON.stringify(newSettings));
    },
    { deep: true },
  );

  function setOverlayOpacity(value: number) {
    settings.value.overlayOpacity = Math.max(0, Math.min(100, value));
  }

  function setOverlayWidth(value: number) {
    settings.value.overlayWidth = Math.max(400, Math.min(1200, value));
  }

  function setOverlayHeight(value: number) {
    settings.value.overlayHeight = Math.max(200, Math.min(800, value));
  }

  return {
    settings,
    setOverlayOpacity,
    setOverlayWidth,
    setOverlayHeight,
  };
});
