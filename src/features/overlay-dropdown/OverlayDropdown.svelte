<script lang="ts">
  import { Card } from "$lib/components/ui";
  import { overlayStateStore } from "$lib/stores/overlayStateStore.svelte";
</script>

{#if overlayStateStore.expanded}
  <Card class="flex-1 h-full bg-(--bg-card)/50">
    Some text goes here
  </Card>
{/if}
