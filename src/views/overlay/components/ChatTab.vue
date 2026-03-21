<script setup lang="ts">
import { computed, ref } from "vue";
import { useConversationStore } from "@/stores/conversation";
import { Send, Square } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import Input from "@/components/ui/input/Input.vue";
import ChatMessage from "@/components/ChatMessage.vue";
import QuickActions from "./QuickActions.vue";
import type { Message } from "@/types";

const conversationStore = useConversationStore();

const streamingMessage = computed<Message>(() => ({
  id: "streaming",
  conversationId: "",
  role: "assistant",
  content: conversationStore.streamingMessage?.content ?? "",
  timestamp: 0,
}));

const inputMessage = ref("");

function sendMessage(content: string) {
  console.log("[ChatTab] sendMessage called with:", content);
  console.log("[ChatTab] isStreamingResponse:", conversationStore.isStreamingResponse);
  if (!content.trim() || conversationStore.isStreamingResponse) {
    console.log("[ChatTab] Early return - empty or streaming");
    return;
  }

  inputMessage.value = "";
  console.log("[ChatTab] Calling conversationStore.sendMessage");
  conversationStore.sendMessage(content);
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
      <div v-if="conversationStore.messages.length === 0 && !conversationStore.isStreamingResponse" class="flex flex-col items-center justify-center min-h-[100px] text-white/40 text-center">
        <p class="text-sm">No messages yet</p>
        <p class="text-xs mt-1 opacity-60">Start a conversation or use Quick Actions below</p>
      </div>
      <ChatMessage
        v-for="message in conversationStore.messages"
        :key="message.id"
        :message="message"
      />
      <ChatMessage
        v-if="conversationStore.isStreamingResponse"
        :message="streamingMessage"
        :is-streaming="true"
      />
    </div>

    <!-- Quick Actions -->
    <div class="shrink-0 px-3 pb-2">
      <QuickActions @select="handleQuickAction" />
    </div>

    <!-- Input -->
    <div class="shrink-0 flex gap-2 px-3 py-2 border-t border-white/10 items-end">
      <Input
        v-model="inputMessage"
        placeholder="Type a message..."
        class="bg-transparent"
        :disabled="conversationStore.isStreamingResponse"
        @keydown="handleKeydown"
      />
      <Button v-if="conversationStore.isStreamingResponse" variant="ghost" size="icon" @click="stopStreaming">
        <Square class="w-4 h-4" />
      </Button>
      <Button v-else variant="ghost" size="icon" @click="handleSend" :disabled="!inputMessage.trim()">
        <Send class="w-4 h-4" />
      </Button>
    </div>
  </div>
</template>
