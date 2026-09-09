import { chatIpc } from "$lib/ipc/chat";
import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";

export function useMessages(conversationId: () => string | null) {
  return createQuery(() => ({
    queryKey: ["messages", conversationId()],
    queryFn: () => chatIpc.getMessages(conversationId()!),
    enabled: conversationId() !== null
  }));
}

export function useSendMessage() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: chatIpc.sendMessage,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["messages"] });
    }
  }));
}

export function useStopMessageStream() {
  return createMutation(() => ({
    mutationFn: chatIpc.stopMessageStream
  }));
}
