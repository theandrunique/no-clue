<script setup lang="ts">
import { ref, onMounted } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { format } from "date-fns";
import { Trash2, User, Monitor } from "lucide-vue-next";
import ChatMessage from "@/components/ChatMessage.vue";

interface Conversation {
  id: string;
  title: string;
  createdAt: number;
  updatedAt: number;
}

interface Message {
  id: string;
  conversationId: string;
  role: "user" | "assistant" | "system";
  content: string;
  screenshotPath?: string;
  timestamp: number;
}

interface Transcript {
  id: string;
  conversationId: string;
  speaker: "user" | "system";
  text: string;
  isFinal: boolean;
  confidence: number;
  timestamp: number;
}

const conversations = ref<Conversation[]>([]);
const selectedConversation = ref<Conversation | null>(null);
const messages = ref<Message[]>([]);
const transcripts = ref<Transcript[]>([]);
const activeTab = ref<"messages" | "transcripts">("messages");
const loadingMessages = ref(false);
const loadingTranscripts = ref(false);

async function loadConversations() {
  try {
    conversations.value = await invoke<Conversation[]>("get_conversations");
  } catch (e) {
    console.error("Failed to load conversations:", e);
  }
}

async function selectConversation(conversation: Conversation) {
  selectedConversation.value = conversation;
  activeTab.value = "messages";
  messages.value = [];
  transcripts.value = [];
  
  loadingMessages.value = true;
  try {
    messages.value = await invoke<Message[]>("get_messages", { conversationId: conversation.id });
    console.log("[ConversationsTab] Loaded messages:", messages.value);
  } catch (e) {
    console.error("Failed to load messages:", e);
  } finally {
    loadingMessages.value = false;
  }
  
  loadingTranscripts.value = true;
  try {
    transcripts.value = await invoke<Transcript[]>("get_transcripts", { conversationId: conversation.id });
    console.log("[ConversationsTab] Loaded transcripts:", transcripts.value);
  } catch (e) {
    console.error("Failed to load transcripts:", e);
  } finally {
    loadingTranscripts.value = false;
  }
}

function formatDate(timestamp: number) {
  return format(new Date(timestamp * 1000), "MMM d, HH:mm");
}

function getSpeakerIcon(speaker: "user" | "system") {
  return speaker === "user" ? User : Monitor;
}

function getSpeakerLabel(speaker: "user" | "system") {
  return speaker === "user" ? "You" : "System";
}

onMounted(() => {
  loadConversations();
});
</script>

<template>
  <div class="flex h-full overflow-hidden">
    <!-- Conversations List -->
    <div class="w-64 shrink-0 border-r border-neutral-800 flex flex-col overflow-hidden">
      <div class="p-3 border-b border-neutral-800 shrink-0">
        <h2 class="text-sm font-medium text-neutral-300">Conversations</h2>
      </div>
      <div class="flex-1 overflow-y-auto">
        <div
          v-for="conversation in conversations"
          :key="conversation.id"
          @click="selectConversation(conversation)"
          class="group relative p-3 border-b border-neutral-800 cursor-pointer hover:bg-neutral-800/50 transition-colors"
          :class="{ 'bg-neutral-800': selectedConversation?.id === conversation.id }"
        >
          <div class="text-sm text-white truncate pr-8">{{ conversation.title }}</div>
          <div class="text-xs text-neutral-500 mt-1">
            {{ formatDate(conversation.updatedAt) }}
          </div>
          <button
            class="absolute right-2 top-1/2 -translate-y-1/2 p-1.5 rounded-md opacity-0 group-hover:opacity-100 hover:bg-neutral-700 transition-all"
            title="Delete conversation"
          >
            <Trash2 class="w-4 h-4 text-neutral-400" />
          </button>
        </div>
        <div
          v-if="conversations.length === 0"
          class="p-3 text-sm text-neutral-500"
        >
          No conversations yet
        </div>
      </div>
    </div>

    <!-- Conversation Details -->
    <div class="flex-1 flex flex-col min-w-0 overflow-hidden" v-if="selectedConversation">
      <div class="p-3 border-b border-neutral-800 shrink-0">
        <h2 class="text-sm font-medium text-white">{{ selectedConversation.title }}</h2>
        <p class="text-xs text-neutral-500">
          {{ formatDate(selectedConversation.createdAt) }}
        </p>
      </div>

      <div class="flex-1 min-h-0 flex flex-col overflow-hidden">
        <div class="flex border-b border-neutral-800 shrink-0">
          <button
            class="px-4 py-2 text-sm transition-colors"
            :class="activeTab === 'messages' ? 'text-white border-b-2 border-white' : 'text-neutral-400 hover:text-white'"
            @click="activeTab = 'messages'"
          >
            Messages ({{ messages.length }})
          </button>
          <button
            class="px-4 py-2 text-sm transition-colors"
            :class="activeTab === 'transcripts' ? 'text-white border-b-2 border-white' : 'text-neutral-400 hover:text-white'"
            @click="activeTab = 'transcripts'"
          >
            Transcripts ({{ transcripts.length }})
          </button>
        </div>

        <div class="flex-1 min-h-0 overflow-y-auto p-4">
          <!-- Messages -->
          <div v-if="activeTab === 'messages'" class="flex flex-col gap-2">
            <div v-if="loadingMessages" class="text-center text-neutral-500 py-8">
              Loading messages...
            </div>
            <template v-else>
              <ChatMessage
                v-for="message in messages"
                :key="message.id"
                :message="message"
              />
              <div v-if="messages.length === 0" class="text-center text-neutral-500 py-8">
                No messages in this conversation
              </div>
            </template>
          </div>

          <!-- Transcripts -->
          <div v-if="activeTab === 'transcripts'" class="flex flex-col gap-2">
            <div v-if="loadingTranscripts" class="text-center text-neutral-500 py-8">
              Loading transcripts...
            </div>
            <template v-else>
              <div
                v-for="transcript in transcripts"
                :key="transcript.id"
                class="px-3 py-2 bg-neutral-800/60 rounded-md border-l-2 text-sm"
                :class="transcript.isFinal ? 'border-blue-500/50' : 'border-yellow-500/50 opacity-70'"
              >
                <div class="flex items-center gap-1.5 text-xs text-neutral-400 mb-1">
                  <component :is="getSpeakerIcon(transcript.speaker)" :size="12" />
                  <span>{{ getSpeakerLabel(transcript.speaker) }}</span>
                  <span v-if="!transcript.isFinal" class="text-yellow-500">(interim)</span>
                </div>
                <div class="break-words">{{ transcript.text }}</div>
                <div class="text-xs text-neutral-500 mt-1 text-right">
                  {{ Math.round(transcript.confidence * 100) }}%
                </div>
              </div>
              <div v-if="transcripts.length === 0" class="text-center text-neutral-500 py-8">
                No transcripts for this conversation
              </div>
            </template>
          </div>
        </div>
      </div>
    </div>

    <div v-else class="flex-1 flex items-center justify-center text-neutral-500">
      Select a conversation to view details
    </div>
  </div>
</template>
