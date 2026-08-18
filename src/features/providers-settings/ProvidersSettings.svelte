<script lang="ts">
  import { SelectField } from "$lib/components/form";
  import { useLlmProviders, useSttProviders } from "$lib/queries/providers";
  import { providerSettingsStore } from "$lib/stores/providerSettings.svelte";

  let llmProvidersQuery = useLlmProviders();

  let llmProviders = $derived(
    (llmProvidersQuery.data ?? []).map((provider) => ({
      label: provider.label,
      value: provider.id
    }))
  );

  let sttProvidersQuery = useSttProviders();

  let sttProviders = $derived(
    (sttProvidersQuery.data ?? []).map((provider) => ({
      label: provider.label,
      value: provider.id
    }))
  );
</script>

<div>
  <SelectField
    label="LLM Provider"
    type="single"
    items={llmProviders}
    loading={llmProvidersQuery.isPending}
    bind:value={providerSettingsStore.llmProviderId}
  />

  <SelectField
    label="STT Provider"
    type="single"
    items={sttProviders}
    loading={sttProvidersQuery.isPending}
    bind:value={providerSettingsStore.sttProviderId}
  />
</div>
