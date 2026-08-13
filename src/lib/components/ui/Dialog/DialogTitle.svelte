<script lang="ts">
  import { cn } from "$lib/utils";
  import { Dialog } from "bits-ui";

  let { children, class: className, ...rest }: Dialog.TitleProps = $props();

  let cls = $derived(cn("mb-1.5 text-lg font-semibold", className));
</script>

<Dialog.Title class={cls} {...rest}>
  {@render children?.()}
</Dialog.Title>
