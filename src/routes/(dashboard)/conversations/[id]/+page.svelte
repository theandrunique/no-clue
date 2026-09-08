<script lang="ts">
  import { page } from "$app/state";
  import { LlmChat } from "$features/llm-chat";
  import { TranscriptList } from "$features/transcript-list";
  import { Button, Card, Tabs } from "$lib/components/ui";
  import { useConversation, useDeleteConversation } from "$lib/queries/conversations";
  import LlmChatContextProvider from "$services/llm-chat/LlmChatContextProvider.svelte";
  import { overlaySessionService } from "$services/overlay/overlaySession.svelte";
  import { createTranscriptionService } from "$services/transcriptions/transcription.svelte";
  import { FileText, MessageSquare, Play, Trash2 } from "@lucide/svelte";

  let tab = $state("chat");
  const conversationId = page.params.conversationId!;

  const conversationQuery = useConversation(() => conversationId);
  const deleteConversation = useDeleteConversation();

  function handleDelete(id: string) {
    deleteConversation.mutate(id, {
      onSuccess: () => {
        if (id === page.params.conversationId) {
          page.params.conversationId = undefined;
        }
      }
    });
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

  let transcriptionService = $derived.by(() => {
    if (conversationId === null) return;
    const service = createTranscriptionService();
    service.init(conversationId);
    return service;
  });
</script>

<div class="flex min-h-0 flex-1 flex-col">
  {#if conversationId && transcriptionService}
    {#key conversationId}
      <Card class="flex min-h-0 flex-1 flex-col gap-3">
        <div class="flex shrink-0 items-center justify-between gap-2 border-b border-(--color-border) pb-3">
          <div class="min-w-0">
            <div class="truncate text-lg font-semibold">
              {conversationQuery.data?.title}
            </div>
            <div class="text-sm text-(--text-muted)">
              {formatDate(conversationQuery.data?.updated_at ?? "")}
            </div>
          </div>

          <div class="flex shrink-0 items-center gap-2">
            <Button variant="secondary" onclick={() => overlaySessionService.start(conversationId)}>
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
            <LlmChatContextProvider conversationId={conversationId}>
              <LlmChat conversationId={conversationId} />
            </LlmChatContextProvider>
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
