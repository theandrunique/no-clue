<script setup lang="ts">
import { onMounted } from "vue";
import { useAiProvidersStore } from "@/stores/aiProviders";

const providersStore = useAiProvidersStore();

onMounted(async () => {
  await providersStore.loadProviders();
  providersStore.loadSelectedProvider();
});
</script>

<template>
  <router-view />
</template>