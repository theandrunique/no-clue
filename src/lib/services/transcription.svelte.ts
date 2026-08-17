import { conversationApi } from "$lib/api/conversation";
import { sttProviderApi } from "$lib/api/sttProvider";
import { audioSettingsStore } from "$lib/stores/audioSettingsStore.svelte";
import { providerSettingsStore } from "$lib/stores/providerSettingsStore.svelte";
import type { TranscriptResult, Transcript } from "$lib/types";
import { getErrorMessage } from "$lib/utils/errors";
import { listen } from "@tauri-apps/api/event";

type TranscriptionStatus = "idle" | "starting" | "listening" | "stopping";

function createTranscriptionService() {
  let status = $state<TranscriptionStatus>("idle");
  let error = $state<string | null>(null);
  let conversationId = $state<string | null>(null);
  let liveResults = $state<TranscriptResult[]>([]);
  let initialized = false;

  function clearError() {
    error = null;
  }

  function handleResult(result: TranscriptResult) {
    if (conversationId && result.conversation_id !== conversationId) return;

    if (!result.is_final) {
      liveResults.push(result);
    } else {
      const index = liveResults.findIndex((r) => r.source === result.source && !r.is_final);
      if (index === -1) {
        liveResults.push(result);
      } else {
        liveResults[index] = result;
      }
    }
  }

  async function loadInitialTranscripts() {
    if (!conversationId) return;
    try {
      const transcripts = await conversationApi.getTranscripts(conversationId);
      liveResults = transcripts.map((t: Transcript) => ({
        id: t.id,
        conversation_id: t.conversation_id,
        source: t.source,
        text: t.text,
        is_final: true,
        confidence: t.confidence,
        created_at: t.created_at
      }));
    } catch (e) {
      error = getErrorMessage(e);
    }
  }

  async function init(id: string) {
    conversationId = id;
    await loadInitialTranscripts();
    if (initialized) return;
    initialized = true;

    await listen<TranscriptResult>("transcription-result", (event) => {
      handleResult(event.payload);
    });
    await listen("transcription-stopped", () => {
      status = "idle";
      clearError();
    });
  }

  async function start() {
    if (status !== "idle") return;
    if (!conversationId) return;

    status = "starting";
    error = null;

    try {
      await sttProviderApi.updateSession(conversationId);
      await sttProviderApi.startTranscription({
        sttProvider: providerSettingsStore.sttProviderId,
        audioConfig: {
          capture_system_audio: audioSettingsStore.capture_system,
          system_audio_device_id: audioSettingsStore.system_device_id,
          capture_microphone: audioSettingsStore.capture_microphone,
          microphone_device_id: audioSettingsStore.microphone_device_id
        }
      });
      status = "listening";
    } catch (e) {
      status = "idle";
      error = getErrorMessage(e);
    }
  }

  async function stop() {
    if (status !== "listening") return;
    status = "stopping";
    try {
      await sttProviderApi.stopTranscription();
    } catch (e) {
      status = "listening";
      error = getErrorMessage(e);
    }
  }

  async function toggle() {
    if (status === "idle") {
      await start();
    } else {
      await stop();
    }
  }

  return {
    get status() {
      return status;
    },
    get isRecording() {
      return status === "starting" || status === "listening" || status === "stopping";
    },
    get error() {
      return error;
    },
    get liveResults() {
      return liveResults;
    },
    clearError,
    init,
    toggle,
    start,
    stop
  };
}

export const transcriptionService = createTranscriptionService();
