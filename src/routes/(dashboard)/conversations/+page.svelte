<script>
  import { Card, Tabs } from "$lib/components/ui";
</script>

<div class="flex-1">
  <div>Conversations</div>

  <Card>
    <Tabs.Root value="some">
      <Tabs.List>
        <Tabs.Trigger value="some">Some</Tabs.Trigger>
        <Tabs.Trigger value="kek">Kek</Tabs.Trigger>
        <Tabs.Trigger value="alololo">Alololo</Tabs.Trigger>
      </Tabs.List>

      <Tabs.Content value="some">some</Tabs.Content>

      <Tabs.Content value="kek">kek</Tabs.Content>

      <Tabs.Content value="alololo">alololo</Tabs.Content>
    </Tabs.Root>
  </Card>
</div>
