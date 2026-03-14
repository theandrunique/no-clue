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
  <div class="flex flex-col h-full">
    <div class="flex-1 overflow-y-auto px-3 py-2 flex flex-col gap-2">
      <div v-if="conversationStore.transcripts.length === 0" class="flex flex-col items-center justify-center h-full text-white/50 text-center">
        <p>No transcripts yet</p>
        <p class="text-xs mt-1">Enable microphone to start transcription</p>
      </div>
      <div
        v-for="transcript in conversationStore.transcripts"
        :key="transcript.id"
        class="px-3 py-2 bg-neutral-700/60 rounded-md border-l-2 text-sm"
        :class="transcript.isFinal ? 'border-blue-500/50' : 'border-yellow-500/50 opacity-70 italic'"
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
    </div>
  </div>
</template>
