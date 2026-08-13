import { systemPromptsApi } from "$lib/api/systemPrompts";
import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";

export function useSystemPrompts() {
  return createQuery(() => ({
    queryKey: ["system-prompts"],
    queryFn: systemPromptsApi.list
  }));
}

export function useCreateSystemPrompt() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: systemPromptsApi.create,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["system-prompts"] });
    }
  }));
}

export function useUpdateSystemPrompt() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: systemPromptsApi.update,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["system-prompts"] });
    }
  }));
}

export function useDeleteSystemPrompt() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: systemPromptsApi.delete,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["system-prompts"] });
    }
  }));
}
