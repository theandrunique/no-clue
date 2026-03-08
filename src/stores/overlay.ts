import { defineStore } from "pinia";
import { ref } from "vue";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";

const COLLAPSED_HEIGHT = 54;
const EXPANDED_HEIGHT = 300;

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
    const window = getCurrentWindow();
    const height = isExpanded.value ? EXPANDED_HEIGHT : COLLAPSED_HEIGHT;
    await window.setSize(new LogicalSize(600, height));
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
