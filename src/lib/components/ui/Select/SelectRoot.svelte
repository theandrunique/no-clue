<script lang="ts">
  import { Select, type WithoutChild } from "bits-ui";
  import { ChevronDown, Check, X } from "@lucide/svelte";
  import { cn } from "$lib/utils";
  import Loader from "$lib/components/ui/Loader.svelte";

  type Props = WithoutChild<Select.RootProps> & {
    placeholder?: string;
    loading?: boolean;
    showClearButton?: boolean;
    items?: { value: string; label: string; disabled?: boolean }[];
    id?: string;
    invalid?: boolean;
  };
  let {
    value = $bindable(),
    items = [],
    placeholder,
    loading,
    disabled: rawDisabled,
    type,
    id,
    showClearButton,
    invalid,
    ...rest
  }: Props = $props();

  let disabled = $derived.by(() => rawDisabled || loading);

  let triggerClasses = $derived(
    cn(
      "flex items-center justify-between w-full px-(--select-padding-x) py-(--select-padding-y) cursor-pointer bg-(--select-bg) select-none",
      "border border-(--select-border-color) hover:border-(--select-border-color-hover) focus:border-(--select-border-color-focused) rounded-(--select-radius)",
      "focus:outline-(length:--border-outline-width) focus:outline-(--select-border-color-focused) focus:outline-offset-0",
      "data-[state=open]:outline-(length:--border-outline-width) data-[state=open]:outline-(--select-border-color-focused) data-[state=open]:outline-offset-0",
      "text-base data-[placeholder]:text-(--text-muted) data-[placeholder]:font-semibold",
      "data-[state=open]:[&_svg]:rotate-180",
      "disabled:opacity-70 disabled:cursor-not-allowed"
    )
  );

  let contentClasses = $derived(
    cn(
      "select-dropdown z-50 max-h-[--bits-select-content-available-height] min-w-(--bits-select-anchor-width)",
      "bg-(--select-dropdown-bg) rounded-(--select-radius) p-(--select-dropdown-padding) shadow-[0_8px_30px_rgb(0,0,0,0.8)]",
      "border border-(--select-border-color)",
      "origin-[--bits-select-content-transform-origin]"
    )
  );

  let itemClasses = $derived(
    cn(
      "relative flex justify-between items-center outline-none group",
      "cursor-pointer select-none px-(--select-padding-x) py-(--select-padding-y) rounded-(--select-radius)",
      "data-[highlighted]:bg-(--select-item-hover)",
      "text-base text-(--text-primary)",
      "data-[disabled]:opacity-50 data-[disabled]:cursor-not-allowed"
    )
  );

  let checkboxClasses = $derived(
    cn(
      "flex items-center justify-center p-(--checkbox-padding) size-5 bg-(--checkbox-bg)",
      "border border-(--checkbox-border-color) group-hover:border-(--checkbox-border-color-hover) rounded-(--checkbox-radius)",
      "disabled:opacity-50 disabled:cursor-not-allowed"
    )
  );
</script>

<Select.Root bind:value={value as never} {items} {disabled} type={type as never} {...rest}>
  <Select.Trigger {id} class={triggerClasses} aria-invalid={invalid ? "true" : undefined}>
    <span class="flex gap-2">
      {#if loading}
        <span class="shrink-0 text-(--text-muted)">
          <Loader />
        </span>
      {/if}
      <Select.Value class="text-nowrap" {placeholder} />
    </span>

    <span class="flex shrink-0 items-center gap-2 text-(--text-muted)">
      {#if showClearButton && value}
        <button
          class="cursor-pointer rounded-(--radius) hover:bg-(--button-bg-secondary-hover)/50 hover:text-(--text-primary)"
          onpointerdown={(e) => e.stopPropagation()}
          onpointerup={(e) => e.stopPropagation()}
          onkeydown={(e) => e.stopPropagation()}
          onclick={(e) => {
            e.stopPropagation();
            value = undefined;
          }}
        >
          <X />
        </button>
      {/if}
      <ChevronDown class="h-6 w-6" />
    </span>
  </Select.Trigger>

  <Select.Portal>
    <Select.Content class={contentClasses} sideOffset={4} collisionPadding={5}>
      {#each items as { value, label, disabled } (value)}
        <Select.Item {value} {label} {disabled} class={itemClasses}>
          {#snippet children({ selected })}
            {#if type === "multiple"}
              <div class="flex items-center gap-2">
                <div class={cn(checkboxClasses, selected ? "bg-(--checkbox-bg-checked)" : "")}>
                  {#if selected}
                    <Check strokeWidth={3} />
                  {/if}
                </div>
                {label}
              </div>
            {:else}
              {label}
              {#if selected}
                <Check class="h-5 w-5" />
              {/if}
            {/if}
          {/snippet}
        </Select.Item>
      {/each}
    </Select.Content>
  </Select.Portal>
</Select.Root>

<style>
  @keyframes enter {
    from {
      opacity: 0;
      transform: scale(0.95);
    }
  }
  @keyframes exit {
    to {
      opacity: 0;
      transform: scale(0.95);
    }
  }

  :global(.select-dropdown[data-state="open"]) {
    animation: enter 150ms ease-out;
  }

  :global(.select-dropdown[data-state="closed"]) {
    animation: exit 150ms ease-out;
  }
</style>
