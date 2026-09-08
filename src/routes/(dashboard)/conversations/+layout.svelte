<script lang="ts">
  import { page } from '$app/state';
  import { ConversationList } from '$features/conversation-list';
  import { Card } from '$lib/components/ui';
  import { useDeleteConversation } from '$lib/queries/conversations';

  let { children } = $props();

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
</script>

<div class="flex min-h-0 flex-1 gap-3 p-3">
  <Card class="flex w-72 shrink-0 flex-col gap-2 overflow-y-auto">
    <ConversationList
      selectedId={page.params.id || null}
      onSelect={(id) => {
        console.log(id);
        page.params.id = id;
      }}
      onDelete={handleDelete} />
  </Card>

  {@render children()}
</div>
