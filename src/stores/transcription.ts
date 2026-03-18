import { defineStore } from "pinia";
import { ref, onScopeDispose } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

export interface TranscriptionResult {
  id: string;
  conversationId: string;
  speaker: "user" | "system";
  text: string;
  isFinal: boolean;
  confidence: number;
  timestamp: number;
}

export const useTranscriptionStore = defineStore("transcription", () => {
  const isEnabled = ref(false);
  const transcripts = ref<TranscriptionResult[]>([]);
  const currentTranscript = ref<TranscriptionResult | null>(null);

  let unlistenResult: UnlistenFn | null = null;
  let unlistenStarted: UnlistenFn | null = null;
  let unlistenStopped: UnlistenFn | null = null;
  let listenersReady = false;

  async function setupListeners() {
    if (listenersReady) return;
    listenersReady = true;

    console.log("[TranscriptionStore] Setting up listeners");

    unlistenResult = await listen<TranscriptionResult>("transcription-result", (event) => {
      console.log("[TranscriptionStore] transcription-result received:", event.payload);
      const payload = event.payload;

      if (payload.isFinal) {
        currentTranscript.value = null;
        transcripts.value.push(payload);
      } else {
        currentTranscript.value = payload;
      }
    });

    unlistenStarted = await listen("transcription-started", () => {
      console.log("[TranscriptionStore] transcription-started received");
      isEnabled.value = true;
    });

    unlistenStopped = await listen("transcription-stopped", () => {
      console.log("[TranscriptionStore] transcription-stopped received");
      isEnabled.value = false;
    });

    onScopeDispose(() => {
      unlistenResult?.();
      unlistenStarted?.();
      unlistenStopped?.();
    });
  }

  async function setIsEnabled(value: boolean) {
    console.log("[TranscriptionStore] setIsEnabled called:", value, "current:", isEnabled.value);
    
    await setupListeners();
    console.log("[TranscriptionStore] Listeners ready");

    try {
      if (value) {
        console.log("[TranscriptionStore] Calling start_transcription");
        await invoke("start_transcription");
      } else {
        console.log("[TranscriptionStore] Calling stop_transcription");
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
