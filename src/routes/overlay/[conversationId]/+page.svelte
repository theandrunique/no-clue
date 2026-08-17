<script>
  import { page } from "$app/state";
  import { OverlayDropdown } from "$features/overlay-dropdown";
  import { OverlayMenu } from "$features/overlay-menu";
  import { llmChatService } from "$lib/services/llmChat.svelte";
  import { transcriptionService } from "$lib/services/transcription.svelte";

  const conversationId = $derived(page.params.conversationId);

  $effect(() => {
    if (conversationId) {
      llmChatService.init(conversationId);
      transcriptionService.init(conversationId);
    }
  });
</script>

<div class="flex h-screen flex-col gap-1 overflow-hidden">
  <OverlayMenu />

  <OverlayDropdown />
</div>
