<script lang="ts">
  import { page } from "$app/state";
  import { LlmChat } from "$features/llm-chat";
  import { OverlayMenu } from "$features/overlay-menu";
  import { TranscriptList } from "$features/transcript-list";
  import { Card, Tabs } from "$lib/components/ui";
  import { FileText, MessageSquare } from "@lucide/svelte";
  import { cubicInOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import { createLlmChatService } from "$services/llm-chat/llmChat.svelte";
  import { createTranscriptionService } from "$services/transcriptions/transcription.svelte";
  import { overlayStateStore } from "$services/overlay/overlayState.svelte";

  const conversationId = $derived(page.params.conversationId);
  let tab = $state("chat");

  let llmChatService = $derived.by(() => {
    if (!conversationId) return;
    const service = createLlmChatService();
    service.init(conversationId);
    return service;
  });

  let transcriptionService = $derived.by(() => {
    if (!conversationId) return;
    const service = createTranscriptionService();
    service.init(conversationId);
    return service;
  });
</script>

<div class="flex h-screen flex-col gap-1 overflow-hidden">
  {#if llmChatService && transcriptionService}
    <OverlayMenu {llmChatService} {transcriptionService} />
  {/if}

  {#if overlayStateStore.expanded && llmChatService && transcriptionService}
    <div transition:fly={{ y: 16, duration: 200, easing: cubicInOut }} class="min-h-0 flex-1">
      <Card class="h-full bg-(--bg-card)/50">
        <Tabs.Root bind:value={tab} class="flex h-full flex-col">
          <Tabs.List>
            <Tabs.Trigger value="chat"><MessageSquare class="h-4 w-4" /> Chat</Tabs.Trigger>
            <Tabs.Trigger value="transcript"><FileText class="h-4 w-4" /> Transcript</Tabs.Trigger>
          </Tabs.List>

          <Tabs.Content value="chat" class="min-h-0 flex-1">
            <LlmChat
              isLoading={llmChatService.isLoading}
              isStreaming={llmChatService.isStreaming}
              onSend={(v) => llmChatService.send(v)}
              onStop={() => llmChatService.stop()}
              messages={llmChatService.messages}
              error={llmChatService.error}
              clearError={() => llmChatService.clearError()}
            />
          </Tabs.Content>

          <Tabs.Content value="transcript" class="min-h-0 flex-1">
            <TranscriptList
              transcripts={transcriptionService.liveResults}
              error={transcriptionService.error}
              onErrorClear={() => transcriptionService.clearError()}
            />
          </Tabs.Content>
        </Tabs.Root>
      </Card>
    </div>
  {/if}
</div>
