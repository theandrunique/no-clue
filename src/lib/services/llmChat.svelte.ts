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
  let reloadOnFinish = false;

  let lastParams = {
    provider: "",
    captureScreenshot: false,
    systemPromptId: undefined as string | undefined
  };

  function clearError() {
    error = null;
  }

  async function loadMessages() {
    if (!conversationId) return;
    isLoading = true;
    try {
      messages = await conversationApi.getMessages(conversationId);
    } catch (e) {
      error = getErrorMessage(e);
    } finally {
      isLoading = false;
    }
  }

  function handleStreamEvent(event: ChatStreamEvent) {
    if (event.type === "error") {
      if (conversationId && event.payload.conversation_id !== conversationId) return;
      isStreaming = false;
      reloadOnFinish = false;
      const message = event.payload.message || "Stream error";
      void loadMessages().then(() => {
        error = message;
      });
      return;
    }

    const chunk = event.payload;
    if (conversationId && chunk.conversation_id !== conversationId) return;

    const last = messages[messages.length - 1];
    if (isStreaming && last && last.role === "assistant") {
      last.content += chunk.content;
    }

    if (chunk.is_finish) {
      isStreaming = false;
      if (reloadOnFinish) {
        reloadOnFinish = false;
        void loadMessages();
      }
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
      finish_reason: null,
      created_at: isoNow()
    };

    const assistantPlaceholder: Message = {
      id: `stream-${crypto.randomUUID()}`,
      conversation_id: conversationId,
      role: "assistant",
      content: "",
      screenshot_path: null,
      finish_reason: null,
      created_at: isoNow()
    };

    messages = [...messages, userMessage, assistantPlaceholder];
    isStreaming = true;

    lastParams = {
      provider: providerSettingsStore.llmProviderId,
      captureScreenshot,
      systemPromptId: activePromptStore.activePromptId ?? undefined
    };

    try {
      const result = await llmProviderApi.sendMessage({
        provider: lastParams.provider,
        conversationId,
        userMessage: trimmed,
        captureScreenshot: lastParams.captureScreenshot,
        systemPromptId: lastParams.systemPromptId
      });
      userMessage.id = result.user_message_id;
    } catch (e) {
      isStreaming = false;
      messages = messages.filter((m) => m.id !== assistantPlaceholder.id);
      await loadMessages();
      error = getErrorMessage(e);
    }
  }

  async function retry(userMessageId: string) {
    if (isStreaming || !conversationId) return;

    error = null;

    const assistantPlaceholder: Message = {
      id: `stream-${crypto.randomUUID()}`,
      conversation_id: conversationId,
      role: "assistant",
      content: "",
      screenshot_path: null,
      finish_reason: null,
      created_at: isoNow()
    };

    messages = [...messages, assistantPlaceholder];
    isStreaming = true;
    reloadOnFinish = true;

    try {
      await llmProviderApi.retryGeneration({
        provider: lastParams.provider,
        conversationId,
        userMessageId,
        captureScreenshot: lastParams.captureScreenshot,
        systemPromptId: lastParams.systemPromptId
      });
    } catch (e) {
      reloadOnFinish = false;
      isStreaming = false;
      messages = messages.filter((m) => m.id !== assistantPlaceholder.id);
      await loadMessages();
      error = getErrorMessage(e);
    }
  }

  async function stop() {
    if (!isStreaming) return;
    reloadOnFinish = true;
    try {
      await llmProviderApi.stopMessageStream();
    } catch (e) {
      reloadOnFinish = false;
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
    retry,
    stop,
    toggleCaptureScreenshot
  };
}
