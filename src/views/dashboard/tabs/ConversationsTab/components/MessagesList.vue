<script setup lang="ts">
import type { Message } from "@/types";
import ChatMessage from "@/components/ChatMessage.vue";

interface Props {
  messages: Message[];
  loading: boolean;
}

defineProps<Props>();
</script>

<template>
  <div class="flex flex-col gap-2">
    <div v-if="loading" class="text-center text-muted-foreground py-8">
      Loading messages...
    </div>
    <template v-else>
      <ChatMessage
        v-for="message in messages"
        :key="message.id"
        :message="message"
      />
      <div v-if="messages.length === 0" class="text-center text-muted-foreground py-8">
        No messages in this conversation
      </div>
    </template>
  </div>
</template>