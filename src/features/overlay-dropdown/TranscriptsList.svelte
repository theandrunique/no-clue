<script lang="ts">
  import TranscriptResult from "$lib/components/TranscriptResult.svelte";
  import { transcriptionService } from "$lib/services/transcription.svelte";
  import ErrorMessage from "./ui/ErrorMessage.svelte";
</script>

<div class="flex h-full flex-col gap-2 overflow-y-auto py-3">
  {#if transcriptionService.error}
    <ErrorMessage error={transcriptionService.error} onClear={() => transcriptionService.clearError()} />
  {/if}

  {#if transcriptionService.liveResults.length === 0}
    <div class="flex h-full items-center justify-center text-(--text-muted)">
      No transcripts yet
    </div>
  {:else}
    {#each transcriptionService.liveResults as result (result.id)}
      <TranscriptResult transcript={result} />
    {/each}
  {/if}
</div>
