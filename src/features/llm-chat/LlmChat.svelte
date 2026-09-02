<script lang="ts">
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import { Button, Loader } from "$lib/components/ui";
  import type { Message } from "$lib/types";
  import type { Llm, LlmSettings } from "$lib/types/providers";
  import LlmChatInput from "./LlmChatInput.svelte";
  import LlmChatMessage from "./LlmChatMessage.svelte";
  import LlmChatQuickActions from "./LlmChatQuickActions.svelte";
  import ModelBar from "./ModelBar.svelte";

  interface LlmChatProps {
    error?: string | null;
    clearError?: () => void;
    isLoading?: boolean;
    messages?: Message[];
    isStreaming: boolean;
    models: Llm[];
    selectedModel: LlmSettings | null;
    hasModels: boolean;
    onModelChange: (model: LlmSettings) => void;
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

  <div bind:this={listEl} class="mx-auto min-h-0 flex-1 pr-1">
    {#if props.isLoading}
      <div class="flex h-full items-center justify-center">
        <Loader />
      </div>
    {:else if props.messages?.length === 0}
      <div class="flex h-full items-center justify-center text-(--text-muted)">No messages yet</div>
    {:else}
      <div class="flex h-full max-w-200 flex-col gap-8 overflow-y-auto px-8">
        {#each props.messages as message, i (message.id)}
          <LlmChatMessage {message} onRetry={retryHandler(i)} />
        {/each}
      </div>
    {/if}
  </div>

  {#if !props.hasModels && !props.isLoading}
    <div class="flex items-center justify-center gap-1 py-4">
      <p class="text-sm">Connect LLM provider to start chatting</p>
      <Button href="/settings">To settings</Button>
    </div>
  {:else}
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

    <ModelBar
      models={props.models}
      model={props.selectedModel}
      disabled={props.isStreaming}
      onModelChange={props.onModelChange}
    />
  {/if}
</div>
