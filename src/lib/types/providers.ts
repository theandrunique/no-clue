export interface Llm {
  id: string;
  display_name: string;
  provider_id: string;
  provider_display_name: string;
  capabilities: LlmCapabilities;
  runtime_config: ModelRuntimeSettingsSchema;
}

export interface LlmCapabilities {
  context_window: number;
  supports_vision: boolean;
  supports_reasoning: boolean;
}

export interface LlmProvider {
  id: string;
  display_name: string;
  settings: LlmProviderSettings | null;
  settings_schema: ProviderSettingsSchema;
}

export interface ProviderSettingsSchema {
  fields: ProviderSettingsField[];
}

export interface ProviderSettingsField {
  key: string;
  display_name: string;
  field_type: FieldType;
  required: boolean;
  placeholder: string | null;
}

export type FieldType = { type: "text" } | { type: "password" } | { type: "select"; options: string[] };

export interface ModelRuntimeSettingsSchema {
  fields: RuntimeSettingsField[];
}

export interface RuntimeSettingsField {
  key: string;
  display_name: string;
  field_type: RuntimeFieldType;
  default_value?: string | number | boolean;
}

export type RuntimeFieldType = { type: "boolean" } | { type: "number" } | { type: "select"; options: string[] };

export type LlmProviderSettings = { type: "TestingProvider" } | { type: "AiTunnel"; api_key: string };
export type SttProviderSettings = { type: "TestingProvider" } | { type: "Deepgram"; api_key: string };

export type LlmSettings = { type: "Fake"; duration: string } | { type: "Qwen3_5Flash"; temperature: number };

export type LlmRuntimeConfig = Record<string, string | number | boolean>;
