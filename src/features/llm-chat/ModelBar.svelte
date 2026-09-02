<script lang="ts">
  import SelectField from "$lib/components/form/SelectField.svelte";
  import { Checkbox, Input } from "$lib/components/ui";
  import type { Llm, LlmRuntimeConfig, LlmSettings, RuntimeSettingsField } from "$lib/types/providers";
  import { defaultsFor, extractRuntime } from "$services/llm-chat/llmSettings";

  let {
    models,
    model,
    disabled,
    onModelChange
  }: {
    models: Llm[];
    model: LlmSettings | null;
    disabled?: boolean;
    onModelChange: (model: LlmSettings) => void;
  } = $props();

  let selectedLlm = $derived(models.find((m) => m.id === model?.type) ?? null);

  let runtimeValues = $state<LlmRuntimeConfig>({});
  let numberDrafts = $state<Record<string, string>>({});
  let lastType = $state<string | null>(null);

  $effect(() => {
    const type = model?.type ?? null;
    if (type === lastType) return;
    lastType = type;
    runtimeValues = { ...extractRuntime(model) };
    numberDrafts = {};
  });

  function defaultFor(llm: Llm): LlmSettings {
    const runtime = defaultsFor(llm);
    return { type: llm.id, ...runtime } as LlmSettings;
  }

  function selectItems() {
    return models.map((m) => ({
      value: m.id,
      label: `${m.provider_display_name} — ${m.display_name}`
    }));
  }

  function handleModelChange(value: string | undefined) {
    const llm = models.find((m) => m.id === value);
    if (!llm) return;
    onModelChange(defaultFor(llm));
  }

  function emitChange() {
    if (!model) return;
    onModelChange({ ...model, ...runtimeValues } as LlmSettings);
  }

  function selectOptions(field: RuntimeSettingsField): string[] {
    return field.field_type.type === "select" ? field.field_type.options : [];
  }
</script>

<div class="flex flex-wrap items-end gap-2 px-1 pb-1">
  <div class="min-w-40">
    <SelectField
      type="single"
      items={selectItems()}
      value={model?.type}
      placeholder={models.length ? "Select model" : "No models"}
      onValueChange={handleModelChange}
      disabled={disabled || models.length === 0}
    />
  </div>

  {#if selectedLlm}
    {#each selectedLlm.runtime_config.fields as field (field.key)}
      {#if field.field_type.type === "boolean"}
        <Checkbox
          labelText={field.display_name}
          checked={Boolean(runtimeValues[field.key])}
          {disabled}
          onCheckedChange={(checked: boolean) => {
            runtimeValues[field.key] = checked;
            emitChange();
          }}
        />
      {:else if field.field_type.type === "number"}
        <label class="flex flex-col gap-1">
          <span class="text-sm select-none">{field.display_name}</span>
          <Input
            type="number"
            bind:value={numberDrafts[field.key]}
            class="w-28"
            {disabled}
            onchange={() => {
              runtimeValues[field.key] = Number(numberDrafts[field.key]);
              emitChange();
            }}
          />
        </label>
      {:else}
        <div class="min-w-32">
          <SelectField
            type="single"
            label={field.display_name}
            items={selectOptions(field).map((o) => ({ label: o, value: o }))}
            value={String(runtimeValues[field.key] ?? "")}
            onValueChange={(value: string | undefined) => {
              runtimeValues[field.key] = value ?? "";
              emitChange();
            }}
            {disabled}
          />
        </div>
      {/if}
    {/each}
  {/if}
</div>
