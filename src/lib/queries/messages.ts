import { chatIpc } from "$lib/ipc/chat";
import { createQuery } from "@tanstack/svelte-query";

export function useMessages(conversationId: () => string | null) {
  return createQuery(() => ({
    queryKey: ["messages", conversationId()],
    queryFn: () => chatIpc.getMessages(conversationId()!),
    enabled: conversationId() !== null
  }));
}
