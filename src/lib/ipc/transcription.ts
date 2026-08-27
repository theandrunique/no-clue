import type { TranscriptionStatus, AudioCaptureConfig, Transcript } from "$lib/types";
import type { ProviderDescriptor } from "$lib/types/providers";
import { invoke } from "@tauri-apps/api/core";

export const transcriptionIpc = {
  startTranscription: (request: { sttProvider: string; audioConfig: AudioCaptureConfig }) =>
    invoke("start_transcription", request),

  stopTranscription: () => invoke("stop_transcription"),

  updateSession: (conversationId: string) => invoke("update_transcription_session", { conversationId }),

  getCurrentState: () => invoke<TranscriptionStatus>("get_current_state"),

  getTranscripts: (conversationId: string) => invoke<Transcript[]>("get_transcripts", { conversationId }),

  getProviders: () => invoke<ProviderDescriptor[]>("get_stt_providers")
};
