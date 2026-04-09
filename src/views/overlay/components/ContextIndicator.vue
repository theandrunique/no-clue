<script setup lang="ts">
import { computed } from "vue";
import { useChatStore } from "@/stores/chat";

const chatStore = useChatStore();

const hasModel = computed(() => chatStore.modelInfo !== null);
</script>

<template>
  <div v-if="hasModel" class="flex items-center gap-2 text-xs text-white/60">
    <span class="font-medium">{{ chatStore.modelInfo?.modelName }}</span>
    <span class="text-white/40">{{ chatStore.modelInfo?.contextWindow?.toLocaleString() }}</span>
    <span v-if="chatStore.modelInfo?.supportsVision" class="text-green-400">vision</span>
  </div>
</template>