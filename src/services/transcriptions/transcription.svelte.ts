import { transcriptionIpc } from "$lib/ipc/transcription";
import { Events, listenEvent } from "$lib/events";
import type { TranscriptResult, Transcript, TranscriptionStatus } from "$lib/types";
import { getErrorMessage } from "$lib/utils/errors";
import { audioSettingsStore } from "$services/settings/audioSettings.svelte";
import { providerSettingsStore } from "$services/settings/providerSettings.svelte";

export function createTranscriptionService() {
  let status = $state<TranscriptionStatus>("idle");
  let error = $state<string | null>(null);
  let conversationId = $state<string | null>(null);
  let liveResults = $state<TranscriptResult[]>([]);

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
      const transcripts = await transcriptionIpc.getTranscripts(conversationId);
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

    await listenEvent(Events.transcriptionResult, handleResult);
    await listenEvent(Events.transcriptionStatus, (s) => {
      status = s;
    });
    await listenEvent(Events.transcriptionError, (e) => {
      console.error(e);
      error = e;
    });

    try {
      status = await transcriptionIpc.getCurrentState();
    } catch (e) {
      error = getErrorMessage(e);
    }
  }

  async function start() {
    if (!conversationId) return;

    try {
      await transcriptionIpc.updateSession(conversationId);
      await transcriptionIpc.startTranscription({
        sttProvider: providerSettingsStore.sttProviderId,
        audioConfig: {
          capture_system_audio: audioSettingsStore.capture_system,
          system_audio_device_id: audioSettingsStore.system_device_id,
          capture_microphone: audioSettingsStore.capture_microphone,
          microphone_device_id: audioSettingsStore.microphone_device_id
        }
      });
    } catch (e) {
      error = getErrorMessage(e);
    }
  }

  async function stop() {
    try {
      await transcriptionIpc.stopTranscription();
    } catch (e) {
      error = getErrorMessage(e);
    }
  }

  async function toggle() {
    if (status === "starting" || status === "running" || status === "stopping") {
      await stop();
    } else {
      await start();
    }
  }

  return {
    get status() {
      return status;
    },
    get isRecording() {
      return status === "starting" || status === "running" || status === "stopping";
    },
    get error() {
      return error;
    },
    get liveResults() {
      return liveResults;
    },
    clearError,
    init,
    start,
    stop,
    toggle
  };
}
