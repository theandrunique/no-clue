<script lang="ts">
  import { Button, Card } from "$lib/components/ui";
  import { overlaySessionService } from "$lib/services/overlaySession.svelte";
  import { overlayStateStore } from "$lib/stores/overlayState.svelte";
  import { Camera, ChevronDown, ChevronUp, Mic, X } from "@lucide/svelte";
</script>

<Card
  class="flex h-[54px] items-center justify-between bg-(--bg-card)/50 px-1.5"
  data-tauri-drag-region
>
  <span class="flex items-center">
    <Button variant="icon" onclick={() => overlayStateStore.toggleExpanded()}>
      {#if overlayStateStore.expanded}
        <ChevronDown />
      {:else}
        <ChevronUp />
      {/if}
    </Button>

    <Button variant="icon">
      <Mic />
    </Button>

    <Button variant="icon">
      <Camera />
    </Button>
  </span>

  <Button variant="icon" onclick={() => overlaySessionService.stop()}>
    <X />
  </Button>
</Card>
