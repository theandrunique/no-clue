<script lang="ts">
  import { Card, Tabs } from "$lib/components/ui";
  import { overlayStateStore } from "$lib/stores/overlayState.svelte";
  import { FileText, MessageSquare } from "@lucide/svelte";
  import { cubicInOut } from "svelte/easing";
  import { fly } from "svelte/transition";
  import LlmChat from "./LlmChat.svelte";
  import TranscriptionsList from "./TranscriptionsList.svelte";
</script>

{#if overlayStateStore.expanded}
  <div transition:fly={{ y: 16, duration: 200, easing: cubicInOut }} class="min-h-0 flex-1">
    <Card class="h-full bg-(--bg-card)/50">
      <Tabs.Root value="chat">
        <Tabs.List>
          <Tabs.Trigger value="chat"><MessageSquare class="h-4 w-4" /> Chat</Tabs.Trigger>
          <Tabs.Trigger value="transcript"><FileText class="h-4 w-4" /> Transcript</Tabs.Trigger>
        </Tabs.List>

        <Tabs.Content value="chat">
          <LlmChat />
        </Tabs.Content>

        <Tabs.Content value="transcript">
          <TranscriptionsList />
        </Tabs.Content>
      </Tabs.Root>
    </Card>
  </div>
{/if}
