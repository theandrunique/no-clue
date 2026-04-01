<script setup lang="ts">
import { onMounted } from "vue";
import { useProvidersStore } from "@/stores/providers";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

const providersStore = useProvidersStore();

const startTest = async () => {
  const outputs = await invoke("get_output_devices");
  console.log(outputs);
  const inputs = await invoke("get_input_devices");
  console.log(inputs);

  await invoke("test_stream_audio", {
    deviceId: null,
    durationSecs: 5,
  });

  listen("test-stream-started", (e) => console.log("started:", e.payload));
  listen("test-stream-progress", (e) => console.log("progress:", e.payload));
  listen("test-stream-stopped", (e) => console.log("stopped:", e.payload));
};

onMounted(async () => {
  await providersStore.loadProviders();
  providersStore.loadSelectedProvider();
  await startTest();
});
</script>

<template>
  <router-view />
</template>
