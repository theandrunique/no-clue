import { conversationApi } from "$lib/api/conversation";
import { llmProviderApi } from "$lib/api/llmProvider";
import { activePromptStore } from "$lib/stores/acitvePrompt.svelte";
import { providerSettingsStore } from "$lib/stores/providerSettings.svelte";
import type { ChatStreamEvent, Message } from "$lib/types";
import { getErrorMessage } from "$lib/utils/errors";
import { listen } from "@tauri-apps/api/event";

function isoNow(): string {
  return new Date().toISOString();
}

export function createLlmChatService() {
  let conversationId = $state<string | null>(null);
  let messages = $state<Message[]>([]);
  let isStreaming = $state(false);
  let isLoading = $state(false);
  let error = $state<string | null>(null);
  let captureScreenshot = $state(false);
  let initialized = false;

  function clearError() {
    error = null;
  }

  function handleStreamEvent(event: ChatStreamEvent) {
    if (event.event_type === "message:error") {
      isStreaming = false;
      const last = messages[messages.length - 1];
      if (last && last.role === "assistant" && last.content === "") {
        messages = messages.filter((m) => m.id !== last.id);
      }
      error = event.payload.message || "Stream error";
      return;
    }

    const chunk = event.payload;
    if (conversationId && chunk.conversationId !== conversationId) return;

    const last = messages[messages.length - 1];
    if (isStreaming && last && last.role === "assistant") {
      last.content += chunk.content;
    }

    if (chunk.isFinish) {
      isStreaming = false;
    }
  }

  async function loadMessages() {
    if (!conversationId) return;
    isLoading = true;
    error = null;
    try {
      messages = await conversationApi.getMessages(conversationId);
    } catch (e) {
      error = getErrorMessage(e);
    } finally {
      isLoading = false;
    }
  }

  async function init(id: string) {
    conversationId = id;
    await loadMessages();
    if (initialized) return;
    initialized = true;

    await listen<ChatStreamEvent>("chat-stream", (event) => {
      handleStreamEvent(event.payload);
    });
  }

  async function send(text: string) {
    const trimmed = text.trim();
    if (isStreaming || !conversationId || !trimmed) return;

    error = null;

    const userMessage: Message = {
      id: crypto.randomUUID(),
      conversation_id: conversationId,
      role: "user",
      content: trimmed,
      screenshot_path: null,
      created_at: isoNow()
    };

    const assistantPlaceholder: Message = {
      id: `stream-${crypto.randomUUID()}`,
      conversation_id: conversationId,
      role: "assistant",
      content: "",
      screenshot_path: null,
      created_at: isoNow()
    };

    messages = [...messages, userMessage, assistantPlaceholder];
    isStreaming = true;

    try {
      await llmProviderApi.sendMessage({
        provider: providerSettingsStore.llmProviderId,
        conversationId,
        userMessage: trimmed,
        captureScreenshot,
        systemPromptId: activePromptStore.activePromptId ?? undefined
      });
    } catch (e) {
      isStreaming = false;
      messages = messages.filter((m) => m.id !== assistantPlaceholder.id);
      error = getErrorMessage(e);
    }
  }

  async function stop() {
    if (!isStreaming) return;
    try {
      await llmProviderApi.stopMessageStream();
    } catch (e) {
      error = getErrorMessage(e);
    }
  }

  function toggleCaptureScreenshot() {
    captureScreenshot = !captureScreenshot;
  }

  return {
    get conversationId() {
      return conversationId;
    },
    get messages() {
      return messages;
    },
    get isStreaming() {
      return isStreaming;
    },
    get isLoading() {
      return isLoading;
    },
    get error() {
      return error;
    },
    get captureScreenshot() {
      return captureScreenshot;
    },
    clearError,
    init,
    send,
    stop,
    toggleCaptureScreenshot
  };
}
