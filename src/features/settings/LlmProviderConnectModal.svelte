<script lang="ts">
  import { Dialog, Button } from "$lib/components/ui";
  import TextField from "$lib/components/form/TextField.svelte";
  import SelectField from "$lib/components/form/SelectField.svelte";
  import { useSaveLlmProviderSettings } from "$lib/queries/llmProviders";
  import { llmProviderConnectModal } from "$services/settings/llmProviderConnectModal.svelte";
  import type { FieldType, LlmProviderSettings, LlmProvider } from "$lib/types/providers";

  let currentProvider = $derived(llmProviderConnectModal.status.provider);

  let saveSettingsMutation = useSaveLlmProviderSettings();

  let connectValues = $state<Record<string, string>>({});
  let connectErrors = $state<Record<string, string>>({});

  async function submitConnect() {
    if (!currentProvider) return;

    const errors: Record<string, string> = {};
    for (const field of currentProvider.settings_schema.fields) {
      const value = connectValues[field.key] ?? "";
      if (field.required && !value.trim()) {
        errors[field.key] = `${field.display_name} is required`;
      }
    }

    connectErrors = errors;
    if (Object.keys(errors).length > 0) return;

    saveSettingsMutation.mutate(
      {
        providerId: currentProvider.id,
        settings: buildSettings(currentProvider, connectValues)
      },
      {
        onError: (e) => {
          console.error(e);
        },
        onSuccess: () => {
          llmProviderConnectModal.close();
        }
      }
    );
  }

  function buildSettings(provider: LlmProvider, values: Record<string, string>): LlmProviderSettings {
    if (provider.id === "ai-tunnel") {
      return { type: "AiTunnel", api_key: values["api_key"] ?? "" };
    }
    return { type: "TestingProvider" };
  }

  function fieldErrors(key: string): string[] | undefined {
    const message = connectErrors[key];
    return message ? [message] : undefined;
  }

  function selectOptions(fieldType: FieldType): string[] {
    if (fieldType.type === "select") return fieldType.options;
    return [];
  }
</script>

<Dialog.Root bind:open={() => llmProviderConnectModal.status.isOpen, () => llmProviderConnectModal.close()}>
  <Dialog.Portal>
    <Dialog.Overlay />

    <Dialog.Content>
      <Dialog.Title>Connect {currentProvider?.display_name}</Dialog.Title>
      <Dialog.Close />

      <form class="mt-4 flex flex-col gap-4" onsubmit={submitConnect} novalidate>
        {#each currentProvider?.settings_schema.fields ?? [] as field (field.key)}
          {#if field.field_type.type === "select"}
            <SelectField
              type="single"
              label={field.display_name}
              items={selectOptions(field.field_type).map((o) => ({ label: o, value: o }))}
              bind:value={connectValues[field.key]}
              errors={fieldErrors(field.key)}
              placeholder={field.placeholder ?? "Select..."}
            />
          {:else}
            <TextField
              label={field.display_name}
              type={field.field_type.type}
              name={field.key}
              placeholder={field.placeholder ?? undefined}
              bind:value={connectValues[field.key]}
              errors={fieldErrors(field.key)}
              required={field.required}
            />
          {/if}
        {/each}

        {#if saveSettingsMutation.isError}
          <p class="text-sm text-(--text-error)">{saveSettingsMutation.error}</p>
        {/if}

        <div class="flex items-center justify-end gap-2">
          <Button type="button" variant="secondary" onclick={() => llmProviderConnectModal.close()}>Cancel</Button>
          <Button type="submit" disabled={saveSettingsMutation.isPending}>
            {saveSettingsMutation.isPending ? "Saving..." : "Connect"}
          </Button>
        </div>
      </form>
    </Dialog.Content>
  </Dialog.Portal>
</Dialog.Root>
