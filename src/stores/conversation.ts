import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useChatStore } from "./chat";
import { useTranscriptionStore } from "./transcription";

export const useConversationStore = defineStore("conversation", () => {
  const currentConversationId = ref<string | null>(null);

  const chatStore = useChatStore();
  const transcriptionStore = useTranscriptionStore();

  const messages = computed(() => chatStore.messages);
  const isStreamingResponse = computed(() => chatStore.isStreamingResponse);
  const streamingMessage = computed(() => chatStore.currentStreamingMessage);
  const isScreenshotEnabled = computed(() => chatStore.isScreenshotEnabled);
  const transcripts = computed(() => transcriptionStore.transcripts);
  const currentTranscript = computed(() => transcriptionStore.currentTranscript);
  const isTranscriptionEnabled = computed(() => transcriptionStore.isEnabled);

  async function createConversation() {
    const conversation = await invoke<{ id: string }>("create_conversation");
    currentConversationId.value = conversation.id;

    await transcriptionStore.updateSession(conversation.id);
  }

  async function ensureConversation() {
    if (!currentConversationId.value) {
      await createConversation();
    }
  }

  async function sendMessage(content: string) {
    await ensureConversation();
    
    // Read from localStorage directly - this works across windows
    const savedProvider = localStorage.getItem("selected_provider");
    const provider = savedProvider || "fake";
    await chatStore.sendMessage(currentConversationId.value!, content, provider);
  }

  async function toggleTranscription() {
    if (!currentConversationId.value) {
      await createConversation();
    }
    await transcriptionStore.setIsEnabled(!transcriptionStore.isEnabled);
  }

  async function clearCurrentConversation() {
    chatStore.clearMessages();
    transcriptionStore.clearTranscripts();

    if (transcriptionStore.isEnabled) {
      await createConversation();
    } else {
      currentConversationId.value = null;
    }
  }

  return {
    currentConversationId,

    messages,
    isStreamingResponse,
    streamingMessage,
    isScreenshotEnabled,
    setCaptureScreenshot: chatStore.setCaptureScreenshot,
    stopStream: chatStore.stopStream,
    sendMessage,

    transcripts,
    currentTranscript,
    isTranscriptionEnabled,
    toggleTranscription,

    clearCurrentConversation,
  };
});
