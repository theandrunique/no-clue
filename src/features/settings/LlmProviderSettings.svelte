<script lang="ts">
  import { Button, Loader } from "$lib/components/ui";
  import { Plus, Unplug } from "@lucide/svelte";
  import { useLlmProviders, useRemoveLlmProviderSettings } from "$lib/queries/llmProviders";
  import { llmProviderConnectModal } from "$services/settings/llmProviderConnectModal.svelte";
  import LlmProviderConnectModal from "./LlmProviderConnectModal.svelte";

  let query = useLlmProviders();

  let removeMutation = useRemoveLlmProviderSettings();

  function disconnect(providerId: string) {
    removeMutation.mutate(providerId, {
      onError: (e) => {
        console.error(e, "Error while removing llm provider");
      }
    });
  }
</script>

{#if query.isPending}
  <div class="flex items-center justify-center py-8">
    <Loader />
  </div>
{:else if query.isError}
  <p class="text-sm text-(--text-error)">Failed to load LLM providers ({query.error})</p>
{:else}
  <div class="flex flex-col gap-3">
    {#each query.data ?? [] as provider (provider.id)}
      <div class="flex items-center justify-between">
        <div class="flex flex-col">
          <span>{provider.display_name}</span>
          {#if provider.settings}
            <span class="text-sm text-(--text-muted)">Connected</span>
          {/if}
        </div>

        {#if provider.settings}
          <Button variant="secondary" disabled={removeMutation.isPending} onclick={() => disconnect(provider.id)}>
            <Unplug />
            Disconnect
          </Button>
        {:else}
          <Button onclick={() => llmProviderConnectModal.open(provider)}>
            <Plus />
            Connect
          </Button>
        {/if}
      </div>
    {/each}
  </div>
{/if}

<LlmProviderConnectModal />
