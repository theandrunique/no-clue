<script setup lang="ts">
import { formatDistanceToNow } from "date-fns";
import { Check, MoreHorizontal, Pencil, Trash2 } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader } from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { type SystemPrompt } from "@/types";

interface Props {
  prompts: SystemPrompt[];
  selectedId: string | null;
}

defineProps<Props>();

const emit = defineEmits<{
  select: [prompt: SystemPrompt];
  edit: [prompt: SystemPrompt];
  requestDelete: [id: string];
}>();

function formatRelativeTime(timestamp: number) {
  const date = new Date(timestamp * 1000);
  if (isNaN(date.getTime())) return "Unknown";
  return formatDistanceToNow(date, { addSuffix: true });
}

function truncateText(text: string, maxLen: number) {
  if (text.length <= maxLen) return text;
  return text.slice(0, maxLen).trim() + "...";
}
</script>

<template>
  <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-4 p-4 overflow-y-auto">
    <Card
      v-for="prompt in prompts"
      :key="prompt.id"
      class="group relative cursor-pointer transition-all hover:shadow-md dark:bg-card dark:border-border"
      :class="{
        'ring-1 ring-primary dark:ring-primary': selectedId === prompt.id,
        'bg-accent dark:bg-accent': selectedId !== prompt.id,
      }"
      @dblclick="emit('select', prompt)"
    >
      <CardHeader class="p-4 pb-2">
        <div class="flex items-start justify-between gap-2">
          <div class="flex-1 min-w-0">
            <span class="font-semibold text-sm truncate">{{ prompt.name }}</span>
          </div>
        </div>
      </CardHeader>
      <CardContent class="p-4 pt-0">
        <p class="text-xs text-muted-foreground line-clamp-3">
          {{ truncateText(prompt.prompt, 150) }}
        </p>
        <p class="text-xs text-muted-foreground mt-3">
          Updated {{ formatRelativeTime(prompt.updatedAt) }}
        </p>
      </CardContent>
      <div
        v-if="selectedId === prompt.id"
        class="absolute top-3 right-3 bg-green-500/10 text-green-600 dark:bg-green-500/20 dark:text-green-400 rounded-full p-1"
      >
        <Check class="w-3 h-3" />
      </div>
      <div class="absolute bottom-3 right-3">
        <DropdownMenu>
          <DropdownMenuTrigger as-child>
            <Button
              size="icon"
              variant="ghost"
              class="shrink-0 opacity-0 group-hover:opacity-100 transition-opacity"
              @click.stop
            >
              <MoreHorizontal class="w-4 h-4" />
            </Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent align="end">
            <DropdownMenuItem @click.stop="emit('edit', prompt)">
              <Pencil class="w-4 h-4 mr-2" />
              Edit
            </DropdownMenuItem>
            <DropdownMenuItem
              class="text-destructive dark:text-destructive"
              @click.stop="emit('requestDelete', prompt.id)"
            >
              <Trash2 class="w-4 h-4 mr-2" />
              Delete
            </DropdownMenuItem>
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
    </Card>
    <div
      v-if="prompts.length === 0"
      class="col-span-full flex items-center justify-center text-muted-foreground text-sm py-8"
    >
      No system prompts yet
    </div>
  </div>
</template>
