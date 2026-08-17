<script lang="ts">
  import { Button, Input } from "$lib/components/ui";
  import { llmChatService } from "$lib/services/llmChat.svelte";
  import { Send, Square } from "@lucide/svelte";

  let draft = $state("");

  function handleSend() {
    if (llmChatService.isStreaming || !draft.trim()) return;
    llmChatService.send(draft);
    draft = "";
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter" && !event.shiftKey) {
      event.preventDefault();
      handleSend();
    }
  }
</script>

<div class="flex shrink-0 items-center gap-2">
  <Input
    bind:value={draft}
    placeholder="Type a message..."
    disabled={llmChatService.isLoading}
    onkeydown={handleKeydown}
  />

  {#if llmChatService.isStreaming}
    <Button variant="secondary" onclick={() => llmChatService.stop()}>
      <Square class="size-4" />
    </Button>
  {:else}
    <Button onclick={handleSend} disabled={!draft.trim() || llmChatService.isLoading}>
      <Send class="size-4" />
    </Button>
  {/if}
</div>
