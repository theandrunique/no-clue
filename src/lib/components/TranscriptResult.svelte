<script lang="ts">
  import type { Transcript, TranscriptResult } from "$lib/types";
  import { Mic, MonitorSpeaker } from "@lucide/svelte";

  let {
    transcript
  }: {
    transcript: TranscriptResult | Transcript;
  } = $props();

  function formatTime(iso: string) {
    return new Date(iso).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  }

  function resultClasses(result: TranscriptResult | Transcript) {
    return result?.is_final
      ? "border-(--color-border) bg-(--bg-card)"
      : "border-(--color-border)/50 bg-(--bg-card)/50 opacity-70";
  }
</script>

<div class={["flex flex-col gap-1 rounded-(--radius) border px-3 py-2", resultClasses(transcript)].join(" ")}>
  <div class="flex items-center justify-between gap-2">
    {#if transcript.source === "microphone"}
      <span class="flex shrink-0 items-center gap-1 text-sm text-(--text-success)">
        <Mic class="size-4" />
        Mic
      </span>
    {:else if transcript.source === "system"}
      <span class="flex shrink-0 items-center gap-1 text-sm text-(--text-info)">
        <MonitorSpeaker class="size-4" />
        System
      </span>
    {/if}

    <span class="text-sm text-(--text-muted)">
      {transcript.confidence.toFixed(2)}
    </span>

    <span class="text-sm text-(--text-muted)">{formatTime(transcript.created_at)}</span>
  </div>

  <p class="text-base whitespace-pre-wrap">{transcript.text}</p>
</div>
