<script>
  import { AudioSettings } from "$features/audio-settings";
  import { ProvidersSettings } from "$features/providers-settings";
  import { Card } from "$lib/components/ui";
</script>

<div class="flex min-w-0 flex-1 flex-col gap-4 overflow-auto p-2">
  <h1 class="text-2xl font-semibold">Settings</h1>

  <Card>
    <AudioSettings />
  </Card>

  <Card>
    <ProvidersSettings />
  </Card>

  <Card>
    <div>Shourtcuts</div>
  </Card>
</div>
