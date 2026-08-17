import { llmProviderApi } from "$lib/api/llmProvider";
import { sttProviderApi } from "$lib/api/sttProvider";
import { createQuery } from "@tanstack/svelte-query";

export function useLlmProviders() {
  return createQuery(() => ({
    queryKey: ["providers", "llm"],
    queryFn: llmProviderApi.getProviders
  }));
}

export function useSttProviders() {
  return createQuery(() => ({
    queryKey: ["providers", "stt"],
    queryFn: sttProviderApi.getProviders
  }));
}
