import { defineStore } from "pinia";
import { ref } from "vue";
import { getCurrentWindow, LogicalSize } from "@tauri-apps/api/window";
import { useSettingsStore } from "./settings";

const COLLAPSED_HEIGHT = 54;

export const useOverlayStore = defineStore("overlay", () => {
  const isExpanded = ref(false);
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

  function setCurrentConversationId(id: string | null) {
    currentConversationId.value = id;
  }

  return {
    isExpanded,
    currentConversationId,
    toggleExpanded,
    setExpanded,
    resizeWindow,
    setCurrentConversationId,
  };
});
