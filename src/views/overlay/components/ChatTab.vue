<script setup lang="ts">
import { ref } from "vue";
import { useConversationStore } from "@/stores/conversation";
import { Send, Square } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import Input from "@/components/ui/input/Input.vue";
import ChatMessage from "./ChatMessage.vue";
import StreamingMessage from "./StreamingMessage.vue";
import QuickActions from "./QuickActions.vue";

const conversationStore = useConversationStore();

const inputMessage = ref("");

function sendMessage(content: string) {
  if (!content.trim() || conversationStore.isStreaming) return;

  const message = content;
  inputMessage.value = "";

  conversationStore.sendMessage(message);
}

function stopStreaming() {
  conversationStore.stopStream();
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
  <div class="flex flex-col h-full min-h-0">
    <!-- Messages Area -->
    <div class="flex-1 min-h-0 overflow-y-auto px-3 py-2 flex flex-col gap-2">
      <div v-if="conversationStore.messages.length === 0 && !conversationStore.isStreaming" class="flex flex-col items-center justify-center min-h-[100px] text-white/40 text-center">
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
    <div class="flex-shrink-0 px-3 pb-2">
      <QuickActions @select="handleQuickAction" />
    </div>

    <!-- Input -->
    <div class="flex-shrink-0 flex gap-2 px-3 py-2 border-t border-white/10 items-end">
      <Input
        v-model="inputMessage"
        placeholder="Type a message..."
        class="bg-transparent"
        :disabled="conversationStore.isStreaming"
        @keydown="handleKeydown"
      />
      <Button v-if="conversationStore.isStreaming" variant="ghost" size="icon" @click="stopStreaming">
        <Square class="w-4 h-4" />
      </Button>
      <Button v-else variant="ghost" size="icon" @click="handleSend" :disabled="!inputMessage.trim()">
        <Send class="w-4 h-4" />
      </Button>
    </div>
  </div>
</template>
