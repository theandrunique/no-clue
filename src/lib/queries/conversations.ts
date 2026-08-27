import { conversationIpc } from "$lib/ipc/conversation";
import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";

export function useConversations() {
  return createQuery(() => ({
    queryKey: ["conversations"],
    queryFn: conversationIpc.list
  }));
}

export function useConversation(id: () => string | null) {
  return createQuery(() => ({
    queryKey: ["conversations", id()],
    queryFn: () => conversationIpc.get(id()!),
    enabled: id() !== null
  }));
}

export function useCreateConversation() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: conversationIpc.create,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["conversations"] });
    }
  }));
}

export function useDeleteConversation() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: conversationIpc.remove,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["conversations"] });
    }
  }));
}
