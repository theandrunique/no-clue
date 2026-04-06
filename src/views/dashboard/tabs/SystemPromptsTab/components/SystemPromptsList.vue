<script setup lang="ts">
import { format } from "date-fns";
import { Pencil, Trash2 } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { type SystemPrompt } from "@/types";

interface Props {
  prompts: SystemPrompt[];
  selectedId: string | null;
}

defineProps<Props>();

const emit = defineEmits<{
  select: [prompt: SystemPrompt];
  edit: [prompt: SystemPrompt];
  delete: [id: string];
  setActive: [id: string];
}>();

function formatDate(timestamp: number) {
  const date = new Date(timestamp * 1000);
  if (isNaN(date.getTime())) return "Unknown";
  return format(date, "MMM d, HH:mm");
}

function truncate(text: string, maxLen: number) {
  if (text.length <= maxLen) return text;
  return text.slice(0, maxLen) + "...";
}
</script>

<template>
  <div class="w-72 shrink-0 border-r border-border flex flex-col overflow-hidden bg-card">
    <div class="flex-1 overflow-y-auto">
      <div
        v-for="prompt in prompts"
        :key="prompt.id"
        @click="emit('select', prompt)"
        class="group relative p-3 border-b border-border cursor-pointer hover:bg-accent transition-colors"
        :class="{ 'bg-accent': selectedId === prompt.id }"
      >
        <div class="text-sm text-foreground truncate pr-16">{{ prompt.name }}</div>
        <div class="text-xs text-muted-foreground mt-1">
          {{ formatDate(prompt.updatedAt) }}
        </div>
        <div class="text-xs text-muted-foreground mt-1 truncate pr-16">
          {{ truncate(prompt.prompt, 50) }}
        </div>
        <div class="absolute right-2 top-1/2 -translate-y-1/2 flex gap-1 opacity-0 group-hover:opacity-100">
          <Button
            v-if="selectedId !== prompt.id"
            size="icon"
            variant="ghost"
            @click.stop="emit('setActive', prompt.id)"
            title="Set as active"
          >
            <span class="text-xs font-medium">Set</span>
          </Button>
          <Button
            size="icon"
            variant="ghost"
            @click.stop="emit('edit', prompt)"
          >
            <Pencil class="w-4 h-4" />
          </Button>
          <Button
            size="icon"
            variant="ghost"
            @click.stop="emit('delete', prompt.id)"
          >
            <Trash2 class="w-4 h-4 text-destructive-foreground" />
          </Button>
        </div>
      </div>
      <div v-if="prompts.length === 0" class="p-3 text-sm text-muted-foreground">
        No system prompts yet
      </div>
    </div>
  </div>
</template>
