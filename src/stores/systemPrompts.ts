import { defineStore } from "pinia";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { type SystemPrompt } from "@/types";

const ACTIVE_PROMPT_KEY = "active_system_prompt_id";

export const useSystemPromptsStore = defineStore("systemPrompts", () => {
  const prompts = ref<SystemPrompt[]>([]);
  const activePromptId = ref<string | null>(null);
  const loading = ref(false);

  function getActivePromptId(): string | null {
    return localStorage.getItem(ACTIVE_PROMPT_KEY);
  }

  async function loadPrompts() {
    loading.value = true;
    try {
      prompts.value = await invoke<SystemPrompt[]>("get_system_prompts");
      activePromptId.value = getActivePromptId();
    } catch (e) {
      console.error("Failed to load prompts:", e);
    } finally {
      loading.value = false;
    }
  }

  async function createPrompt(payload: { name: string; prompt: string }) {
    try {
      const newPrompt = await invoke<SystemPrompt>("create_system_prompt", {
        name: payload.name,
        prompt: payload.prompt,
      });
      prompts.value.unshift(newPrompt);
      return newPrompt;
    } catch (e) {
      console.error("Failed to create prompt:", e);
      throw e;
    }
  }

  async function updatePrompt(id: string, payload: { name: string; prompt: string }) {
    try {
      await invoke("update_system_prompt", {
        id,
        name: payload.name,
        prompt: payload.prompt,
      });
      await loadPrompts();
    } catch (e) {
      console.error("Failed to update prompt:", e);
      throw e;
    }
  }

  async function deletePrompt(id: string) {
    try {
      await invoke("delete_system_prompt", { id });
      prompts.value = prompts.value.filter((p) => p.id !== id);
      if (activePromptId.value === id) {
        activePromptId.value = null;
        localStorage.removeItem(ACTIVE_PROMPT_KEY);
      }
    } catch (e) {
      console.error("Failed to delete prompt:", e);
      throw e;
    }
  }

  function setActive(id: string) {
    activePromptId.value = id;
    localStorage.setItem(ACTIVE_PROMPT_KEY, id);
  }

  return {
    prompts,
    activePromptId,
    loading,
    loadPrompts,
    createPrompt,
    updatePrompt,
    deletePrompt,
    setActive,
  };
});