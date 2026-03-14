import { defineStore } from "pinia";
import { ref } from "vue";

export interface Message {
  id: string;
  conversationId: string;
  role: "user" | "assistant" | "system";
  content: string;
  timestamp: number;
}

export interface Transcript {
  id: string;
  conversationId: string;
  speaker: "user" | "system";
  text: string;
  isFinal: boolean;
  confidence?: number;
  timestamp: number;
}

export interface Conversation {
  id: string;
  title: string;
  createdAt: number;
  updatedAt: number;
}

export const useConversationStore = defineStore("conversation", () => {
  const messages = ref<Message[]>([]);
  const transcripts = ref<Transcript[]>([]);
  const currentConversationId = ref<string | null>(null);
  const isStreaming = ref(false);
  const currentStreamingContent = ref("");

  function addMessage(message: Omit<Message, "id" | "timestamp">) {
    const newMessage: Message = {
      ...message,
      id: crypto.randomUUID(),
      timestamp: Date.now(),
    };
    messages.value.push(newMessage);
    return newMessage;
  }

  function addUserMessage(conversationId: string, content: string) {
    return addMessage({ conversationId, role: "user", content });
  }

  function addAssistantMessage(conversationId: string, content: string) {
    return addMessage({ conversationId, role: "assistant", content });
  }

  function addTranscript(transcript: Omit<Transcript, "id" | "timestamp">) {
    const newTranscript: Transcript = {
      ...transcript,
      id: crypto.randomUUID(),
      timestamp: Date.now(),
    };
    transcripts.value.push(newTranscript);
    return newTranscript;
  }

  function updateTranscript(id: string, text: string, isFinal: boolean) {
    const transcript = transcripts.value.find((t) => t.id === id);
    if (transcript) {
      transcript.text = text;
      transcript.isFinal = isFinal;
    }
  }

  function setCurrentConversation(id: string | null) {
    currentConversationId.value = id;
    if (id) {
      // TODO: Load conversation from backend
    }
  }

  function setStreaming(value: boolean) {
    isStreaming.value = value;
  }

  function setStreamingContent(content: string) {
    currentStreamingContent.value = content;
  }

  function clearMessages() {
    messages.value = [];
  }

  function clearTranscripts() {
    transcripts.value = [];
  }

  function clearAll() {
    clearMessages();
    clearTranscripts();
    currentConversationId.value = null;
  }

  return {
    messages,
    transcripts,
    currentConversationId,
    isStreaming,
    currentStreamingContent,
    addMessage,
    addUserMessage,
    addAssistantMessage,
    addTranscript,
    updateTranscript,
    setCurrentConversation,
    setStreaming,
    setStreamingContent,
    clearMessages,
    clearTranscripts,
    clearAll,
  };
});
