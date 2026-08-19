<script lang="ts">
  import { Button, Card } from "$lib/components/ui";
  import type { createLlmChatService } from "$services/llm-chat/llmChat.svelte";
  import { overlaySessionService } from "$services/overlay/overlaySession.svelte";
  import { overlayStateStore } from "$services/overlay/overlayState.svelte";
  import type { createTranscriptionService } from "$services/transcriptions/transcription.svelte";
  import { Camera, CameraOff, ChevronDown, ChevronUp, Mic, MicOff, X } from "@lucide/svelte";

  let {
    llmChatService,
    transcriptionService
  }: {
    llmChatService: ReturnType<typeof createLlmChatService>;
    transcriptionService: ReturnType<typeof createTranscriptionService>;
  } = $props();
</script>

<Card class="flex h-[54px] items-center justify-between bg-(--bg-card)/50 px-1.5" data-tauri-drag-region>
  <span class="flex items-center">
    <Button variant="icon" onclick={() => overlayStateStore.toggleExpanded()}>
      {#if overlayStateStore.expanded}
        <ChevronDown />
      {:else}
        <ChevronUp />
      {/if}
    </Button>

    <Button
      variant="icon"
      onclick={() => transcriptionService.toggle()}
      class={transcriptionService.error ? "text-(--text-error)" : ""}
    >
      {#if transcriptionService.isRecording}
        <Mic />
      {:else}
        <MicOff />
      {/if}
    </Button>

    <Button variant="icon" onclick={() => llmChatService.toggleCaptureScreenshot()}>
      {#if llmChatService.captureScreenshot}
        <Camera />
      {:else}
        <CameraOff />
      {/if}
    </Button>
  </span>

  <span class="flex min-w-0 items-center justify-center px-2">
    {#if transcriptionService.error}
      <span class="truncate text-sm font-semibold text-(--text-error)">
        {transcriptionService.error}
      </span>
    {/if}
  </span>

  <Button variant="icon" onclick={() => overlaySessionService.stop()}>
    <X />
  </Button>
</Card>
