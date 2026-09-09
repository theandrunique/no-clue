<script lang="ts">
  import type { Snippet } from "svelte";
  import { useQueryClient } from "@tanstack/svelte-query";
  import { Events, listenEvent } from "$lib/events";
  import type { ChatStreamEvent, Message } from "$lib/types";
  import { setLlmChatContext, type LlmChatContext } from "./llmChatContext";

  let { children, conversationId }: { children: Snippet; conversationId: string | null } = $props();

  const queryClient = useQueryClient();

  let isStreaming = $state(false);
  let currentMessage = $state<Message | null>(null);

  function handleStreamEvent(event: ChatStreamEvent) {
    console.log("Chat stream event", event);
    if (conversationId && event.payload.conversation_id !== conversationId) return;

    if (event.type === "start") {
      isStreaming = true;
      currentMessage = {
        conversation_id: event.payload.conversation_id,
        id: event.payload.message_id,
        content: "",
        role: "assistant",
        created_at: new Date().toISOString(),
        finish_reason: null,
        screenshot_path: null
      };
      return;
    } else if (event.type === "chunk") {
      if (currentMessage !== null) {
        currentMessage.content += event.payload.delta;
      } else {
        console.warn("currentMessage was null but delta is recieved");
      }
      return;
    } else if (event.type === "finish") {
      isStreaming = false;
      currentMessage = {
        ...currentMessage!,
        created_at: event.payload.created_at,
        finish_reason: event.payload.finish_reason
      };

      queryClient.invalidateQueries({ queryKey: ["messages", conversationId] }).then(() => {
        currentMessage = null;
      });
      return;
    }
  }

  $effect(() => {
    let unlisten = listenEvent(Events.chatStream, handleStreamEvent);

    return () => {
      unlisten.then((fn) => fn());
    };
  });

  const context: LlmChatContext = {
    get isStreaming() {
      return isStreaming;
    },
    get currentMessage() {
      return currentMessage;
    }
  };

  setLlmChatContext(context);
</script>

{@render children()}
