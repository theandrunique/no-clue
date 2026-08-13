<script lang="ts">
  import { cn } from "$lib/utils";
  import { X } from "@lucide/svelte";
  import type { HTMLInputAttributes } from "svelte/elements";

  type Props = HTMLInputAttributes & {
    value?: string;
    showClearButton?: boolean;
  };

  let { value = $bindable(), class: className = "", showClearButton, ...rest }: Props = $props();

  let classes = $derived(
    cn(
      "w-full px-(--input-padding-x) py-(--input-padding-y) bg-(--input-bg) select-none",
      "border rounded-(--input-radius) border-(--input-border-color) hover:border-(--input-border-color-hover)",
      "text-base placeholder:text-(--text-muted) placeholder:font-semibold",
      "focus:outline-(length:--border-outline-width) focus:outline-(--input-border-color-focused) focus:outline-offset-0 focus:border-(--input-border-color-focused)",
      "disabled:opacity-60 disabled:cursor-not-allowed",
      className
    )
  );
</script>

<div class="relative w-full">
  <input bind:value class={classes} class:pr-9={showClearButton && value} {...rest} />
  {#if showClearButton && value}
    <button
      class={cn(
        "absolute inset-y-0 right-2.5 my-auto cursor-pointer rounded-(--radius)",
        "flex size-6 items-center justify-center",
        "text-(--text-muted) hover:bg-(--button-bg-secondary-hover) hover:text-(--text-primary)"
      )}
      onclick={() => (value = "")}
    >
      <X />
    </button>
  {/if}
</div>
