import { type SystemPrompt } from "$lib/types";
import { invoke } from "@tauri-apps/api/core";

export const systemPromptsApi = {
  list: () => invoke<SystemPrompt[]>("get_system_prompts"),

  create: (request: { name: string; prompt: string }) =>
    invoke<SystemPrompt>("create_system_prompt", {
      name: request.name,
      prompt: request.prompt
    }),

  update: (request: { id: string; name: string; prompt: string }) =>
    invoke("update_system_prompt", {
      id: request.id,
      name: request.name,
      prompt: request.prompt
    }),

  delete: (id: string) => invoke("delete_system_prompt", { id })
};
