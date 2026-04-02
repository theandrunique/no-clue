import { defineStore } from "pinia";
import { ref, onScopeDispose } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";
import type { TranscriptionResult } from "@/types";

const SELECTED_STT_PROVIDER_KEY = "selected_stt_provider";

interface AudioCaptureConfig {
  capture_system_audio: boolean;
  system_audio_device_id: string | null;
  capture_microphone: boolean;
  microphone_device_id: string | null;
}

export const useTranscriptionStore = defineStore("transcription", () => {
  const isEnabled = ref(false);
  const transcripts = ref<TranscriptionResult[]>([]);
  const currentTranscript = ref<TranscriptionResult | null>(null);

  let unlistenResult: UnlistenFn | null = null;
  let unlistenStarted: UnlistenFn | null = null;
  let unlistenStopped: UnlistenFn | null = null;
  let listenersReady = false;

  function getSelectedSttProvider(): string {
    return localStorage.getItem(SELECTED_STT_PROVIDER_KEY) || "fake";
  }

  function buildAudioConfig(): AudioCaptureConfig {
    return {
      capture_system_audio: true,
      system_audio_device_id: null,
      capture_microphone: true,
      microphone_device_id: null,
    };
  }

  async function setupListeners() {
    if (listenersReady) return;
    listenersReady = true;

    unlistenResult = await listen<TranscriptionResult>("transcription-result", (event) => {
      const payload = event.payload;

      if (payload.isFinal) {
        currentTranscript.value = null;
        transcripts.value.push(payload);
      } else {
        currentTranscript.value = payload;
      }
    });

    unlistenStarted = await listen("transcription-started", () => {
      isEnabled.value = true;
    });

    unlistenStopped = await listen("transcription-stopped", () => {
      isEnabled.value = false;
    });

    onScopeDispose(() => {
      unlistenResult?.();
      unlistenStarted?.();
      unlistenStopped?.();
    });
  }

  async function setIsEnabled(value: boolean) {
    await setupListeners();

    try {
      if (value) {
        const providerId = getSelectedSttProvider();
        const audioConfig = buildAudioConfig();

        await invoke("start_transcription", {
          sttProvider: providerId,
          audioConfig,
        });
      } else {
        await invoke("stop_transcription");
      }
    } catch (e) {
      console.error("[TranscriptionStore] Error:", e);
    }
  }

  async function updateSession(conversationId: string) {
    await invoke("update_transcription_session", { conversationId });
  }

  function clearTranscripts() {
    transcripts.value = [];
    currentTranscript.value = null;
  }

  return {
    isEnabled,
    transcripts,
    currentTranscript,
    setIsEnabled,
    updateSession,
    clearTranscripts,
  };
});
