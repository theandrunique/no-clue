import { defineStore } from "pinia";
import { ref, onScopeDispose } from "vue";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface Transcript {
  id: string;
  conversationId: string;
  speaker: "user" | "system";
  text: string;
  isFinal: boolean;
  confidence?: number;
  timestamp: number;
}

export const useTranscriptionStore = defineStore("transcription", () => {
  const isEnabled = ref(false);
  const transcripts = ref<Transcript[]>([]);
  const currentTranscript = ref<Transcript | null>(null);
  const currentConversationId = ref<string | null>(null);
  let initialized = false;
  let unlisten: UnlistenFn | null = null;

  function init() {
    if (initialized) return;
    initialized = true;

    listen<{ text: string; is_final: boolean; speaker: string; confidence?: number }>(
      "transcription-result",
      (event) => {
        if (!currentConversationId.value) return;

        const speaker = event.payload.speaker as "user" | "system";

        if (event.payload.is_final) {
          if (currentTranscript.value) {
            currentTranscript.value = null;
          }
          const newTranscript: Transcript = {
            id: crypto.randomUUID(),
            conversationId: currentConversationId.value,
            speaker,
            text: event.payload.text,
            isFinal: true,
            confidence: event.payload.confidence,
            timestamp: Date.now(),
          };
          transcripts.value.push(newTranscript);
        } else {
          if (!currentTranscript.value) {
            currentTranscript.value = {
              id: crypto.randomUUID(),
              conversationId: currentConversationId.value,
              speaker,
              text: event.payload.text,
              isFinal: false,
              confidence: event.payload.confidence,
              timestamp: Date.now(),
            };
          } else {
            currentTranscript.value.text = event.payload.text;
            currentTranscript.value.confidence = event.payload.confidence;
          }
        }
      }
    ).then((unlistenFn) => {
      unlisten = unlistenFn;
    });

    onScopeDispose(() => {
      if (unlisten) {
        unlisten();
        unlisten = null;
      }
    });
  }

  function setIsEnabled(value: boolean) {
    isEnabled.value = value;
  }

  function setCurrentConversation(id: string | null) {
    init();
    currentConversationId.value = id;
    clearTranscripts();
  }

  function clearTranscripts() {
    transcripts.value = [];
    currentTranscript.value = null;
  }

  return {
    isEnabled,
    setIsEnabled,
    transcripts,
    currentTranscript,
    currentConversationId,
    setCurrentConversation,
    clearTranscripts,
    init,
  };
});
