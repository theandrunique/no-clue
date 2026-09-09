<script lang="ts">
  import { page } from "$app/state";
  import { LlmChat } from "$features/llm-chat";
  import { OverlayMenu } from "$features/overlay-menu";
  import { TranscriptList } from "$features/transcript-list";
  import { Card, Tabs } from "$lib/components/ui";
  import { FileText, MessageSquare } from "@lucide/svelte";
  import { cubicInOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import LlmChatContextProvider from "$services/llm-chat/LlmChatContextProvider.svelte";
  import { overlayStateStore } from "$services/overlay/overlayState.svelte";
  import TranscriptionContextProvider from "$services/transcriptions/TranscriptionContextProvider.svelte";

  const conversationId = $derived(page.params.conversationId)!;
  let tab = $state("chat");
</script>

<div class="flex h-screen flex-col gap-1 overflow-hidden">
  <LlmChatContextProvider {conversationId}>
    <TranscriptionContextProvider {conversationId}>
      <OverlayMenu />

      {#if overlayStateStore.expanded}
        <div transition:fly={{ y: 16, duration: 200, easing: cubicInOut }} class="min-h-0 flex-1">
          <Card class="h-full bg-(--bg-card)/50">
            <Tabs.Root bind:value={tab} class="flex h-full flex-col">
              <Tabs.List>
                <Tabs.Trigger value="chat"><MessageSquare class="h-4 w-4" /> Chat</Tabs.Trigger>
                <Tabs.Trigger value="transcript"><FileText class="h-4 w-4" /> Transcript</Tabs.Trigger>
              </Tabs.List>

              <Tabs.Content value="chat" class="min-h-0 flex-1">
                <LlmChat {conversationId} />
              </Tabs.Content>

              <Tabs.Content value="transcript" class="min-h-0 flex-1">
                <TranscriptList {conversationId} />
              </Tabs.Content>
            </Tabs.Root>
          </Card>
        </div>
      {/if}
    </TranscriptionContextProvider>
  </LlmChatContextProvider>
</div>
