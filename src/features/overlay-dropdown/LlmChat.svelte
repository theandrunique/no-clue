<script lang="ts">
  import LlmChatMessage from "$lib/components/LlmChatMessage.svelte";
  import { Loader } from "$lib/components/ui";
  import { llmChatService } from "$lib/services/llmChat.svelte";
  import LlmChatInput from "./LlmChatInput.svelte";
  import QuickActions from "./QuickActions.svelte";
  import ErrorMessage from "./ui/ErrorMessage.svelte";

  let listEl: HTMLDivElement;

  $effect(() => {
    void llmChatService.messages;
    if (listEl) listEl.scrollTop = listEl.scrollHeight;
  });
</script>

<div class="flex h-full flex-col gap-2 py-2">
  {#if llmChatService.error}
    <ErrorMessage error={llmChatService.error} onClear={() => llmChatService.clearError()} />
  {/if}

  <div bind:this={listEl} class="flex flex-1 flex-col gap-2 overflow-y-auto pr-1">
    {#if llmChatService.isLoading}
      <div class="flex h-full items-center justify-center">
        <Loader />
      </div>
    {:else if llmChatService.messages.length === 0}
      <div class="flex h-full items-center justify-center text-(--text-muted)">No messages yet</div>
    {:else}
      {#each llmChatService.messages as message (message.id)}
        <LlmChatMessage {message} />
      {/each}
    {/if}
  </div>

  <QuickActions />

  <LlmChatInput />
</div>
