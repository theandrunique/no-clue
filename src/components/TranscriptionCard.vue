<script setup lang="ts">
import type { Transcript, TranscriptionResult } from "@/types";
import { User, Monitor } from "lucide-vue-next";

interface Props {
  transcript: Transcript | TranscriptionResult;
}

const props = defineProps<Props>();

function getSpeakerIcon(speaker: "user" | "system") {
  return speaker === "user" ? User : Monitor;
}

function getSpeakerLabel(speaker: "user" | "system") {
  return speaker === "user" ? "You" : "System";
}

function getIsFinal(): boolean {
  if (props.transcript && 'isFinal' in props.transcript) {
    return props.transcript.isFinal;
  }
  return true;
}

const isFinal = getIsFinal();
</script>

<template>
  <div
    class="px-3 py-2 bg-neutral-700/60 rounded-md border-l-2 text-sm"
    :class="[
      isFinal
        ? 'border-blue-500/50'
        : 'border-yellow-500/50 opacity-70 italic'
    ]"
  >
    <div class="flex items-center gap-1.5 text-xs text-white/50 mb-1">
      <component :is="getSpeakerIcon(transcript.speaker)" :size="12" />
      <span>{{ getSpeakerLabel(transcript.speaker) }}</span>
      <span v-if="!isFinal" class="text-yellow-500">(interim)</span>
    </div>
    <div class="break-words">{{ transcript.text }}</div>
    <div v-if="transcript.confidence" class="text-xs text-white/30 mt-1 text-right">
      {{ Math.round(transcript.confidence * 100) }}%
    </div>
  </div>
</template>