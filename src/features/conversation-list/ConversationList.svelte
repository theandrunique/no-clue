<script lang="ts">
  import { Button, Loader } from "$lib/components/ui";
  import { useConversations } from "$lib/queries/conversations";
  import { cn } from "$lib/utils";
  import { Trash2 } from "@lucide/svelte";

  let {
    selectedId,
    onSelect,
    onDelete
  }: {
    selectedId: string | null;
    onSelect: (id: string) => void;
    onDelete: (id: string) => void;
  } = $props();

  const query = useConversations();

  function formatDate(value: string) {
    return new Intl.DateTimeFormat("en", {
      month: "short",
      day: "numeric",
      hour: "numeric",
      minute: "2-digit",
      hour12: false
    }).format(new Date(value));
  }
</script>

<div class="flex flex-col gap-1">
  {#if query.isLoading}
    <div class="flex items-center justify-center py-6">
      <Loader />
    </div>
  {:else if (query.data ?? []).length === 0}
    <div class="px-3 py-6 text-center text-sm text-(--text-muted)">No conversations yet</div>
  {:else}
    {#each query.data ?? [] as conversation (conversation.id)}
      <button
        class={cn(
          "group flex cursor-pointer items-center gap-2 rounded-(--radius) px-3 py-2 hover:bg-(--button-bg-secondary-hover)",
          conversation.id === selectedId && "bg-(--button-bg-secondary-hover)"
        )}
        onclick={() => onSelect(conversation.id)}
      >
        <div class="min-w-0 flex-1 text-start">
          <div class="truncate text-sm font-semibold">{conversation.title}</div>
          <div class="text-xs text-(--text-muted)">{formatDate(conversation.updated_at)}</div>
        </div>
        <Button
          variant="icon"
          class="shrink-0 opacity-0 group-hover:opacity-100"
          onclick={(event: MouseEvent) => {
            event.stopPropagation();
            onDelete(conversation.id);
          }}
        >
          <Trash2 class="size-4" />
        </Button>
      </button>
    {/each}
  {/if}
</div>
