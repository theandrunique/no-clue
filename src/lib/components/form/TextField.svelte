<script lang="ts">
  import { Input } from "../ui";
  import { useId } from "bits-ui";
  import { fly } from "svelte/transition";
  import type { ComponentProps } from "svelte";

  type Props = ComponentProps<typeof Input> & {
    label?: string;
    errors?: string[];
  };

  let { value = $bindable(), label, errors, ...props }: Props = $props();

  const id = useId();
</script>

<div class="flex flex-col gap-1">
  {#if label}
    <label for={id} class="text-sm select-none">{label}</label>
  {/if}

  <Input bind:value {id} aria-invalid={errors != undefined ? "true" : undefined} {...props} />

  {#if errors}
    <p class="text-sm text-(--text-error)" transition:fly={{ y: -5, duration: 200 }}>
      {errors}
    </p>
  {/if}
</div>
