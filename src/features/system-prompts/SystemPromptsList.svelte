<script lang="ts">
  import { Button, Card } from "$lib/components/ui";
  import { useSystemPrompts } from "$lib/queries/systemPrompts";
  import { activePromptStore } from "$services/system-prompts/activePrompt.svelte";
  import { editSystemPromptModal } from "$services/system-prompts/editSystemPromptModal.svelte";
  import { Check, SquarePen } from "@lucide/svelte";

  let query = useSystemPrompts();

  function formatDateTime(value: string) {
    return new Intl.DateTimeFormat("en", {
      year: "numeric",
      month: "short",
      day: "numeric",
      hour: "numeric",
      minute: "numeric",
      hour12: false
    }).format(new Date(value));
  }
</script>

<div class="grid grid-cols-1 gap-3 overflow-auto sm:grid-cols-2 lg:grid-cols-3">
  {#each query.data ?? [] as systemPrompt (systemPrompt.id)}
    <Card
      class="relative flex cursor-pointer flex-col"
      ondblclick={() => activePromptStore.toggleActivePrompt(systemPrompt.id)}
    >
      <span class="truncate font-semibold">{systemPrompt.name}</span>
      <p class="line-clamp-3 flex-1 text-xs text-(--text-muted)">{systemPrompt.prompt}</p>
      <p class="text-xs text-(--text-muted)">{formatDateTime(systemPrompt.updated_at)}</p>

      {#if activePromptStore.activePromptId === systemPrompt.id}
        <div class="absolute top-3 right-3 rounded-full bg-green-500/20 p-1 text-green-400">
          <Check class="h-3 w-3" />
        </div>
      {/if}

      <div class="absolute right-3 bottom-3 rounded-full p-1">
        <Button variant="icon" onclick={() => editSystemPromptModal.open(systemPrompt)}>
          <SquarePen />
        </Button>
      </div>
    </Card>
  {/each}
</div>
