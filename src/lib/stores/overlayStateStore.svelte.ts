import { LogicalSize } from "@tauri-apps/api/dpi";
import { getCurrentWindow } from "@tauri-apps/api/window";

const COLLAPSED_HEIGHT = 54;
const EXPANDED_HEIGHT = 500;
const WIDTH = 500;

function createOverlayStateStore() {
  let expanded = $state(false);

  function toggleExpanded() {
    expanded = !expanded;
    resizeWindow();
  }

  async function resizeWindow() {
    const window = getCurrentWindow();
    const height = expanded ? EXPANDED_HEIGHT : COLLAPSED_HEIGHT;
    await window.setSize(new LogicalSize(WIDTH, height));
  }

  return {
    get expanded() {
      return expanded;
    },
    toggleExpanded
  };
}

export const overlayStateStore = createOverlayStateStore();
