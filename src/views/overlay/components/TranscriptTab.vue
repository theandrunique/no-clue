<script setup lang="ts">
import { useConversationStore } from "@/stores/conversation";
import { User, Monitor } from "lucide-vue-next";

const conversationStore = useConversationStore();

function getSpeakerIcon(speaker: "user" | "system") {
  return speaker === "user" ? User : Monitor;
}

function getSpeakerLabel(speaker: "user" | "system") {
  return speaker === "user" ? "You" : "System";
}
</script>

<template>
  <div class="flex flex-col h-full min-h-0">
    <div class="flex-1 min-h-0 overflow-y-auto px-3 py-2 flex flex-col gap-2">
      <div v-if="conversationStore.transcripts.length === 0 && !conversationStore.currentTranscript" class="flex flex-col items-center justify-center h-full text-white/50 text-center">
        <p>No transcripts yet</p>
        <p class="text-xs mt-1">Enable microphone to start transcription</p>
      </div>
      
      <!-- Final transcripts -->
      <div
        v-for="transcript in conversationStore.transcripts"
        :key="transcript.id"
        class="px-3 py-2 bg-neutral-700/60 rounded-md border-l-2 text-sm border-blue-500/50"
      >
        <div class="flex items-center gap-1.5 text-xs text-white/50 mb-1">
          <component :is="getSpeakerIcon(transcript.speaker)" :size="12" />
          <span>{{ getSpeakerLabel(transcript.speaker) }}</span>
        </div>
        <div class="break-words">{{ transcript.text }}</div>
        <div v-if="transcript.confidence" class="text-xs text-white/30 mt-1 text-right">
          {{ Math.round(transcript.confidence * 100) }}%
        </div>
      </div>
      
      <!-- Current (interim) transcript -->
      <div
        v-if="conversationStore.currentTranscript"
        class="px-3 py-2 bg-neutral-700/60 rounded-md border-l-2 text-sm border-yellow-500/50 opacity-70 italic"
      >
        <div class="flex items-center gap-1.5 text-xs text-white/50 mb-1">
          <component :is="getSpeakerIcon(conversationStore.currentTranscript.speaker)" :size="12" />
          <span>{{ getSpeakerLabel(conversationStore.currentTranscript.speaker) }}</span>
          <span class="text-yellow-500">(interim)</span>
        </div>
        <div class="break-words">{{ conversationStore.currentTranscript.text }}</div>
        <div v-if="conversationStore.currentTranscript.confidence" class="text-xs text-white/30 mt-1 text-right">
          {{ Math.round(conversationStore.currentTranscript.confidence * 100) }}%
        </div>
      </div>
    </div>
  </div>
</template>
