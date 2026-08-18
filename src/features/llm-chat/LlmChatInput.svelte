<script lang="ts">
  import { Button, Input } from "$lib/components/ui";
  import { Send, Square } from "@lucide/svelte";

  let props: {
    isLoading: boolean;
    isStreaming: boolean;
    onSend: (message: string) => void;
    onStop: () => void;
  } = $props();

  let draft = $state("");

  function handleSend() {
    if (props.isStreaming || !draft.trim()) return;
    props.onSend(draft);
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
  <Input bind:value={draft} placeholder="Type a message..." disabled={props.isLoading} onkeydown={handleKeydown} />

  {#if props.isStreaming}
    <Button variant="secondary" onclick={() => props.onStop()}>
      <Square class="size-4" />
    </Button>
  {:else}
    <Button onclick={handleSend} disabled={!draft.trim() || props.isLoading}>
      <Send class="size-4" />
    </Button>
  {/if}
</div>
