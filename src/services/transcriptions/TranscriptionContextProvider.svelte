<script lang="ts">
  import { Events, listenEvent } from "$lib/events";
  import { transcriptionIpc } from "$lib/ipc/transcription";
  import type { TranscriptionStatus, TranscriptResult } from "$lib/types";
  import { audioSettingsStore } from "$services/settings/audioSettings.svelte";
  import { providerSettingsStore } from "$services/settings/providerSettings.svelte";
  import type { Snippet } from "svelte";
  import { setTranscriptionContext, type TranscriptionContext } from "./transcriptionContext";
  import { useQueryClient } from "@tanstack/svelte-query";

  let { children, conversationId }: { children: Snippet; conversationId: string | null } = $props();

  const queryClient = useQueryClient();

  let status = $state<TranscriptionStatus>("idle");
  let interimResults = $state<TranscriptResult[]>([]);

  function handleResult(result: TranscriptResult) {
    if (conversationId && result.conversation_id !== conversationId) return;

    if (!result.is_final) {
      const index = interimResults.findIndex((r) => r.source === result.source);
      if (index === -1) {
        interimResults.push(result);
      } else {
        interimResults[index] = result;
      }
    } else {
      const index = interimResults.findIndex((r) => r.source === result.source);
      if (index !== -1) {
        interimResults.splice(index, 1);
      }

      queryClient.cancelQueries({ queryKey: ["transcripts", conversationId] });
      queryClient.invalidateQueries({ queryKey: ["transcripts", conversationId] });
    }
  }

  async function init() {
    await listenEvent(Events.transcriptionStream, (e) => {
      if (e.type === "result") {
        handleResult(e.payload.transcript);
      } else if (e.type === "status") {
        status = e.payload.status;
      } else if (e.type === "error") {
        console.error(e.payload.error);
      }
    });

    status = await transcriptionIpc.getCurrentState();
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
      console.error(e);
    }
  }

  async function stop() {
    try {
      await transcriptionIpc.stopTranscription();
    } catch (e) {
      console.error(e);
    }
  }

  async function toggle() {
    if (status === "starting" || status === "running" || status === "stopping") {
      await stop();
    } else {
      await start();
    }
  }

  init();

  const context: TranscriptionContext = {
    get status() {
      return status;
    },
    get interim() {
      return interimResults;
    },
    toggle
  };

  setTranscriptionContext(context);
</script>

{@render children()}
