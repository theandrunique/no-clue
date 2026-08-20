import { chatApi } from "$lib/api/chat";
import { Events, listenEvent } from "$lib/events";
import type { ChatStreamEvent, Message } from "$lib/types";
import { getErrorMessage } from "$lib/utils/errors";
import { providerSettingsStore } from "$services/settings/providerSettings.svelte";
import { activePromptStore } from "$services/system-prompts/activePrompt.svelte";

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
      messages = await chatApi.getMessages(conversationId);
    } catch (e) {
      error = getErrorMessage(e);
    } finally {
      isLoading = false;
    }
  }

  function handleStreamEvent(event: ChatStreamEvent) {
    if (event.type === "finish") {
      if (conversationId && event.payload.conversation_id !== conversationId) return;
      isStreaming = false;
      if (event.payload.finish_reason.type === "error") {
        reloadOnFinish = false;
        const message = event.payload.finish_reason.payload.message || "Stream error";
        void loadMessages().then(() => {
          error = message;
        });
      } else if (reloadOnFinish) {
        reloadOnFinish = false;
        void loadMessages();
      }
      return;
    }

    if (event.type === "start") {
      if (conversationId && event.payload.conversation_id !== conversationId) return;
      return;
    }

    const payload = event.payload;
    if (conversationId && payload.conversation_id !== conversationId) return;

    const last = messages[messages.length - 1];
    if (isStreaming && last && last.role === "assistant") {
      last.content += payload.delta;
    }
  }

  async function init(id: string) {
    conversationId = id;
    await loadMessages();
    if (initialized) return;
    initialized = true;

    await listenEvent(Events.chatStream, handleStreamEvent);
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
      const newMessage = await chatApi.sendMessage({
        provider: lastParams.provider,
        conversationId,
        userMessage: trimmed,
        captureScreenshot: lastParams.captureScreenshot,
        systemPromptId: lastParams.systemPromptId
      });
      userMessage.id = newMessage.id;
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
      await chatApi.retryGeneration({
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
      await chatApi.stopMessageStream();
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
