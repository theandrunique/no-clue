<script lang="ts">
  import { Button, Card } from "$lib/components/ui";
  import { overlaySessionService } from "$services/overlay/overlaySession.svelte";
  import { overlayStateStore } from "$services/overlay/overlayState.svelte";
  import { getTranscriptionContext } from "$services/transcriptions/transcriptionContext";
  import { Camera, CameraOff, ChevronDown, ChevronUp, Mic, MicOff, X } from "@lucide/svelte";

  const transcriptionContext = getTranscriptionContext();
</script>

<Card class="flex h-13.5 items-center justify-between bg-(--bg-card)/50 px-1.5" data-tauri-drag-region>
  <span class="flex items-center">
    <Button variant="icon" onclick={() => overlayStateStore.toggleExpanded()}>
      {#if overlayStateStore.expanded}
        <ChevronDown />
      {:else}
        <ChevronUp />
      {/if}
    </Button>

    <Button variant="icon" onclick={() => transcriptionContext.toggle()}>
      {#if ["starting", "running", "stopping"].includes(transcriptionContext.status)}
        <Mic />
      {:else}
        <MicOff />
      {/if}
    </Button>

    <Button variant="icon">
      {#if true}
        <Camera />
      {:else}
        <CameraOff />
      {/if}
    </Button>
  </span>

  <span class="flex min-w-0 items-center justify-center px-2"> </span>

  <Button variant="icon" onclick={() => overlaySessionService.stop()}>
    <X />
  </Button>
</Card>
