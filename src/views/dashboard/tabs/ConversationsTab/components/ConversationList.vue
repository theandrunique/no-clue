<script setup lang="ts">
import { format } from "date-fns";
import { Trash2 } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import type { Conversation } from "@/types";

interface Props {
  conversations: Conversation[];
  selectedId: string | null;
}

defineProps<Props>();

const emit = defineEmits<{
  select: [conversation: Conversation];
  delete: [id: string];
}>();

function formatDate(timestamp: number) {
  return format(new Date(timestamp * 1000), "MMM d, HH:mm");
}
</script>

<template>
  <div class="w-64 shrink-0 border-r border-border flex flex-col overflow-hidden bg-card">
    <div class="flex-1 overflow-y-auto">
      <div
        v-for="conversation in conversations"
        :key="conversation.id"
        @click="emit('select', conversation)"
        class="group relative p-3 border-b border-border cursor-pointer hover:bg-accent transition-colors"
        :class="{ 'bg-accent': selectedId === conversation.id }"
      >
        <div class="text-sm text-foreground truncate pr-8">{{ conversation.title }}</div>
        <div class="text-xs text-muted-foreground mt-1">
          {{ formatDate(conversation.updatedAt) }}
        </div>
        <Button
            class="absolute right-2 top-1/2 -translate-y-1/2 opacity-0 group-hover:opacity-100"
            @click.stop="emit('delete', conversation.id)"
            variant="ghost"
            size="icon"
        >
            <Trash2 class="w-4 h-4 text-destructive-foreground" />
        </Button>
      </div>
      <div v-if="conversations.length === 0" class="p-3 text-sm text-muted-foreground">
        No conversations yet
      </div>
    </div>
  </div>
</template>
