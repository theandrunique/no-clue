<script lang="ts">
  import { CreateSystemPromptModal } from "$features/create-system-prompt";
  import { EditSystemPromptModal } from "$features/edit-system-prompt";
  import AppSidebar from "$lib/components/AppSidebar.svelte";

  let { children } = $props();
</script>

<div class="flex h-screen">
  <AppSidebar />
  {@render children()}

  <CreateSystemPromptModal />
  <EditSystemPromptModal />
</div>
