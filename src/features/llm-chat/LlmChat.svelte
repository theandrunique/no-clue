<script lang="ts">
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import { Button, Loader } from "$lib/components/ui";
  import { useMessages } from "$lib/queries/messages";
  import { modelSettingsStore } from "$services/settings/modelSettings.svelte";
  import { getLlmChatContext } from "$services/llm-chat/llmChatContext";
  import LlmChatInput from "./LlmChatInput.svelte";
  import LlmChatMessage from "./LlmChatMessage.svelte";
  import LlmChatQuickActions from "./LlmChatQuickActions.svelte";
  import ModelBar from "./ModelBar.svelte";

  const ctx = getLlmChatContext();
  const messagesQuery = useMessages(() => ctx.conversationId);

  let listEl: HTMLDivElement;

  $effect(() => {
    void messagesQuery.data;
    if (listEl) listEl.scrollTop = listEl.scrollHeight;
  });
</script>

<div class="flex h-full flex-col gap-2 py-2">
  {#if ctx.error}
    <ErrorMessage error={ctx.error} onClear={() => ctx.clearError()} />
  {/if}

  <div bind:this={listEl} class="mx-auto min-h-0 flex-1 pr-1">
    {#if messagesQuery.isLoading}
      <div class="flex h-full items-center justify-center">
        <Loader />
      </div>
    {:else if messagesQuery.data?.length === 0}
      <div class="flex h-full items-center justify-center text-(--text-muted)">No messages yet</div>
    {:else}
      <div class="flex h-full max-w-200 flex-col gap-8 overflow-y-auto px-8">
        {#each messagesQuery.data ?? [] as message (message.id)}
          <LlmChatMessage {message} />
        {/each}
      </div>
    {/if}
  </div>

  {#if !modelSettingsStore.hasModels && !messagesQuery.isLoading}
    <div class="flex items-center justify-center gap-1 py-4">
      <p class="text-sm">Connect LLM provider to start chatting</p>
      <Button href="/settings">To settings</Button>
    </div>
  {:else}
    <LlmChatQuickActions
      isStreaming={ctx.isStreaming}
      isLoading={messagesQuery.isLoading}
      onSend={(v) => ctx.send(v)}
    />

    <LlmChatInput
      isStreaming={ctx.isStreaming}
      isLoading={messagesQuery.isLoading}
      onSend={(v) => ctx.send(v)}
      onStop={() => ctx.stop()}
    />

    <ModelBar
      models={modelSettingsStore.models}
      model={modelSettingsStore.selectedModel}
      disabled={ctx.isStreaming}
      onModelChange={(m) => modelSettingsStore.setSelectedModel(m)}
    />
  {/if}
</div>
