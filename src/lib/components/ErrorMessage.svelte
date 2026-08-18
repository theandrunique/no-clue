<script lang="ts">
  let props: {
    error: string;
    onClear: () => void;
  } = $props();
</script>

<div
  class="flex items-center justify-between gap-2 rounded-(--radius) border border-(--color-error) bg-(--red-7)/20 px-3 py-2 text-sm font-semibold text-(--text-error)"
>
  <span>{props.error}</span>
  <button class="cursor-pointer" onclick={() => props.onClear()}>×</button>
</div>
