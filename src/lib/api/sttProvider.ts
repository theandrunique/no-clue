import type { AudioCaptureConfig } from "$lib/types";
import type { ProviderDescriptor } from "$lib/types/providers";
import { invoke } from "@tauri-apps/api/core";

export const sttProviderApi = {
  startTranscription: (request: { sttProvider: string; audioConfig: AudioCaptureConfig }) =>
    invoke("start_transcription", request),

  stopTranscription: () => invoke("stop_transcription"),

  updateSession: (conversationId: string) =>
    invoke("update_transcription_session", { conversationId }),

  getProviders: () => invoke<ProviderDescriptor[]>("get_stt_providers")
};
