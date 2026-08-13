<script lang="ts">
  import { cn } from "$lib/utils";
  import type { HTMLAttributes } from "svelte/elements";

  let { class: className, children, ...rest }: HTMLAttributes<HTMLDivElement> = $props();
</script>

<div
  class={cn("rounded-(--radius) border border-(--color-border) bg-(--bg-card) p-4", className)}
  {...rest}
>
  {@render children?.()}
</div>
