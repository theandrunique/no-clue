<script setup lang="ts">
import { useConversationStore } from "@/stores/conversation";
import TranscriptionCard from "@/components/TranscriptionCard.vue";

const conversationStore = useConversationStore();
</script>

<template>
  <div class="flex flex-col h-full min-h-0">
    <div class="flex-1 min-h-0 overflow-y-auto px-3 py-2 flex flex-col gap-2">
      <div v-if="conversationStore.transcripts.length === 0 && !conversationStore.currentTranscript" class="flex flex-col items-center justify-center h-full text-white/50 text-center">
        <p>No transcripts yet</p>
        <p class="text-xs mt-1">Enable microphone to start transcription</p>
      </div>
      
      <TranscriptionCard
        v-for="transcript in conversationStore.transcripts"
        :key="transcript.id"
        :transcript="transcript"
      />
      
      <TranscriptionCard
        v-if="conversationStore.currentTranscript"
        :transcript="conversationStore.currentTranscript"
      />
    </div>
  </div>
</template>