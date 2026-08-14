export interface ProviderDescriptor {
  id: string;
  label: string;
  fields: FieldDescriptor[];
}

export interface FieldDescriptor {
  key: string;
  label: string;
  field_type: FieldType;
  required: boolean;
  placeholder?: string;
}

export type FieldType = "text" | "password" | { Select: { options: string[] } };

export function getFieldTypeString(fieldType: FieldType): string {
  if (typeof fieldType === "string") return fieldType;
  if ("Select" in fieldType) return "select";
  return "text";
}

export type LlmProviderSettings =
  | { type: "Fake" }
  | { type: "Ollama"; base_url?: string; model: string }
  | { type: "AiTunnel"; api_key: string; model: string };

export type SttProviderSettings =
  { type: "Fake" } | { type: "Deepgram"; api_key?: string; language?: string; model?: string };

export function getFieldValue(
  settings: LlmProviderSettings | SttProviderSettings,
  key: string
): string {
  if ("Ollama" === settings.type) {
    if (key === "base_url") return settings.base_url || "";
    if (key === "model") return settings.model || "";
  }
  if ("AiTunnel" === settings.type) {
    if (key === "api_key") return settings.api_key || "";
    if (key === "model") return settings.model || "";
  }
  if ("Deepgram" === settings.type) {
    if (key === "api_key") return settings.api_key || "";
    if (key === "language") return settings.language || "";
    if (key === "model") return settings.model || "";
  }
  return "";
}
