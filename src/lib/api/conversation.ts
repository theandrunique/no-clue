import type { Conversation, Message, Transcript } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

export const conversationApi = {
  create: () => invoke<Conversation>("create_conversation"),

  getMessages: (conversationId: string) => invoke<Message[]>("get_messages", { conversationId }),

  getTranscripts: (conversationId: string) =>
    invoke<Transcript[]>("get_transcripts", { conversationId })
};
