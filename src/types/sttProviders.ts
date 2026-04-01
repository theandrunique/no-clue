export interface SttProviderDescriptor {
  id: string;
  label: string;
  fields: SttFieldDescriptor[];
}

export interface SttFieldDescriptor {
  key: string;
  label: string;
  field_type: SttFieldType;
  required: boolean;
  placeholder?: string;
}

export type SttFieldType = 
  | "text"
  | "password"
  | { Select: { options: string[] } };

export function getSttFieldTypeString(fieldType: SttFieldType): string {
  if (typeof fieldType === "string") return fieldType;
  if ("Select" in fieldType) return "select";
  return "text";
}

export type SttProviderConfig = 
  | { type: "Fake" }
  | { type: "Deepgram"; api_key?: string; language?: string; model?: string };

export function getSttFieldValue(settings: SttProviderConfig, key: string): string {
  if ("Deepgram" === settings.type) {
    if (key === "api_key") return settings.api_key || "";
    if (key === "language") return settings.language || "";
    if (key === "model") return settings.model || "";
  }
  return "";
}

export interface AudioCaptureConfig {
  capture_system_audio: boolean;
  system_audio_device_id: string | null;
  capture_microphone: boolean;
  microphone_device_id: string | null;
}