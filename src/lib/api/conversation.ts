import type { Conversation } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

export const conversationApi = {
  create: () => invoke<Conversation>("create_conversation"),
  list: () => invoke<Conversation[]>("get_conversations"),
  get: (id: string) => invoke<Conversation | null>("get_conversation", { id }),
  remove: (id: string) => invoke<void>("delete_conversation", { id })
};
