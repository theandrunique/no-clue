import { systemPromptsIpc } from "$lib/ipc/systemPrompts";
import { createMutation, createQuery, useQueryClient } from "@tanstack/svelte-query";

export function useSystemPrompts() {
  return createQuery(() => ({
    queryKey: ["system-prompts"],
    queryFn: systemPromptsIpc.list
  }));
}

export function useCreateSystemPrompt() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: systemPromptsIpc.create,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["system-prompts"] });
    }
  }));
}

export function useUpdateSystemPrompt() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: systemPromptsIpc.update,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["system-prompts"] });
    }
  }));
}

export function useDeleteSystemPrompt() {
  const qc = useQueryClient();

  return createMutation(() => ({
    mutationFn: systemPromptsIpc.delete,
    onSettled: () => {
      qc.invalidateQueries({ queryKey: ["system-prompts"] });
    }
  }));
}
