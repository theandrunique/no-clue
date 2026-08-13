<script>
  import { Card } from "$lib/components/ui";
  import { useSystemPrompts } from "$lib/queries/systemPrompts";

  let query = useSystemPrompts();

</script>

<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4">
  {#each query.data ?? [] as systemPrompt (systemPrompt.id)}
    <Card>
      <span class="font-semibold truncate">{systemPrompt.name}</span>
      <p class="grow text-xs text-(--text-muted) line-clamp-3">{systemPrompt.prompt}</p>
    </Card>
  {/each}
</div>
