<script lang="ts">
  import { cn } from "$lib/utils";
  import { Check } from "@lucide/svelte";
  import { Checkbox, Label, useId, type WithoutChildrenOrChild } from "bits-ui";

  let {
    id = useId(),
    checked = $bindable(false),
    ref = $bindable(null),
    labelRef = $bindable(null),
    labelText,
    disabled,
    ...restProps
  }: WithoutChildrenOrChild<Checkbox.RootProps> & {
    labelText: string;
    labelRef?: HTMLLabelElement | null;
  } = $props();

  let rootClasses = $derived(
    cn(
      "flex items-center justify-center p-(--checkbox-padding) size-5 cursor-pointer",
      "bg-(--checkbox-bg) data-[state=checked]:bg-(--checkbox-bg-checked)",
      "border border-(--checkbox-border-color) hover:border-(--checkbox-border-color-hover) rounded-(--checkbox-radius)",
      "disabled:opacity-50 disabled:cursor-not-allowed"
    )
  );

  let labelClasses = $derived(cn("cursor-pointer", disabled ? "cursor-not-allowed" : ""));
</script>

<div class="flex items-center gap-2">
  <Checkbox.Root {id} bind:checked bind:ref class={rootClasses} {...restProps} {disabled}>
    {#snippet children({ checked, indeterminate })}
      {#if indeterminate}
        -
      {:else if checked}
        <Check strokeWidth={3} />
      {:else}
        <Check class="invisible" strokeWidth={3} />
      {/if}
    {/snippet}
  </Checkbox.Root>
  <Label.Root for={id} bind:ref={labelRef} class={labelClasses}>
    {labelText}
  </Label.Root>
</div>
