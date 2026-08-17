import { invoke } from "@tauri-apps/api/core";

export const overlayApi = {
  start: (conversationId: string) => invoke("start_overlay_session", { conversationId }),
  stop: () => invoke("stop_overlay_session")
};
