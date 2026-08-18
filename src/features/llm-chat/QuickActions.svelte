<script lang="ts">
  import { Button } from "$lib/components/ui";

  let props: {
    isLoading?: boolean;
    isStreaming?: boolean;
    onSend: (v: string) => void;
  } = $props();

  const QUICK_ACTIONS = [
    { id: "summarize", label: "Summarize", prompt: "Summarize this conversation" },
    { id: "explain", label: "Explain", prompt: "Explain the last message in simple terms" },
    { id: "fix", label: "Fix grammar", prompt: "Fix the grammar and spelling of my last message" },
    { id: "todo", label: "To-do list", prompt: "Create a to-do list from our conversation" },
    { id: "next-steps", label: "Next steps", prompt: "What should I do next?" }
  ];
</script>

<div class="flex shrink-0 gap-1.5 overflow-x-auto pb-1">
  {#each QUICK_ACTIONS as action (action.id)}
    <Button
      variant="secondary"
      class="shrink-0 px-3 py-1 text-sm"
      onclick={() => props.onSend(action.prompt)}
      disabled={props.isStreaming || props.isLoading}
    >
      {action.label}
    </Button>
  {/each}
</div>
