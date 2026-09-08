<script>
  import { LlmChatNew } from "$features/llm-chat";
  import { Card } from "$lib/components/ui";
  import LlmChatContextProvider from "$services/llm-chat/LlmChatContextProvider.svelte";
</script>

<Card class="flex flex-1 items-center justify-center">
  <LlmChatContextProvider conversationId={null}>
    <LlmChatNew />
  </LlmChatContextProvider>
</Card>
