import type { Llm, LlmRuntimeConfig, LlmSettings, RuntimeSettingsField } from "$lib/types/providers";

function defaultValueFor(field: RuntimeSettingsField): string | number | boolean {
  if (typeof field.default_value !== "undefined") return field.default_value;
  switch (field.field_type.type) {
    case "boolean":
      return false;
    case "number":
      return 0;
    case "select":
      return field.field_type.options[0] ?? "";
  }
}

export function defaultsFor(llm: Llm): LlmRuntimeConfig {
  const out: LlmRuntimeConfig = {};
  for (const field of llm.runtime_config.fields) {
    out[field.key] = defaultValueFor(field);
  }
  return out;
}

export function extractRuntime(model: LlmSettings | null | undefined): LlmRuntimeConfig {
  if (!model) return {};
  switch (model.type) {
    case "Fake":
      return { duration: model.duration };
    case "Qwen3_5Flash":
      return { temperature: model.temperature };
  }
}

export function buildLlmSettings(llm: Llm, overrides?: LlmRuntimeConfig): LlmSettings {
  const merged = { ...defaultsFor(llm), ...overrides };
  switch (llm.id) {
    case "fake":
      return { type: "Fake", duration: String(merged.duration ?? "12s") };
    case "qwen3.5-flash-02-23":
      return {
        type: "Qwen3_5Flash",
        temperature: Number(merged.temperature ?? 0.7)
      };
    default:
      throw new Error(`Unknown LLM model '${llm.id}'`);
  }
}
