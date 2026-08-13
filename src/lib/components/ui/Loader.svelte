<script lang="ts">
  import { cn } from "$lib/utils";
  import { LoaderCircle } from "@lucide/svelte";

  interface Props {
    class?: string;
  }

  let { class: className }: Props = $props();
</script>

<LoaderCircle class={cn("animate-spin bg-(--bg-gray-8)", className)} />
