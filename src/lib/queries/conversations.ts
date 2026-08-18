import { conversationApi } from "$lib/api/conversation";
import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";

export function useConversations() {
  return createQuery(() => ({
    queryKey: ["conversations"],
    queryFn: conversationApi.list
  }));
}

export function useConversation(id: () => string | null) {
  return createQuery(() => ({
    queryKey: ["conversations", id()],
    queryFn: () => conversationApi.get(id()!),
    enabled: id() !== null
  }));
}

export function useMessages(id: () => string) {
  return createQuery(() => ({
    queryKey: ["conversations", id(), "messages"],
    queryFn: () => conversationApi.getMessages(id()),
    enabled: !!id()
  }));
}

export function useTranscripts(id: () => string) {
  return createQuery(() => ({
    queryKey: ["conversations", id(), "transcripts"],
    queryFn: () => conversationApi.getTranscripts(id()),
    enabled: !!id()
  }));
}

export function useCreateConversation() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: conversationApi.create,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["conversations"] });
    }
  }));
}

export function useDeleteConversation() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: conversationApi.remove,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["conversations"] });
    }
  }));
}
