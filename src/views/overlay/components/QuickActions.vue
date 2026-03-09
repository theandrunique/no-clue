<script setup lang="ts">
import { useOverlayStore } from "@/stores/overlay";
import { useChatStore } from "@/stores/chat";
import { Zap } from "lucide-vue-next";
import { Button } from "@/components/ui/button";

const overlayStore = useOverlayStore();
const chatStore = useChatStore();

const quickActions = [
  { label: "What's on my screen?", prompt: "What's currently displayed on my screen?" },
  { label: "Summarize audio", prompt: "Summarize what's being said in the audio." },
  { label: "What did I say?", prompt: "What was the last thing I said?" },
  { label: "Explain this", prompt: "Can you explain what you just said?" },
];

async function executeAction(prompt: string) {
  if (!overlayStore.currentConversationId) {
    return;
  }

  chatStore.addUserMessage(overlayStore.currentConversationId, prompt);
  chatStore.setStreaming(true);
}
</script>

<template>
  <div class="p-3 flex flex-col h-full">
    <div class="grid grid-cols-2 gap-2">
      <Button
        v-for="action in quickActions"
        :key="action.label"
        variant="outline"
        size="sm"
        class="h-auto py-3 px-2 whitespace-normal text-center text-xs gap-1.5"
        @click="executeAction(action.prompt)"
      >
        <Zap :size="14" />
        <span>{{ action.label }}</span>
      </Button>
    </div>
    <p class="mt-auto text-center text-[11px] text-white/30 pt-3">Click to send to AI</p>
  </div>
</template>
