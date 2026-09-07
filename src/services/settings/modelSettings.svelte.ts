import { llmProvidersIpc } from "$lib/ipc/llmProviders";
import type { Llm, LlmSettings } from "$lib/types/providers";
import { getErrorMessage } from "$lib/utils/errors";
import { buildLlmSettings, extractRuntime } from "$services/llm-chat/llmSettings";
import { providerSettingsStore } from "./providerSettings.svelte";

function createModelSettingsStore() {
  let models = $state<Llm[]>([]);
  let error = $state<string | null>(null);

  function resolveSelection() {
    if (models.length === 0) {
      providerSettingsStore.llm = null;
      return;
    }

    const stored = providerSettingsStore.llm;
    const storedModel = stored ? models.find((m) => m.id === stored.type) : undefined;

    const resolved = storedModel ? buildLlmSettings(storedModel, extractRuntime(stored)) : buildLlmSettings(models[0]);

    providerSettingsStore.llm = resolved;
  }

  async function loadModels() {
    try {
      models = await llmProvidersIpc.getAvailableModels();
      resolveSelection();
    } catch (e) {
      error = getErrorMessage(e);
    }
  }

  function setSelectedModel(model: LlmSettings) {
    providerSettingsStore.llm = model;
  }

  return {
    get models() {
      return models;
    },
    get selectedModel() {
      return providerSettingsStore.llm;
    },
    get hasModels() {
      return models.length > 0 && providerSettingsStore.llm !== null;
    },
    get error() {
      return error;
    },
    loadModels,
    setSelectedModel
  };
}

export const modelSettingsStore = createModelSettingsStore();
