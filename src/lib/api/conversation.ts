import type { Conversation } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

export const conversationApi = {
  create: () => invoke<Conversation>("create_conversation")
};
