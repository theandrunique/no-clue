import { chatApi } from "$lib/api/chat";
import { transcriptionApi } from "$lib/api/transcription";
import { createQuery } from "@tanstack/svelte-query";

export function useLlmProviders() {
  return createQuery(() => ({
    queryKey: ["providers", "llm"],
    queryFn: chatApi.getProviders
  }));
}

export function useSttProviders() {
  return createQuery(() => ({
    queryKey: ["providers", "stt"],
    queryFn: transcriptionApi.getProviders
  }));
}
