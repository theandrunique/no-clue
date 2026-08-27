import { chatIpc } from "$lib/ipc/chat";
import { transcriptionIpc } from "$lib/ipc/transcription";
import { createQuery } from "@tanstack/svelte-query";

export function useLlmProviders() {
  return createQuery(() => ({
    queryKey: ["providers", "llm"],
    queryFn: chatIpc.getProviders
  }));
}

export function useSttProviders() {
  return createQuery(() => ({
    queryKey: ["providers", "stt"],
    queryFn: transcriptionIpc.getProviders
  }));
}
