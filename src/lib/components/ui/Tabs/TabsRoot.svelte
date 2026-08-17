<script lang="ts">
  import { Tabs } from "bits-ui";

  let { value = $bindable(), children, ...rest }: Tabs.RootProps = $props();
</script>

<Tabs.Root bind:value {...rest}>
  {@render children?.()}
</Tabs.Root>
