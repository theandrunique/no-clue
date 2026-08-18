<script lang="ts">
  import { Select } from "../ui";
  import { useId } from "bits-ui";
  import { fly } from "svelte/transition";
  import type { ComponentProps } from "svelte";

  type Props = ComponentProps<typeof Select> & {
    label?: string;
    errors?: string[];
  };

  let { value = $bindable(), label, errors, type, ...props }: Props = $props();

  const id = useId();
</script>

<div class="flex flex-col gap-1">
  {#if label}
    <label for={id} class="text-sm select-none">{label}</label>
  {/if}

  <Select bind:value={value as never} type={type as never} invalid={errors != undefined} {id} {...props} />

  {#if errors}
    <p class="text-sm font-semibold text-(--text-error)" transition:fly={{ y: -5, duration: 200 }}>
      {errors}
    </p>
  {/if}
</div>
