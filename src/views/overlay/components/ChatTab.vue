<script setup lang="ts">
import { ref } from "vue";
import { useConversationStore } from "@/stores/conversation";
import { useOverlayStore } from "@/stores/overlay";
import { Send } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import Input from "@/components/ui/input/Input.vue";
import ChatMessage from "./ChatMessage.vue";
import StreamingMessage from "./StreamingMessage.vue";
import QuickActions from "./QuickActions.vue";

const conversationStore = useConversationStore();
const overlayStore = useOverlayStore();

const inputMessage = ref("");

function sendMessage(content: string) {
  if (!content.trim()) return;

  const message = content;
  inputMessage.value = "";

  if (overlayStore.currentConversationId) {
    conversationStore.addUserMessage(overlayStore.currentConversationId, message);
    conversationStore.setStreaming(true);
  }
}

function handleSend() {
  sendMessage(inputMessage.value);
}

function handleQuickAction(prompt: string) {
  sendMessage(prompt);
}

function handleKeydown(e: KeyboardEvent) {
  if (e.key === "Enter" && !e.shiftKey) {
    e.preventDefault();
    handleSend();
  }
}
</script>

<template>
  <div class="flex flex-col h-full">
    <!-- Messages Area -->
    <div class="flex-1 overflow-y-auto px-3 py-2 flex flex-col gap-2">
      <div v-if="conversationStore.messages.length === 0" class="flex flex-col items-center justify-center h-full text-white/40 text-center">
        <p class="text-sm">No messages yet</p>
        <p class="text-xs mt-1 opacity-60">Start a conversation or use Quick Actions below</p>
      </div>
      <ChatMessage
        v-for="message in conversationStore.messages"
        :key="message.id"
        :message="message"
      />
      <StreamingMessage
        v-if="conversationStore.isStreaming"
        :content="conversationStore.currentStreamingContent"
      />
    </div>

    <!-- Quick Actions -->
    <div class="px-3 pb-2">
      <QuickActions @select="handleQuickAction" />
    </div>

    <!-- Input -->
    <div class="flex gap-2 px-3 py-2 border-t border-white/10 items-end">
      <Input
        v-model="inputMessage"
        placeholder="Type a message..."
        class="bg-transparent"
        @keydown="handleKeydown"
      />
      <Button variant="ghost" size="icon" @click="handleSend" :disabled="!inputMessage.trim()">
        <Send class="w-4 h-4" />
      </Button>
    </div>
  </div>
</template>
