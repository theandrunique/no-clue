<script lang="ts">
  import type { Snippet } from "svelte";
  import { useQueryClient } from "@tanstack/svelte-query";
  import { chatIpc } from "$lib/ipc/chat";
  import { Events, listenEvent } from "$lib/events";
  import type { ChatStreamEvent } from "$lib/types";
  import { getErrorMessage } from "$lib/utils/errors";
  import { modelSettingsStore } from "$services/settings/modelSettings.svelte";
  import { activePromptStore } from "$services/system-prompts/activePrompt.svelte";
  import { setLlmChatContext, type LlmChatContext } from "./llmChatContext";

  let { children, conversationId }: { children: Snippet; conversationId: string | null } = $props();

  const queryClient = useQueryClient();

  let isStreaming = $state(false);
  let error = $state<string | null>(null);
  let captureScreenshot = $state(false);
  let reloadOnFinish = false;
  let unlisten: (() => void) | null = null;

  function handleStreamEvent(event: ChatStreamEvent) {
    if (event.type === "finish") {
      if (conversationId && event.payload.conversation_id !== conversationId) return;
      isStreaming = false;
      if (event.payload.finish_reason.type === "error") {
        reloadOnFinish = false;
        const message = event.payload.finish_reason.payload.message || "Stream error";
        error = message;
      } else if (reloadOnFinish) {
        reloadOnFinish = false;
      }
      void queryClient.invalidateQueries({ queryKey: ["messages", conversationId] });
      return;
    }

    if (event.type === "start") {
      if (conversationId && event.payload.conversation_id !== conversationId) return;
      return;
    }

    const payload = event.payload;
    if (conversationId && payload.conversation_id !== conversationId) return;
  }

  async function initStream() {
    if (unlisten) return;
    unlisten = await listenEvent(Events.chatStream, handleStreamEvent);
  }

  function cleanup() {
    if (unlisten) {
      unlisten();
      unlisten = null;
    }
  }

  async function send(text: string) {
    const trimmed = text.trim();
    if (isStreaming || !conversationId || !trimmed) return;

    const model = modelSettingsStore.selectedModel;
    if (!model) return;

    error = null;

    isStreaming = true;

    try {
      await chatIpc.sendMessage({
        model,
        conversationId,
        userMessage: trimmed,
        captureScreenshot,
        systemPromptId: activePromptStore.activePromptId ?? undefined
      });
    } catch (e) {
      isStreaming = false;
      error = getErrorMessage(e);
    }
  }

  async function stop() {
    if (!isStreaming || !conversationId) return;
    reloadOnFinish = true;
    try {
      await chatIpc.stopMessageStream(conversationId);
    } catch (e) {
      reloadOnFinish = false;
      error = getErrorMessage(e);
    }
  }

  function clearError() {
    error = null;
  }

  function toggleCaptureScreenshot() {
    captureScreenshot = !captureScreenshot;
  }

  $effect(() => {
    if (conversationId === null) {
      cleanup();
      return;
    }

    modelSettingsStore.loadModels();
    void initStream();

    return () => {
      cleanup();
    };
  });

  const context: LlmChatContext = {
    get conversationId() {
      return conversationId!;
    },
    get isStreaming() {
      return isStreaming;
    },
    get error() {
      return error;
    },
    get captureScreenshot() {
      return captureScreenshot;
    },
    send,
    stop,
    clearError,
    toggleCaptureScreenshot
  };

  setLlmChatContext(context);
</script>

{@render children()}
