<script lang="ts">
  import type { Message } from "$lib/types";

  let { message }: { message: Message } = $props();

  function messageClasses(message: Message) {
    if (message.role === "user") {
      return "self-end bg-(--color-primary) text-(--text-on-primary)";
    }
    if (message.role === "assistant") {
      return "self-start bg-(--button-bg-secondary)";
    }
    return "self-start bg-(--button-bg-secondary) opacity-80";
  }
</script>

<div
  class={["max-w-[85%] rounded-(--radius) px-3 py-2 text-base whitespace-pre-wrap", messageClasses(message)].join(" ")}
>
  {message.content}
</div>
