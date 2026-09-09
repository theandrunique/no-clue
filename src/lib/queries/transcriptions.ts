import { transcriptionIpc } from "$lib/ipc/transcription";
import { createQuery } from "@tanstack/svelte-query";

export function useTranscripts(conversationId: () => string | null) {
  return createQuery(() => ({
    queryKey: ["transcripts", conversationId()],
    queryFn: () => transcriptionIpc.getTranscripts(conversationId()!),
    enabled: conversationId() !== null
  }));
}
