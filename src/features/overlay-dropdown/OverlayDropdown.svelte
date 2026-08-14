<script lang="ts">
  import { Card } from "$lib/components/ui";
  import { overlayStateStore } from "$lib/stores/overlayStateStore.svelte";
  import { cubicInOut } from "svelte/easing";
  import { fly } from "svelte/transition";
</script>

{#if overlayStateStore.expanded}
  <div transition:fly={{ y: 16, duration: 200, easing: cubicInOut }} class="min-h-0 flex-1">
    <Card class="h-full bg-(--bg-card)/50">Some text goes here</Card>
  </div>
{/if}
