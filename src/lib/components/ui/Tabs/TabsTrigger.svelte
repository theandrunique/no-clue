<script lang="ts">
  import { cn } from "$lib/utils";
  import { Tabs } from "bits-ui";

  let { children, class: className, ...rest }: Tabs.TriggerProps = $props();
</script>

<Tabs.Trigger
  class={cn(
    "flex items-center justify-center gap-1 rounded-(--radius) py-1 select-none data-[state=active]:bg-(--gray-8) data-[state=active]:shadow-(--tabs-trigger-shadow)",
    className
  )}
  {...rest}
>
  {@render children?.()}
</Tabs.Trigger>
