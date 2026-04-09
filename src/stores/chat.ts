import { defineStore } from "pinia";
import { ref, computed, onScopeDispose } from "vue";
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

export interface ModelInfo {
  modelName: string;
  contextWindow: number;
  supportsVision: boolean;
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
  const modelInfo = ref<ModelInfo | null>(null);

  const canUseScreenshot = computed(() => modelInfo.value?.supportsVision ?? false);

  let unlistenStream: UnlistenFn | null = null;

  async function setupListeners() {
    if (unlistenStream) return;

    unlistenStream = await listen<ChatStreamEvent>("chat-stream", (event) => {
      const event_type = event.payload.event_type;

      if (event_type === "message:chunk") {
        const { payload } = event.payload;

        if (payload.isFinish) {
          if (streamingMessage.value) {
            messages.value.push(streamingMessage.value);
          }
          isStreamingResponse.value = false;
          streamingMessage.value = null;
        } else {
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
    currentStreamingConversationId.value = conversationId;
    await setupListeners();

    const systemPromptId = localStorage.getItem("active_system_prompt_id");

    const newMessage: Message = {
      id: crypto.randomUUID(),
      conversationId,
      role: "user",
      content,
      screenshotPath: undefined,
      timestamp: Date.now(),
    };
    messages.value.push(newMessage);

    try {
      await invoke("send_message", {
        provider,
        conversationId,
        userMessage: content,
        captureScreenshot: isScreenshotEnabled.value,
        systemPromptId,
      });
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
    if (!canUseScreenshot.value && capture) {
      return;
    }
    isScreenshotEnabled.value = capture;
  }

  async function loadModelInfo() {
    const provider = localStorage.getItem("selected_ai_provider");
    if (!provider) return;

    try {
      const info = await invoke<ModelInfo>("get_model_info", { provider });
      modelInfo.value = info;
      if (!info.supportsVision) {
        isScreenshotEnabled.value = false;
      }
    } catch (error) {
      console.error("Failed to load model info:", error);
    }
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
    canUseScreenshot,
    modelInfo,
    setCaptureScreenshot,
    sendMessage,
    stopStream,
    clearMessages,
    loadModelInfo,
  };
});
