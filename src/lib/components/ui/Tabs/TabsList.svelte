<script lang="ts">
  import { cn } from "$lib/utils";
  import { Tabs } from "bits-ui";

  let { children, class: className, ...rest }: Tabs.ListProps = $props();
</script>

<Tabs.List
  class={cn(
    "grid w-full auto-cols-fr grid-flow-col gap-1 rounded-(--radius) border border-(--color-border) bg-(--gray-10) p-1 font-semibold shadow-(--tabs-list-shadow)",
    className
  )}
  {...rest}
>
  {@render children?.()}
</Tabs.List>
