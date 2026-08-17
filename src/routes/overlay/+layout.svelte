<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { getCurrentWindow } from "@tauri-apps/api/window";

  let { children } = $props();

  const window = getCurrentWindow();
  window.onCloseRequested(async (event) => {
    event.preventDefault();
    await invoke("close_overlay_session");
  });
</script>

{@render children()}
