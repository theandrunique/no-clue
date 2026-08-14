import { providersApi } from "$lib/api/providers";
import { createQuery } from "@tanstack/svelte-query";

export function useLlmProviders() {
  return createQuery(() => ({
    queryKey: ["providers", "llm"],
    queryFn: providersApi.getLlmProviders
  }));
}

export function useSttProviders() {
  return createQuery(() => ({
    queryKey: ["providers", "stt"],
    queryFn: providersApi.getSttProviders
  }));
}
