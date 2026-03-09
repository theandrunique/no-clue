<script setup lang="ts">
import { ref } from "vue";
import { useChatStore } from "@/stores/chat";
import { useOverlayStore } from "@/stores/overlay";
import { Send } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import Input from "@/components/ui/input/Input.vue";

const chatStore = useChatStore();
const overlayStore = useOverlayStore();

const inputMessage = ref("");

async function sendMessage() {
  if (!inputMessage.value.trim() || !overlayStore.currentConversationId) return;

  const userMessage = inputMessage.value;
  inputMessage.value = "";

  chatStore.addUserMessage(overlayStore.currentConversationId, userMessage);

  chatStore.setStreaming(true);

  try {
    // TODO: Implement actual chat completion via Rust
  } catch (error) {
    console.error("Failed to send message:", error);
  }
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    sendMessage();
  }
}
</script>

<template>
  <div class="flex flex-col">
    <div class="flex-1 overflow-y-auto px-3 py-2 flex flex-col gap-2">
      <div v-if="chatStore.messages.length === 0" class="flex flex-col items-center justify-center h-full text-white/50 text-center">
        <p>No messages yet</p>
        <p class="text-xs mt-1">Start a conversation or use Quick Actions</p>
      </div>
      <div
        v-for="message in chatStore.messages"
        :key="message.id"
        class="max-w-[85%] px-3 py-2 rounded-lg text-sm break-words"
        :class="message.role === 'user' ? 'self-end bg-blue-600/80' : 'self-start bg-neutral-700/80'"
      >
        {{ message.content }}
      </div>
      <div v-if="chatStore.isStreaming" class="self-start bg-neutral-700/80 px-3 py-2 rounded-lg text-sm">
        {{ chatStore.currentStreamingContent }}<span class="animate-pulse">▊</span>
      </div>
    </div>
    <div class="flex gap-2 px-3 py-2 border-t border-white/10 items-end">
      <Input
        placeholder="Type a message..."
        @keydown="handleKeydown"
      />
      <Button variant="ghost" size="icon" @click="sendMessage" :disabled="!inputMessage.trim()">
        <Send class="w-4 h-4" />
      </Button>
    </div>
  </div>
</template>
