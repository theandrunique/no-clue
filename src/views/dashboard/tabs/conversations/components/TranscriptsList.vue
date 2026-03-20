<script setup lang="ts">
import type { Transcript } from "@/types";
import TranscriptionCard from "@/components/TranscriptionCard.vue";

interface Props {
  transcripts: Transcript[];
  loading: boolean;
}

defineProps<Props>();
</script>

<template>
  <div class="flex flex-col gap-2">
    <div v-if="loading" class="text-center text-muted-foreground py-8">
      Loading transcripts...
    </div>
    <template v-else>
      <TranscriptionCard
        v-for="transcript in transcripts"
        :key="transcript.id"
        :transcript="transcript"
      />
      <div v-if="transcripts.length === 0" class="text-center text-muted-foreground py-8">
        No transcripts for this conversation
      </div>
    </template>
  </div>
</template>