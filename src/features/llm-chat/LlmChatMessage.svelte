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
        class="text-(text-on-primary) max-w-[85%] rounded-(--radius) bg-(--color-primary) px-3 py-2 text-base whitespace-pre-wrap"
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
    <div class="mt-2 flex justify-start">
      <ErrorMessage error={errorText(message)!} />
    </div>
  {/if}

  {#if message.role === "assistant"}
    <div class="mt-2 flex items-center justify-start gap-1">
      <Button variant="icon">
        <Copy class="size-3.5" />
      </Button>

      <Button variant="icon" onclick={() => onRetry?.()}>
        <RotateCw class="size-3.5" />
      </Button>
    </div>
  {/if}
</div>
