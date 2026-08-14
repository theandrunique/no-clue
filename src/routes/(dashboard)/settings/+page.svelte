<script>
  import { AudioSettings } from "$features/audio-settings";
  import { Card } from "$lib/components/ui";
</script>

<div class="flex min-w-0 flex-1 flex-col gap-4 p-2 overflow-auto">
  <h1 class="text-2xl font-semibold">Settings</h1>

  <Card>
    <AudioSettings />
  </Card>

  <Card>
    <div>LLM provider</div>
    <div>STT provider</div>
  </Card>

  <Card>
    <div>Shourtcuts</div>
  </Card>
</div>
