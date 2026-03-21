import { defineStore } from "pinia";
import { ref, onScopeDispose } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { Message } from "@/types";

export interface ChunkPayload {
  conversationId: string,
  content: string;
  isFinish: boolean;
  timestamp: number;
}

export interface ErrorPayload {
  code: string;
  message: string;
}

export type ChatStreamEvent =
  | { event_type: 'message:chunk'; payload: ChunkPayload }
  | { event_type: 'message:error'; payload: ErrorPayload };

export const useChatStore = defineStore("chat", () => {
  const messages = ref<Message[]>([]);
  const isStreamingResponse = ref(false);
  const streamingMessage = ref<Message | null>(null);
  const isScreenshotEnabled = ref(true);
  const currentStreamingConversationId = ref<string | null>(null);

  let unlistenStream: UnlistenFn | null = null;

  async function setupListeners() {
    if (unlistenStream) return;

    console.log("[ChatStore] Setting up chat-stream listener");

    unlistenStream = await listen<ChatStreamEvent>("chat-stream", (event) => {
      console.log("[ChatStore] chat-stream event received:", event.payload);
      const event_type = event.payload.event_type;

      if (event_type === "message:chunk") {
        const { payload } = event.payload;

        if (payload.isFinish) {
          console.log("[ChatStore] Stream finished");
          if (streamingMessage.value) {
            messages.value.push(streamingMessage.value);
          }
          isStreamingResponse.value = false;
          streamingMessage.value = null;
        } else {
          console.log("[ChatStore] Chunk received:", payload.content);
          isStreamingResponse.value = true;

          if (!streamingMessage.value) {
            streamingMessage.value = {
              id: crypto.randomUUID(),
              conversationId: payload.conversationId,
              role: "assistant",
              content: "",
              timestamp: Date.now(),
            };
          }
          streamingMessage.value.content += payload.content;
        }
      }
    });

    onScopeDispose(() => {
      unlistenStream?.();
      unlistenStream = null;
    });
  }

  async function sendMessage(conversationId: string, content: string, provider: string) {
    console.log("[ChatStore] sendMessage called:", { conversationId, content, provider });

    currentStreamingConversationId.value = conversationId;
    await setupListeners();

    const newMessage: Message = {
      id: crypto.randomUUID(),
      conversationId,
      role: "user",
      content,
      screenshotPath: undefined,
      timestamp: Date.now(),
    };
    console.log("[ChatStore] Adding user message:", newMessage);
    messages.value.push(newMessage);

    try {
      console.log("[ChatStore] Calling invoke send_message");
      await invoke("send_message", {
        provider,
        conversationId,
        userMessage: content,
        captureScreenshot: isScreenshotEnabled.value,
      });
      console.log("[ChatStore] invoke completed");
    } catch (error) {
      console.error("[ChatStore] Failed to send message:", error);
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
    isScreenshotEnabled.value = capture;
  }

  async function clearMessages() {
    if (isStreamingResponse.value) {
      await stopStream();
    }

    messages.value = [];
    streamingMessage.value = null;
    isStreamingResponse.value = false;
  }

  return {
    messages,
    isStreamingResponse,
    currentStreamingMessage: streamingMessage,
    isScreenshotEnabled,
    setCaptureScreenshot,
    sendMessage,
    stopStream,
    clearMessages,
  };
});
