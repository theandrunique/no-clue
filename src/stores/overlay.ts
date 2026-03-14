import { defineStore } from "pinia";
import { ref } from "vue";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { useSettingsStore } from "./settings";

const COLLAPSED_HEIGHT = 54;

export const useOverlayStore = defineStore("overlay", () => {
  const isExpanded = ref(false);
  const isTranscriptionEnabled = ref(false);
  const captureScreenshot = ref(true);
  const currentConversationId = ref<string | null>(null);

  async function toggleExpanded() {
    isExpanded.value = !isExpanded.value;
    await resizeWindow();
  }

  async function setExpanded(expanded: boolean) {
    isExpanded.value = expanded;
    await resizeWindow();
  }

  async function resizeWindow() {
    const settingsStore = useSettingsStore();
    const window = getCurrentWindow();
    const width = settingsStore.settings.overlayWidth;
    const height = isExpanded.value ? settingsStore.settings.overlayHeight : COLLAPSED_HEIGHT;
    await window.setSize(new LogicalSize(width, height));
  }

  function setTranscriptionEnabled(enabled: boolean) {
    isTranscriptionEnabled.value = enabled;
  }

  function setCaptureScreenshot(capture: boolean) {
    captureScreenshot.value = capture;
  }

  function setCurrentConversationId(id: string | null) {
    currentConversationId.value = id;
  }

  return {
    isExpanded,
    isTranscriptionEnabled,
    captureScreenshot,
    currentConversationId,
    toggleExpanded,
    setExpanded,
    resizeWindow,
    setTranscriptionEnabled,
    setCaptureScreenshot,
    setCurrentConversationId,
  };
});
