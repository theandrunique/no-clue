<script lang="ts">
  import ErrorMessage from "$lib/components/ErrorMessage.svelte";
  import { Loader } from "$lib/components/ui";
  import type { Transcript, TranscriptResult as TranscriptResultType } from "$lib/types";
  import TranscriptResult from "./TranscriptResult.svelte";

  let {
    error,
    onErrorClear,
    transcripts,
    isLoading
  }: {
    transcripts: (Transcript | TranscriptResultType)[];
    isLoading?: boolean;
    error?: string | null;
    onErrorClear: () => void;
  } = $props();
</script>

<div class="flex h-full flex-col gap-2 overflow-y-auto py-1">
  {#if error}
    <ErrorMessage {error} onClear={() => onErrorClear()} />
  {/if}

  {#if isLoading}
    <div class="flex h-full items-center justify-center">
      <Loader />
    </div>
  {:else if transcripts.length === 0}
    <div class="flex h-full items-center justify-center text-(--text-muted)">No transcripts yet</div>
  {:else}
    {#each transcripts as result (result.id)}
      <TranscriptResult transcript={result} />
    {/each}
  {/if}
</div>
