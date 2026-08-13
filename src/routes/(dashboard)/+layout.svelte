<script lang="ts">
  import AppSidebar from '$lib/components/AppSidebar.svelte';

  let { children } = $props();
</script>

<div class="h-screen flex">
  <AppSidebar />
  {@render children()}
</div>
