<script lang="ts">
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import { Loader } from "$lib/components/ui";
  import type { Message } from "$lib/types";
  import LlmChatInput from "./LlmChatInput.svelte";
  import LlmChatMessage from "./LlmChatMessage.svelte";
  import LlmChatQuickActions from "./LlmChatQuickActions.svelte";

  interface LlmChatProps {
    error?: string | null;
    clearError?: () => void;
    isLoading?: boolean;
    messages?: Message[];
    isStreaming: boolean;
    onSend: (message: string) => void;
    onStop: () => void;
    onRetry?: (userMessageId: string) => void;
  }

  let props: LlmChatProps = $props();

  let listEl: HTMLDivElement;

  $effect(() => {
    void props.messages;
    if (listEl) listEl.scrollTop = listEl.scrollHeight;
  });

  function retryHandler(index: number): (() => void) | undefined {
    const msg = props.messages?.[index];
    if (!msg || msg.role !== "assistant" || !props.onRetry) return undefined;

    const retryable = msg.finish_reason?.type === "error" || msg.finish_reason?.type === "cancelled";
    if (!retryable) return undefined;

    for (let i = index - 1; i >= 0; i--) {
      const prev = props.messages?.[i];
      if (prev?.role === "user") {
        const userMessageId = prev.id;
        return () => props.onRetry!(userMessageId);
      }
    }
    return undefined;
  }
</script>

<div class="flex h-full flex-col gap-2 py-2">
  {#if props.error}
    <ErrorMessage error={props.error} onClear={() => props.clearError?.()} />
  {/if}

  <div bind:this={listEl} class="flex flex-1 flex-col gap-2 overflow-y-auto pr-1">
    {#if props.isLoading}
      <div class="flex h-full items-center justify-center">
        <Loader />
      </div>
    {:else if props.messages?.length === 0}
      <div class="flex h-full items-center justify-center text-(--text-muted)">No messages yet</div>
    {:else}
      {#each props.messages as message, i (message.id)}
        <LlmChatMessage {message} onRetry={retryHandler(i)} />
      {/each}
    {/if}
  </div>

  <LlmChatQuickActions
    isStreaming={props.isStreaming}
    isLoading={props.isLoading ?? false}
    onSend={(v) => props.onSend(v)}
  />

  <LlmChatInput
    isStreaming={props.isStreaming}
    isLoading={props.isLoading ?? false}
    onSend={(v) => props.onSend(v)}
    onStop={() => props.onStop()}
  />
</div>
