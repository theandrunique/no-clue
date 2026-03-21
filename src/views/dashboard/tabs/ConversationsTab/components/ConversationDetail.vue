<script setup lang="ts">
import { format } from "date-fns";
import { MessageSquare, FileText } from "lucide-vue-next";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import type { Conversation, Message, Transcript } from "@/types";
import MessagesList from "./MessagesList.vue";
import TranscriptsList from "./TranscriptsList.vue";

interface Props {
  conversation: Conversation;
  messages: Message[];
  transcripts: Transcript[];
  loadingMessages: boolean;
  loadingTranscripts: boolean;
}

defineProps<Props>();

defineModel<"messages" | "transcripts">("activeTab", { default: "messages" });

function formatDate(timestamp: number) {
  return format(new Date(timestamp * 1000), "MMM d, HH:mm");
}
</script>

<template>
  <div class="flex-1 flex flex-col min-w-0 overflow-hidden">
    <div class="p-3 border-b border-border shrink-0 bg-card">
      <h2 class="text-sm font-medium text-foreground">{{ conversation.title }}</h2>
      <p class="text-xs text-muted-foreground">
        {{ formatDate(conversation.createdAt) }}
      </p>
    </div>

    <Tabs defaultValue="messages" class="flex-1 flex flex-col min-h-0 overflow-hidden">
      <TabsList class="shrink-0 w-full justify-start rounded-none border-b bg-transparent px-2 h-9">
        <TabsTrigger value="messages" class="gap-2">
          <MessageSquare class="w-4 h-4" />
          Messages ({{ messages.length }})
        </TabsTrigger>
        <TabsTrigger value="transcripts" class="gap-2">
          <FileText class="w-4 h-4" />
          Transcripts ({{ transcripts.length }})
        </TabsTrigger>
      </TabsList>

      <TabsContent value="messages" class="flex-1 min-h-0 m-0 overflow-hidden">
        <div class="h-full overflow-y-auto p-4">
          <MessagesList
            :messages="messages"
            :loading="loadingMessages"
          />
        </div>
      </TabsContent>

      <TabsContent value="transcripts" class="flex-1 min-h-0 m-0 overflow-hidden">
        <div class="h-full overflow-y-auto p-4">
          <TranscriptsList
            :transcripts="transcripts"
            :loading="loadingTranscripts"
          />
        </div>
      </TabsContent>
    </Tabs>
  </div>
</template>