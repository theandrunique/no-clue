import { defineStore } from "pinia";
import { ref, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useChatStore } from "./chat";
import { useTranscriptionStore } from "./transcription";

export interface Conversation {
  id: string;
  title: string;
  createdAt: number;
  updatedAt: number;
}

export const useConversationStore = defineStore("conversation", () => {
  const chatStore = useChatStore();
  const transcriptionStore = useTranscriptionStore();

  const currentConversationId = ref<string | null>(null);
  const isInitialized = ref(false);

  const messages = computed(() => chatStore.messages);
  const isStreaming = computed(() => chatStore.isStreaming);
  const currentStreamingContent = computed(() => chatStore.currentStreamingContent);
  const captureScreenshot = computed(() => chatStore.captureScreenshot);

  const transcripts = computed(() => transcriptionStore.transcripts);
  const currentTranscript = computed(() => transcriptionStore.currentTranscript);
  const isTranscriptionEnabled = computed(() => transcriptionStore.isEnabled);

  async function createConversation(): Promise<string> {
    const conversation = await invoke<{ id: string }>("create_conversation");
    currentConversationId.value = conversation.id;

    chatStore.setCurrentConversation(conversation.id);
    transcriptionStore.setCurrentConversation(conversation.id);

    return conversation.id;
  }

  async function ensureConversation() {
    if (!currentConversationId.value) {
      await createConversation();
    }
  }

  async function sendMessage(content: string) {
    await ensureConversation();

    chatStore.sendMessage(
      currentConversationId.value!,
      content,
      "openrouter",
      chatStore.captureScreenshot
    );
  }

  function stopStream() {
    chatStore.stopStream();
  }

  function setCaptureScreenshot(capture: boolean) {
    chatStore.setCaptureScreenshot(capture);
  }

  async function startTranscription() {
    await ensureConversation();

    await invoke("update_transcription_session", {
      conversationId: currentConversationId.value,
    });

    transcriptionStore.setIsEnabled(true);
    await invoke("start_transcription");
  }

  async function stopTranscription() {
    transcriptionStore.setIsEnabled(false);
    await invoke("stop_transcription");
  }

  async function toggleTranscription() {
    if (isTranscriptionEnabled.value) {
      await stopTranscription();
    } else {
      await startTranscription();
    }
  }

  function clearCurrentConversation() {
    chatStore.clearMessages();
    transcriptionStore.clearTranscripts();
    currentConversationId.value = null;
  }

  return {
    currentConversationId,
    isInitialized,
    messages,
    isStreaming,
    currentStreamingContent,
    captureScreenshot,
    transcripts,
    currentTranscript,
    isTranscriptionEnabled,
    createConversation,
    ensureConversation,
    sendMessage,
    stopStream,
    setCaptureScreenshot,
    startTranscription,
    stopTranscription,
    toggleTranscription,
    clearCurrentConversation,
  };
});
