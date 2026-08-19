<script lang="ts">
  import { RefreshCw } from "@lucide/svelte";
  import type { Message } from "$lib/types";

  let { message, onRetry }: { message: Message; onRetry?: () => void } = $props();

  function messageClasses(message: Message) {
    if (message.role === "user") {
      return "self-end bg-(--color-primary) text-(--text-on-primary)";
    }
    if (message.role === "assistant") {
      return "self-start bg-(--button-bg-secondary)";
    }
    return "self-start bg-(--button-bg-secondary) opacity-80";
  }

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

<div
  class={["max-w-[85%] rounded-(--radius) px-3 py-2 text-base whitespace-pre-wrap", messageClasses(message)].join(" ")}
>
  {#if message.content}
    {message.content}
  {/if}

  {#if errorText(message)}
    <div class="mt-2 flex items-center justify-between gap-2 border-t border-(--color-border) pt-2">
      <span class="text-sm font-medium text-(--text-error)">{errorText(message)}</span>
      {#if onRetry}
        <button
          class="flex cursor-pointer items-center gap-1 text-sm text-(--text-muted) hover:text-(--text-foreground)"
          onclick={() => onRetry()}
        >
          <RefreshCw class="size-3.5" />
          Retry
        </button>
      {/if}
    </div>
  {/if}
</div>
