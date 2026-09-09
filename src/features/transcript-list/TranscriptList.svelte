<script lang="ts">
  import { Loader } from "$lib/components/ui";
  import { useTranscripts } from "$lib/queries/transcriptions";
  import TranscriptResult from "./TranscriptResult.svelte";

  let { conversationId }: { conversationId: string | null } = $props();

  const query = useTranscripts(() => conversationId);
</script>

<div class="flex h-full flex-col gap-2 overflow-y-auto py-1">
  {#if query.isPending}
    <div class="flex h-full items-center justify-center">
      <Loader />
    </div>
  {:else if query.data?.length === 0}
    <div class="flex h-full items-center justify-center text-(--text-muted)">No transcripts yet</div>
  {:else}
    {#each query.data as result (result.id)}
      <TranscriptResult transcript={result} />
    {/each}
  {/if}
</div>
