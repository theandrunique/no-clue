<script lang="ts">
  import { ConversationList } from "$features/conversation-list";
  import { LlmChat } from "$features/llm-chat";
  import { TranscriptList } from "$features/transcript-list";
  import { Button, Card, Tabs } from "$lib/components/ui";
  import { useConversation, useDeleteConversation } from "$lib/queries/conversations";
  import { createLlmChatService } from "$lib/services/llmChat.svelte";
  import { overlaySessionService } from "$lib/services/overlaySession.svelte";
  import { createTranscriptionService } from "$lib/services/transcription.svelte";
  import { FileText, MessageSquare, Play, Trash2 } from "@lucide/svelte";

  let selectedId = $state<string | null>(null);
  let tab = $state("chat");

  const deleteConversation = useDeleteConversation();
  const conversationQuery = useConversation(() => selectedId);

  function handleDelete(id: string) {
    deleteConversation.mutate(id);
    if (id === selectedId) {
      selectedId = null;
    }
  }

  function formatDate(value: string) {
    return new Intl.DateTimeFormat("en", {
      month: "short",
      day: "numeric",
      hour: "numeric",
      minute: "2-digit",
      hour12: false
    }).format(new Date(value));
  }

  let llmChatService = $derived.by(() => {
    if (selectedId === null) return;
    const service = createLlmChatService();
    service.init(selectedId);
    return service;
  });

  let transcriptionService = $derived.by(() => {
    if (selectedId === null) return;
    const service = createTranscriptionService();
    service.init(selectedId);
    return service;
  });
</script>

<div class="flex min-h-0 flex-1 gap-3 p-3">
  <Card class="flex w-72 shrink-0 flex-col gap-2 overflow-y-auto">
    <ConversationList {selectedId} onSelect={(id) => (selectedId = id)} onDelete={handleDelete} />
  </Card>

  <div class="flex min-h-0 flex-1 flex-col">
    {#if selectedId && llmChatService && transcriptionService}
      {#key selectedId}
        <Card class="flex min-h-0 flex-1 flex-col gap-3">
          <div class="flex shrink-0 items-center justify-between gap-2 border-b border-(--color-border) pb-3">
            <div class="min-w-0">
              <div class="truncate text-lg font-semibold">
                {conversationQuery.data?.title ?? "New conversation"}
              </div>
              {#if conversationQuery.data?.updated_at}
                <div class="text-sm text-(--text-muted)">
                  {formatDate(conversationQuery.data.updated_at)}
                </div>
              {/if}
            </div>

            <div class="flex shrink-0 items-center gap-2">
              <Button variant="secondary" onclick={() => overlaySessionService.start(selectedId as string)}>
                <Play class="size-4" />
                Resume
              </Button>
              <Button variant="icon" onclick={() => handleDelete}>
                <Trash2 class="size-4" />
              </Button>
            </div>
          </div>

          <Tabs.Root bind:value={tab} class="flex min-h-0 flex-1 flex-col">
            <Tabs.List>
              <Tabs.Trigger value="chat"><MessageSquare class="size-4" /> Chat</Tabs.Trigger>
              <Tabs.Trigger value="transcript"><FileText class="size-4" /> Transcripts</Tabs.Trigger>
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
      {/key}
    {:else}
      <Card class="flex flex-1 items-center justify-center text-(--text-muted)">Select a conversation</Card>
    {/if}
  </div>
</div>
