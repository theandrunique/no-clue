<script lang="ts">
  import { Button, Card } from "$lib/components/ui";
  import { overlayStateStore } from "$lib/stores/overlayStateStore.svelte";
  import { ChevronDown, ChevronUp, Mic, X } from "@lucide/svelte";
  import { invoke } from "@tauri-apps/api/core";
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
  </span>

  <Button variant="icon" onclick={() => invoke("close_overlay_session")}>
    <X />
  </Button>
</Card>
