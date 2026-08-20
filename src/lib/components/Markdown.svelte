<script lang="ts">
  import SvelteMarkdown from "@humanspeak/svelte-markdown";

  let {
    source,
    streaming
  }: {
    source: string;
    streaming: boolean;
  } = $props();
</script>

<SvelteMarkdown {source} {streaming} />
