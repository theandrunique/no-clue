<script lang="ts">
  import { Dialog } from "bits-ui";
  import { cn } from "$lib/utils";
  import { X } from "@lucide/svelte";

  let { class: className, ...rest }: Dialog.CloseProps = $props();

  let cls = $derived(
    cn(
      "absolute top-2 right-2 bg-transparent p-(--button-padding-y) rounded-(--radius) cursor-pointer",
      "hover:bg-(--button-bg-secondary-hover) active:bg-(--button-bg-secondary-active) ",
      className
    )
  );
</script>

<Dialog.Close class={cls} {...rest}>
  <X />
</Dialog.Close>
