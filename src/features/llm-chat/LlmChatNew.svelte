<script lang="ts">
  import { Button } from "$lib/components/ui";
  import { modelSettingsStore } from "$services/settings/modelSettings.svelte";
  import { getLlmChatContext } from "$services/llm-chat/llmChatContext";
  import LlmChatInput from "./LlmChatInput.svelte";
  import LlmChatQuickActions from "./LlmChatQuickActions.svelte";
  import ModelBar from "./ModelBar.svelte";
  import { useCreateConversation } from "$lib/queries/conversations";
  import { useSendMessage } from "$lib/queries/chat";
  import { page } from "$app/state";

  const ctx = getLlmChatContext();
  const createConversationMutation = useCreateConversation();
  const sendMessageMutation = useSendMessage();

  async function handleSend(text: string) {
    const newConversation = await createConversationMutation.mutateAsync();
    page.params.id = newConversation.id;
    await sendMessageMutation.mutateAsync({
      conversationId: newConversation.id,
      userMessage: text,
      systemPromptId: undefined,
      captureScreenshot: true,
      model: { type: "Fake", duration: "15s" },
    })
  }
</script>

<div class="flex-1 min-w-0 flex flex-col gap-2">
  {#if !modelSettingsStore.hasModels}
    <div class="flex items-center justify-center gap-1 py-4">
      <p class="text-sm">Connect LLM provider to start chatting</p>
      <Button href="/settings">To settings</Button>
    </div>
  {:else}
    <LlmChatQuickActions
      isStreaming={ctx.isStreaming}
      isLoading={sendMessageMutation.isPending}
      onSend={(v) => handleSend(v)}
    />

    <LlmChatInput
      isStreaming={ctx.isStreaming}
      isLoading={false}
      onSend={(v) => handleSend(v)}
      onStop={() => console.log("Unexpected stop")}
    />

    <ModelBar
      models={modelSettingsStore.models}
      model={modelSettingsStore.selectedModel}
      disabled={ctx.isStreaming}
      onModelChange={(m) => modelSettingsStore.setSelectedModel(m)}
    />
  {/if}
</div>
