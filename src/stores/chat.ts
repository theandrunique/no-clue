import { defineStore } from "pinia";
import { ref, onScopeDispose } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";

export interface Message {
  id: string;
  conversationId: string;
  role: "user" | "assistant" | "system";
  content: string;
  timestamp: number;
}

export interface Conversation {
  id: string;
  title: string;
  createdAt: number;
  updatedAt: number;
}

export const useChatStore = defineStore("chat", () => {
  const messages = ref<Message[]>([]);
  const isStreaming = ref(false);
  const currentStreamingContent = ref("");
  const currentConversationId = ref<string | null>(null);
  const captureScreenshot = ref(true);
  let initialized = false;
  let unlistenStream: UnlistenFn | null = null;

  function init() {
    if (initialized) return;
    initialized = true;

    listen<string>("chat-stream", (event) => {
      if (event.payload === "[DONE]") {
        if (currentStreamingContent.value && currentConversationId.value) {
          addAssistantMessage(currentConversationId.value, currentStreamingContent.value);
        }
        isStreaming.value = false;
        currentStreamingContent.value = "";
      } else {
        isStreaming.value = true;
        currentStreamingContent.value += event.payload;
      }
    }).then((unlisten) => {
      unlistenStream = unlisten;
    });

    onScopeDispose(() => {
      if (unlistenStream) {
        unlistenStream();
        unlistenStream = null;
      }
    });
  }

  function setCurrentConversation(id: string | null) {
    init();
    currentConversationId.value = id;
    clearMessages();
  }

  function addUserMessage(conversationId: string, content: string) {
    init();
    const newMessage: Message = {
      id: crypto.randomUUID(),
      conversationId,
      role: "user",
      content,
      timestamp: Date.now(),
    };
    messages.value.push(newMessage);
    return newMessage;
  }

  function addAssistantMessage(conversationId: string, content: string) {
    init();
    const newMessage: Message = {
      id: crypto.randomUUID(),
      conversationId,
      role: "assistant",
      content,
      timestamp: Date.now(),
    };
    messages.value.push(newMessage);
    return newMessage;
  }

  async function sendMessage(conversationId: string, content: string, provider: string, captureScreenshot: boolean) {
    init();
    addUserMessage(conversationId, content);

    try {
      await invoke("send_message", {
        provider,
        conversationId,
        userMessage: content,
        captureScreenshot,
      });
    } catch (error) {
      console.error("Failed to send message:", error);
    }
  }

  async function stopStream() {
    try {
      await invoke("stop_stream");
    } catch (error) {
      console.error("Failed to stop stream:", error);
    }
  }

  function setCaptureScreenshot(capture: boolean) {
    captureScreenshot.value = capture;
  }

  function clearMessages() {
    messages.value = [];
    currentStreamingContent.value = "";
    isStreaming.value = false;
  }

  return {
    messages,
    isStreaming,
    currentStreamingContent,
    currentConversationId,
    captureScreenshot,
    setCurrentConversation,
    setCaptureScreenshot,
    addUserMessage,
    addAssistantMessage,
    sendMessage,
    stopStream,
    clearMessages,
    init,
  };
});
