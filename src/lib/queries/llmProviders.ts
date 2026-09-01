import { llmProvidersIpc } from "$lib/ipc/llmProviders";
import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";

export function useLlmProviders() {
  return createQuery(() => ({
    queryKey: ["llm-providers"],
    queryFn: llmProvidersIpc.getProviders
  }));
}

export function useRemoveLlmProviderSettings() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: llmProvidersIpc.removeProviderSettings,
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["llm-providers"] });
    }
  }));
}

export function useSaveLlmProviderSettings() {
  const queryClient = useQueryClient();

  return createMutation(() => ({
    mutationFn: llmProvidersIpc.saveProviderSettings,
    onSettled: () => {
      queryClient.invalidateQueries({ queryKey: ["llm-providers"] });
    }
  }));
}
