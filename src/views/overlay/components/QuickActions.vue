<script setup lang="ts">
import { Zap } from "lucide-vue-next";
import { Button } from "@/components/ui/button";

interface QuickAction {
  label: string;
  prompt: string;
  icon?: object;
}

const emit = defineEmits<{
  (e: "select", prompt: string): void;
}>();

function handleSelect(action: QuickAction) {
  emit("select", action.prompt);
}

const quickActions = [
  { label: "What's on screen?", prompt: "What's currently displayed on my screen?" },
  { label: "Summarize audio", prompt: "Summarize what's being said in the audio." },
  { label: "What did I say?", prompt: "What was the last thing I said?" },
  { label: "Explain this", prompt: "Can you explain what you just said?" },
];

</script>

<template>
  <div class="grid grid-cols-2 gap-1.5">
    <Button
      v-for="action in quickActions"
      :key="action.label"
      variant="outline"
      size="sm"
      class="h-8 py-1 px-2 whitespace-normal text-center text-xs gap-1"
      @click="handleSelect(action)"
    >
      <Zap :size="12" />
      <span class="truncate">{{ action.label }}</span>
    </Button>
  </div>
</template>
