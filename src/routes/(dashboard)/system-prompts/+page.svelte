<script>
  import { SystemPromptsList } from "$features/list-system-prompts";
  import { Button } from "$lib/components/ui";
  import { createSystemPromptModal } from "$lib/stores/createSystemPromptModal.svelte";
  import { Plus } from "@lucide/svelte";
</script>

<div class="flex flex-1 flex-col gap-4 p-2">
  <div class="flex items-center justify-between">
    <h1 class="text-2xl font-semibold">System prompts</h1>
    <Button onclick={() => createSystemPromptModal.open()}><Plus />Create</Button>
  </div>

  <SystemPromptsList />
</div>
