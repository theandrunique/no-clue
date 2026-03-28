<script setup lang="ts">
import { onMounted } from "vue";
import { useProvidersStore } from "@/stores/providers";

const providersStore = useProvidersStore();

onMounted(async () => {
  await providersStore.loadProviders();
  providersStore.loadSelectedProvider();
});
</script>

<template>
  <router-view />
</template>
