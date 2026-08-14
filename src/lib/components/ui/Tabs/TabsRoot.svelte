<script lang="ts">
  import { Tabs } from "bits-ui";

  let { children, ...rest }: Tabs.RootProps = $props();
</script>

<Tabs.Root {...rest}>
  {@render children?.()}
</Tabs.Root>
