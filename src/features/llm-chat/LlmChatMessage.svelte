<script lang="ts">
  import { RotateCw, Copy } from "@lucide/svelte";
  import type { Message } from "$lib/types";
  import Markdown from "$lib/components/Markdown.svelte";
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import { Button } from "$lib/components/ui";

  let { message, onRetry }: { message: Message; onRetry?: () => void } = $props();

  function errorText(message: Message): string | null {
    if (message.finish_reason?.type === "error") {
      return message.finish_reason.payload.message;
    }
    if (message.finish_reason?.type === "cancelled") {
      return "Generation stopped";
    }
    return null;
  }
</script>

<div>
  {#if message.role === "user"}
    <div class="flex justify-end">
      <div
        class="max-w-[85%] rounded-(--radius) px-3 py-2 text-base whitespace-pre-wrap bg-(--color-primary) text-(text-on-primary)"
      >
        {#if message.content}
          {message.content}
        {/if}
      </div>
    </div>
  {:else}
    <Markdown source={message.content} streaming={false} />
  {/if}

  {#if errorText(message)}
    <div class="flex justify-start mt-2">
      <ErrorMessage error={errorText(message)!} />
    </div>
  {/if}

  {#if message.role === "assistant"}
    <div class="flex items-center justify-start gap-1 mt-2">
      <Button variant="icon">
        <Copy class="size-3.5" />
      </Button>

      <Button variant="icon" onclick={() => onRetry?.()}>
        <RotateCw class="size-3.5" />
      </Button>
    </div>
  {/if}
</div>
