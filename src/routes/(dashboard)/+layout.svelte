<script lang="ts">
  import { CreateSystemPromptModal, EditSystemPromptModal } from "$features/system-prompts";
  import AppSidebar from "$lib/components/AppSidebar.svelte";

  let { children } = $props();
</script>

<div class="flex h-screen bg-(--bg-page)">
  <AppSidebar />
  {@render children()}

  <CreateSystemPromptModal />
  <EditSystemPromptModal />
</div>
